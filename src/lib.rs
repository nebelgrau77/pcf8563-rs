//! A platform agnostic Rust driver for the NXP PCF8563 real-time clock,
//! based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//!
//! This driver allows you to:
//! - read date and time, see [`get_datetime()`]
//! - set date and time, see [`set_datetime()`]
//! - set and enable alarms (minutes, hours, day, weekday)
//! - set and enable timer with variable clock frequency
//! - enable, disable and clear timer and alarm interrupts
//! - enable and disable clock output with variable frequency
//!  
//! [`get_datetime()`]: struct.PCF8563.html#method.get_datetime
//! [`set_datetime()`]: struct.PCF8563.html#method.set_datetime
//!
//!## The device
//! The PCF8563 is a CMOS Real-Time Clock (RTC) and calendar optimized for low power
//! consumption. A programmable clock output, interrupt output, and voltage-low detector are also provided. All addresses and data are transferred serially via a two-line bidirectional I2C-bus. Maximum bus speed is 400 kbit/s. The register address is incremented automatically after each written or read data byte.
//!
//! Provides year, month, day, weekday, hours, minutes, and seconds based on a 32.768 kHz quartz crystal.
//! * Century flag
//! * Low backup current
//! * Programmable clock output for peripheral devices (32.768 kHz, 1.024 kHz, 32 Hz, and 1 Hz)
//! * Alarm and timer functions
//! * Open-drain interrupt pin
//!
//! ### Datasheet: [PCF8563](https://www.nxp.com/docs/en/data-sheet/PCF8563.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! Please find additional examples using hardware in this repository: [examples]
//!
//! [examples]: https://github.com/nebelgrau77/pcf8563-rs/tree/main/examples
//!
//! ### Initialization
//!
//! A new instance of the device is created like this:
//!
//! ```rust
//! use pcf8563::*;
//!
//! let mut rtc = PCF::new(i2c);
//! ```
//!
//! The RTC doesn't need any special setup, you can just start reading from/ writing to it.
//! The wrapper function `rtc_init()` can be used for initialization of the device:
//!
//! ```rust
//! rtc.rtc_init().unwrap();
//! ```
//!
//! It clears all the bits in the two control registers, disabling all the interrupts,
//! alarms, timer and special modes (power-on-reset override, external clock).
//! It also sets the timer to the lowest possible frequency (1/60 Hz) for power saving.
//!
//!
//! ### Date and time
//!
//! All the functions regarding setting and reading date and time are defined in the `datetime.rs` module:
//!
//! - `set_datetime` (sets all the date and time components at once)
//! - `get_datetime` (reads all the date and time components at once)
//! - `set_time` (sets only time components, all at once)
//!  
//! ```rust
//!
//! let mut rtc = PCF8563::new(i2c);
//!
//! let now = DateTime {
//!     year: 21,
//!     month: 4,
//!     weekday: 0, // Sunday
//!     day: 4,
//!     hours: 7,
//!     minutes: 15,
//!     seconds: 00,
//! };
//!
//! rtc.set_datetime(&now).unwrap();
//! ```
//!
//! __TO DO__: add description of the century flag
//!
//! ### Alarm
//!
//! All the alarm-related functions are defined in the `alarm.rs` module:
//!
//! - setting and reading single alarm components (minutes, hours, days, weekdays)
//! - enabling and disabling single alarm components
//! - enabling and disabling alarm interrupt (interrupt pin set to active when the alarm event occurs)
//!
//! ```rust
//! // set the alarm to 9:25, the alarm flag AF will be set at that time,
//! // and the interrupt pin set to active
//! rtc.set_alarm_minutes(25).unwrap();
//! rtc.set_alarm_hours(9).unwrap();
//! rtc.control_alarm_minutes(Control::On).unwrap();
//! rtc.control_alarm_hours(Control::On).unwrap();
//! rtc.control_alarm_interrupt(Control::On).unwrap();
//!```
//!
//! To check the alarm flag and clear after it's set:
//!
//! ```rust
//! if rtc.get_alarm_flag().unwrap() {
//!     rtc.clear_alarm_flag().unwrap()
//! }
//!```
//!
//! Each alarm component has to be enabled separately: minutes, hours, day, weekday,
//! but a wrapper function was defined to disable all the alarms at once:
//!
//! ```rust
//! rtc.disable_all_alarms().unwrap();
//! ```
//!
//! ### Timer
//!
//! All the timer-related functions are defined in the `timer.rs` module
//!
//! The timer can be set with a value up to 255, and one of the four clock frequencies can be chosen:
//! - 4096 Hz
//! - 64 Hz
//! - 1 Hz (every second)
//! - 1/60 Hz (every minute)
//!
//! When the countdown ends, TF bit flag is set. The timer can also set the interrupt pin
//! to active, and the output mode can be chosen between continuous and pulsating (please consult the datasheet for more information).
//!
//! __NOTE__: if both AIE (alarm interrupt) and TIE (timer interrupt) settings are enabled, the status of the interrupt pin will be
//! the result of an OR operation, i.e. will be active when either alarm or timer will trigger the interrupt event.
//!
//! ```rust
//! rtc.set_timer_frequency(TimerFreq::Timer_1Hz).unwrap(); // set frequency to 1 Hz
//! rtc.set_timer(30).unwrap(); // set timer to 30 ticks
//! rtc.control_timer_interrupt(Control::On).unwrap(); // enable timer interrupt
//! rtc.control_timer(Control::On).unwrap(); // start the timer
//!
//! // after 30 seconds the timer will set the TF flag and the interrupt pin will become active
//!
//! rtc.control_timer(Control::Off).unwrap(); // disable the timer
//! rtc.clear_timer_flag().unwrap(); // clear the timer flag
//! ```
//!
//! ### Clock output
//!
//! All the clock output-related functions are defined in the `clkout.rs` module
//!
//! The clock output is a square wave with 50% duty cycle on the dedicated open drain pin (a 10k
//! pull-up resistor between the pin and 3V3 is necessary). It can be used as input to a charge pump,
//! as an external clock for a microcontroller, etc.
//!
//! The clock output can be enabled or disabled, and the frequency can be set to:
//! - 32768Hz (default setting)
//! - 1024 Hz
//! - 32 Hz
//! - 1 Hz
//!
//! On reset the clock output is enabled and set to 32768 Hz
//!
//! ```rust
//! rtc.set_clkout_frequency(ClkoutFreq::Clkout_1024Hz).unwrap(); // set the frequency
//! rtc.control_clkout(Control::On).unwrap(); // enable the clock output
//! ```
//!
//! ### RTC Control
//! All the other control functions are defined in the `control.rs` module
//!
//! - `control_clock()` - starts and stops the internal clock of the RTC
//! - `is_clock_running()` - checks the STOP flag (if cleared, the clock is running)
//! - `get_voltage_low_flag()` - checks whether the VL flag was triggered (see datasheet for details)
//! - `clear_voltage_low_flag()` - clears the voltage low detection flag
//! - `control_ext_clk_test_mode()` - enables the EXT_CLK test mode (see datasheet for details)
//! - `control_power_on_reset_override()` - enables the POR override mode (see datasheet for details)

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

