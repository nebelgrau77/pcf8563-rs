/* 
set the time, then read it every second and print over USART
*/

#![no_main]
#![no_std]

use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32l4xx_hal::{
    delay::Delay,
    prelude::*,
    serial::{Config, Serial},
    i2c::I2c,
    };

use pcf8563::*;

use core::fmt::Write;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);

    let mut led = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    // set up USART 
    let tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
    let rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);

    let serial = Serial::usart2(
        dp.USART2,
        (tx,rx),
        Config::default().baudrate(9600.bps()),
        clocks,
        &mut rcc.apb1r1,
    );

    let (mut tx, mut _rx) = serial.split();

    // get the delay provider
    let mut delay = Delay::new(cp.SYST, clocks);
    
    // set up I2C
    let mut scl = gpioa.pa9.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    scl.internal_pull_up(&mut gpioa.pupdr, true);
    let scl = scl.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut sda = gpioa.pa10.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    sda.internal_pull_up(&mut gpioa.pupdr, true);
    let sda = sda.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks, &mut rcc.apb1r1);

    // set up RTC 
    let mut rtc = PCF8563::new(i2c);

    rtc.rtc_init().unwrap();

    
    let now = DateTime {
        year: 21, // 2021
        month: 6, // October
        weekday: 2, // Sunday
        day: 4, 
        hours: 12,
        minutes: 22,
        seconds: 00,
    };
    
    // set date and time in one go
    rtc.set_datetime(&now).unwrap();

    loop {                
        led.set_high().ok();        
        delay.delay_ms(500 as u32);

        
        let now = rtc.get_datetime().unwrap();
 
        // format the date and time in a long format
        // eg. "Today is Sunday, 4 April 2021 16:43:00"
        writeln!(tx, "Today is {}, {} {} 20{:02} {:02}:{:02}:{:02}\r", 
                weekday_name(now),
                now.day, 
                month_name(now),
                now.year, 
                now.hours, 
                now.minutes, 
                now.seconds,
                ).unwrap();
        
                 
        led.set_low().ok();
        delay.delay_ms(500 as u32);
        
    }
}

// Helper function to get the correct name of the day
fn weekday_name(datetime: DateTime) -> &'static str  {
    let mut name = "Sunday";
    let day = datetime.weekday;
    match day {
        0 => {name = "Sunday"}
        1 => {name = "Monday"}
        2 => {name = "Tuesday"}
        3 => {name = "Wednesday"}
        4 => {name = "Thursday"}
        5 => {name = "Friday"}
        6 => {name = "Saturday"}
        _ => ()
    }
    name
}

// Helper function to get the correct name of the month
fn month_name(datetime: DateTime) -> &'static str  {
    let mut name = "January";
    let month = datetime.month;
    match month {
        1 => {name = "January"}
        2 => {name = "February"}
        3 => {name = "March"}
        4 => {name = "April"}
        5 => {name = "May"}
        6 => {name = "June"}
        7 => {name = "July"}
        8 => {name = "August"}
        9 => {name = "September"}
        10 => {name = "October"}
        11 => {name = "November"}
        12 => {name = "December"}        
        _ => ()
    }
    name
}