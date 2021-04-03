// example for STM32L4 MCU
// sets the time, then prints it over serial every second

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

    // set up GPIO ports
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);

    // set up LED for some blinking
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

    let (mut tx, mut rx) = serial.split();

    // get the delay provider
    let mut delay = Delay::new(cp.SYST, clocks);
    
    // set up I2C bus
    let mut scl = gpioa.pa9.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    scl.internal_pull_up(&mut gpioa.pupdr, true);
    let scl = scl.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut sda = gpioa.pa10.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    sda.internal_pull_up(&mut gpioa.pupdr, true);
    let sda = sda.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks, &mut rcc.apb1r1);
    
    // set up the PCF8563 device
    let mut pcf = PCF8563::new(i2c); 

    // prepare date and time to be set
    let now = DateTime {
        year: 21,
        month: 3,        
        weekday: 4,
        day: 31,
        hours: 21,
        minutes: 58,
        seconds: 00,
    };

    // set date and time in one go
    pcf.set_datetime(&now).unwrap();

    loop {
        led.set_high().ok();
        delay.delay_ms(500 as u32);
   
        //get date and time in one go
        let time = pcf.get_datetime().unwrap();

        writeln!(tx, "{:02}/{:02}/{:02} {:02}:{:02}:{:02} day {}\r", 
                time.year, time.month, time.day, 
                time.hours, time.minutes, time.seconds,
                time.weekday).unwrap();

        led.set_low().ok();
        delay.delay_ms(500 as u32);

        }
}