use embedded_hal as hal;

use hal::i2c::{I2c, Operation, SevenBitAddress};

/// All possible errors in this crate
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error<I2cE>
where
    I2cE: embedded_hal::i2c::Error,
 {
    /// I2C bus error
    I2C(I2cE),
    /// Invalid input data
    InvalidInputData,
}

struct Register;

impl Register {
    const CTRL_STATUS_1: u8 = 0x00;
    const CTRL_STATUS_2: u8 = 0x01;
    const VL_SECONDS: u8 = 0x02;
    //const MINUTES           : u8 = 0x03;
    //const HOURS             : u8 = 0x04;
    //const DAYS              : u8 = 0x05;
    //const WEEKDAYS          : u8 = 0x06;
    const CENTURY_MONTHS: u8 = 0x07;
    //const YEARS             : u8 = 0x08;
    const MINUTE_ALARM: u8 = 0x09;
    const HOUR_ALARM: u8 = 0x0A;
    const DAY_ALARM: u8 = 0x0B;
    const WEEKDAY_ALARM: u8 = 0x0C;
    const CLKOUT_CTRL: u8 = 0x0D;
    const TIMER_CTRL: u8 = 0x0E;
    const TIMER: u8 = 0x0F;
}

struct BitFlags;

impl BitFlags {
    const TEST1: u8 = 0b1000_0000;
    const STOP: u8 = 0b0010_0000;
    const TESTC: u8 = 0b0000_1000;
    const TI_TP: u8 = 0b0001_0000;
    const AF: u8 = 0b0000_1000;
    const TF: u8 = 0b0000_0100;
    const AIE: u8 = 0b0000_0010;
    const TIE: u8 = 0b0000_0001;
    const AE: u8 = 0b1000_0000; // alarm enable/disable for all four settings
    const TE: u8 = 0b1000_0000; // timer enable/disable
    const FE: u8 = 0b1000_0000; // clockout enable/disable
    const VL: u8 = 0b1000_0000; // voltage low detector flag
    const C: u8 = 0b1000_0000; // century flag
}

