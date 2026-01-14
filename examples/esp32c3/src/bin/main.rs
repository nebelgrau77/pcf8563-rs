#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::time::Rate;
use esp_hal::i2c::master::{I2c, Config as I2cConfig};
use esp_hal::Blocking;
use embassy_sync::{
    signal::Signal,
    blocking_mutex::raw::{
        CriticalSectionRawMutex,                    
    },    
};
use esp_println as _;

use pcf8563::*;

static TIMESIGNAL: Signal<CriticalSectionRawMutex, DateTime> = Signal::new();

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let i2c_bus = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(100)),
    ).unwrap()
    .with_scl(peripherals.GPIO7)
    .with_sda(peripherals.GPIO6);
    
    info!("i2c set up ");

    let mut rtc = PCF8563::new(i2c_bus);

    info!("RTC set up");

    /*
    // initial setup
    rtc.rtc_init().unwrap();
    rtc.set_datetime(&DateTime {year: 26, month: 1, day: 14, hours: 22, minutes: 5, seconds: 0, weekday: 4}).unwrap();
     */
    
    spawner.spawn(get_time(rtc)).ok();

    loop {
        // get time every second                      
        let time = TIMESIGNAL.wait().await;
        info!("date & time: 20{:02}-{:02}-{:02} {:02}:{:02}:{:02}", time.year, time.month, time.day, time.hours, time.minutes, time.seconds);
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}



#[embassy_executor::task]
async fn get_time(mut rtc: PCF8563<I2c<'static, Blocking>>) {
    // get time from RTC, update TIMESIGNAL
    loop {
        let time = rtc.get_datetime().unwrap();                        
        TIMESIGNAL.signal(time);
        Timer::after(Duration::from_millis(250)).await;
    }
}
