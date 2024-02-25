# Rust PCF8563 Real-Time Clock Driver

![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

A platform agnostic Rust driver for the NXP PCF8563 real-time clock,
based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

Based on this [RTC driver](https://github.com/eldruin/ds1307-rs) 

This driver allows you to:
- Read and set date and time. See: `get_datetime` and `set_datetime`
- Set only the time (HH:MM:SS) for clock applications without the calendar function
- Read and set the alarm minutes, hours, day and weekday
- Enable the alarm components separately
- Disable the alarm components separately or all at once
- Set the timer and timer frequency
- Set clock output frequency and enable/disable clock output
- Enable and disable alarm interrupt and timer interrupt
- Read and set various other control functions

### TO DO:
- [x] test with other MCUs
- [ ] add `get_timer_frequency` function
- [ ] add `get_timer_interrupt_mode` function
- [ ] add `get_clkout_frequency` function 
- [ ] add an nRF example
- [ ] add better tests

[How this driver was ~~won~~ written](https://nebelgrau77.github.io/posts/rust_driver/)

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
        month: 4, // April
        weekday: 0, // Sunday
        day: 4, 
        hours: 16,
        minutes: 52,
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

