# Rust PCF8563 Real-Time Clock Driver

![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

A platform agnostic Rust driver for the NXP PCF8563 real-time clock,
based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

Based on this [DS1307 driver](https://github.com/eldruin/ds1307-rs) 

This driver allows you to:
- Read and set date and time. See: `get_datetime`

### Functions implemented so far:

[] init function (reset everything, shut off alarms etc.)  
[] start/stop clock  
[x] get datetime  
[x] set datetime  
[x] set the alarm (minutes, hours, day, weekday)  
[] get the alarm setting (minutes, hours, day, weekday)  
[x] enable/disable alarm components (minutes, hours, day, weekday)
[x] clear alarm flag
[x] enable/disable alarm interrupt
[] start/stop clock output  
[] set clock output frequency  
[] get clock output frequency  
[x] clear timer flag
[x] start/stop timer  
[x] set timer frequency  
[] get timer frequency  
[x] set time for timer to count down from  
[] set the control settings  
[] get the control settings  
[] set time only (for applications like clocks, without data)

### Blog post placeholder:

[blog post](https://nebelgrau77.github.io/posts/helloworld/)

## The device

The PCF8563 is a CMOS Real-Time Clock (RTC) and calendar optimized for low power
consumption. A programmable clock output, interrupt output, and voltage-low detector are also provided. All addresses and data are transferred serially via a two-line bidirectional I2C-bus. Maximum bus speed is 400 kbit/s. The register address is incremented automatically after each written or read data byte.

Provides year, month, day, weekday, hours, minutes, and seconds based on a
32.768 kHz quartz crystal.
* Century flag
* Low backup current
* Programmable clock output for peripheral devices (32.768 kHz, 1.024 kHz, 32 Hz, and
1 Hz)
* Alarm and timer functions
* Open-drain interrupt pin

Datasheet: [PCF8563](https://www.nxp.com/docs/en/data-sheet/PCF8563.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [examples]

[examples]: https://github.com/nebelgrau77/pcf8563-rs/tree/main/examples

### add other examples in the folder (NRF, ideally Linux)

```rust
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

    // set up the flash, reset & clock control, and power control
    // set up the clocks    
    // set up GPIO ports
    // set up LED for some blinking
    // set up USART
    // get the delay provider    
    // set up I2C bus
    
    // set up the PCF8563 device
    let mut rtc = PCF8563::new(i2c); 

    // prepare date and time to be set
    let now = DateTime {
        year: 21, // 2021
        month: 3,        
        weekday: 4, // Wednesday, Sunday is 0
        day: 31,
        hours: 21,
        minutes: 58,
        seconds: 00,
    };

    // set date and time in one go
    rtc.set_datetime(&now).unwrap();

    loop {
        led.set_high().ok();
        delay.delay_ms(500 as u32);
   
        //get date and time in one go
        let time = rtc.get_datetime().unwrap();

        writeln!(tx, "{:02}/{:02}/{:02} {:02}:{:02}:{:02} day {}\r", 
                time.year, time.month, time.day, 
                time.hours, time.minutes, time.seconds,
                time.weekday).unwrap();

        led.set_low().ok();
        delay.delay_ms(500 as u32);

        }
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/nebelgrau77/pcf8563-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