const DEVICE_ADDRESS: u8 = 0x51;
//const DEVICE_ADDRESS: u8 = 0xa2;

/// Two possible choices, used for various enable/disable bit flags
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]

pub enum Control {
    /// Enable some feature, eg. timer
    On,
    /// Disable some feature, eg. timer
    Off,
}

/// PCF8563 driver
#[derive(Debug, Default)]
pub struct PCF8563<I2C> {
    /// The concrete I2C device implementation.
    i2c: I2C,
}

mod alarm;
mod clkout;
mod control;
mod datetime;
mod timer;
pub use clkout::ClkoutFreq;
pub use datetime::{DateTime, Time};
pub use timer::{InterruptOutput, TimerFreq};

impl<I2C> PCF8563<I2C>
where
    I2C: embedded_hal::i2c::I2c    
{
    /// Create a new instance of the PCF8563 driver.
    pub fn new(i2c: I2C) -> Self {
        PCF8563 { i2c }
    }

    /// Destroy driver instance, return I2C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Write to a register.
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<I2c::Error>> {
        let payload: [u8; 2] = [register, data];
        self.i2c.write(DEVICE_ADDRESS, &payload).map_err(Error::I2C)
    }

    /// Read from a register.
    fn read_register(&mut self, register: u8) -> Result<u8, Error<I2c::Error>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }

    /// Check if specific bits are set.
    fn is_register_bit_flag_high(&mut self, address: u8, bitmask: u8) -> Result<bool, Error<I2c::Error>> {
        let data = self.read_register(address)?;
        Ok((data & bitmask) != 0)
    }

    /// Set specific bits.
    fn set_register_bit_flag(&mut self, address: u8, bitmask: u8) -> Result<(), Error<I2c::Error>> {
        let data = self.read_register(address)?;
        if (data & bitmask) == 0 {
            self.write_register(address, data | bitmask)
        } else {
            Ok(())
        }
    }

    /// Clear specific bits.
    fn clear_register_bit_flag(&mut self, address: u8, bitmask: u8) -> Result<(), Error<I2c::Error>> {
        let data = self.read_register(address)?;
        if (data & bitmask) != 0 {
            self.write_register(address, data & !bitmask)
        } else {
            Ok(())
        }
    }
}

/// Convert the Binary Coded Decimal value to decimal (only the lowest 7 bits).
fn decode_bcd(input: u8) -> u8 {
    let digits: u8 = input & 0xf;
    let tens: u8 = (input >> 4) & 0x7;
    10 * tens + digits
}

/// Convert the decimal value to Binary Coded Decimal.
fn encode_bcd(input: u8) -> u8 {
    let digits: u8 = input % 10;
    let tens: u8 = input / 10;
    let tens = tens << 4;
    tens + digits
}

#[cfg(test)]
mod tests {
    //use embedded_hal_mock as hal;

    use super::*;

    #[test]
    fn can_convert_decode_bcd() {
        assert_eq!(0, decode_bcd(0b0000_0000));
        assert_eq!(1, decode_bcd(0b0000_0001));
        assert_eq!(9, decode_bcd(0b0000_1001));
        assert_eq!(10, decode_bcd(0b0001_0000));
        assert_eq!(11, decode_bcd(0b0001_0001));
        assert_eq!(19, decode_bcd(0b0001_1001));
        assert_eq!(20, decode_bcd(0b0010_0000));
        assert_eq!(21, decode_bcd(0b0010_0001));
        assert_eq!(59, decode_bcd(0b0101_1001));
    }

    #[test]
    fn can_convert_encode_bcd() {
        assert_eq!(0b0000_0000, encode_bcd(0));
        assert_eq!(0b0000_0001, encode_bcd(1));
        assert_eq!(0b0000_1001, encode_bcd(9));
        assert_eq!(0b0001_0000, encode_bcd(10));
        assert_eq!(0b0001_0001, encode_bcd(11));
        assert_eq!(0b0001_1001, encode_bcd(19));
        assert_eq!(0b0010_0000, encode_bcd(20));
        assert_eq!(0b0010_0001, encode_bcd(21));
        assert_eq!(0b0101_1001, encode_bcd(59));
    }
}
