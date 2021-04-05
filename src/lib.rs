//! This is a platform agnostic Rust driver for the NXP PCF8563 real-time clock,
//! based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//!
//! # Initialization
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
//! # Date and time
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
//! # Alarm
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
//! # Timer
//! 
//! All the timer-related functions are defined in the `timer.rs` module
//!
//! (to be completed)
//! 
//! # Clock output
//! 
//! All the clock output-related functions are defined in the `clkout.rs` module
//! 
//! (to be completed)
//! 
//! # RTC Control
//! 
//! All the other control functions are defined in the `control.rs` module
//! 
//! (to be completed)
//! 



#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

use embedded_hal as hal;

use hal::blocking::i2c::{Write, WriteRead};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2C(E),
    /// Invalid input data
    InvalidInputData,
}

struct Register;

impl Register {
    const CTRL_STATUS_1     : u8 = 0x00;
    const CTRL_STATUS_2     : u8 = 0x01;
    const VL_SECONDS        : u8 = 0x02;
    const MINUTES           : u8 = 0x03;
    const HOURS             : u8 = 0x04;
    const DAYS              : u8 = 0x05;
    const WEEKDAYS          : u8 = 0x06;
    const CENTURY_MONTHS    : u8 = 0x07;
    const YEARS             : u8 = 0x08;
    const MINUTE_ALARM      : u8 = 0x09;
    const HOUR_ALARM        : u8 = 0x0A;
    const DAY_ALARM         : u8 = 0x0B;
    const WEEKDAY_ALARM     : u8 = 0x0C;
    const CLKOUT_CTRL       : u8 = 0x0D;
    const TIMER_CTRL        : u8 = 0x0E;
    const TIMER             : u8 = 0x0F;
}

struct BitFlags;

impl BitFlags {
    const TEST1                 : u8 = 0b1000_0000;
    const STOP                  : u8 = 0b0010_0000;
    const TESTC                 : u8 = 0b0000_1000;
    const TI_TP                 : u8 = 0b0001_0000;
    const AF                    : u8 = 0b0000_1000;
    const TF                    : u8 = 0b0000_0100;
    const AIE                   : u8 = 0b0000_0010;
    const TIE                   : u8 = 0b0000_0001;
    const AE                    : u8 = 0b1000_0000; // alarm enable/disable for all four settings
    const TE                    : u8 = 0b1000_0000; // timer enable/disable
    const FE                    : u8 = 0b1000_0000; // clockout enable/disable
    const VL                    : u8 = 0b1000_0000; // voltage low detector flag
    const C                     : u8 = 0b1000_0000; // century flag
    }   

const DEVICE_ADDRESS: u8 = 0x51;

/// Control states
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]

/// Two possible choices, used for various enable/disable bit flags
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

mod datetime;
mod alarm;
mod timer;
mod clkout;
mod control;
pub use datetime::{DateTime, Time};
pub use timer::{TimerFreq, InterruptOutput};
pub use clkout::ClkoutFreq;


impl <I2C, E> PCF8563<I2C>
where 
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance.
    pub fn new(i2c: I2C) -> Self {
        PCF8563 { i2c }
    }

    /// Destroy driver instance, return I2C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Write to a register. Buffer is big enough to contain the whole datetime.
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<E>> {
        let payload: [u8; 2] = [register, data]; //need to figure out sending whole datetime
        self.i2c.write(DEVICE_ADDRESS, &payload).map_err(Error::I2C)
    }

    /// Read from a register 
    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }

    /// Check if specific bits are set
    fn is_register_bit_flag_high(&mut self, address: u8, bitmask: u8) -> Result<bool, Error<E>> {
        let data = self.read_register(address)?;
        Ok((data & bitmask) != 0)
    }

    /// Set specific bits
    fn set_register_bit_flag(&mut self, address: u8, bitmask: u8) -> Result<(), Error<E>> {
        let data = self.read_register(address)?;
        if (data & bitmask) == 0 { // does it mean that they are different if 0?
            self.write_register(address, data | bitmask)
        } else {
            Ok(())
        }
    }

    /// Clear specific bits
    fn clear_register_bit_flag(&mut self, address: u8, bitmask: u8) -> Result<(), Error<E>> {
        let data = self.read_register(address)?;
        if (data & bitmask) != 0 { // what does this mean?
            self.write_register(address, data & !bitmask)
        } else {
            Ok(())
        }
    }
}

/// convert the Binary Coded Decimal value to decimal (only the lowest 7 bits)
fn decode_bcd(input: u8) -> u8 {    
    let digits: u8 = input & 0xf;
    let tens: u8 = (input >> 4) & 0x7;
    10 * tens + digits
    }

/// convert the decimal value to Binary Coded Decimal
fn encode_bcd(input: u8) -> u8 {
    let digits: u8 = input%10;
    let tens: u8 = input/10;
    let tens = tens << 4;
    tens + digits
    }

#[cfg(test)]
mod tests {
    use embedded_hal_mock as hal;
    
    use super::*;

    #[test]
    fn can_convert_decode_bcd() {
        assert_eq!(0,  decode_bcd(0b0000_0000));
        assert_eq!(1,  decode_bcd(0b0000_0001));
        assert_eq!(9,  decode_bcd(0b0000_1001));
        assert_eq!(10, decode_bcd(0b0001_0000));
        assert_eq!(11, decode_bcd(0b0001_0001));
        assert_eq!(19, decode_bcd(0b0001_1001));
        assert_eq!(20, decode_bcd(0b0010_0000));
        assert_eq!(21, decode_bcd(0b0010_0001));
        assert_eq!(59, decode_bcd(0b0101_1001));
    }
    
    #[test]
    fn can_convert_encode_bcd() {
        assert_eq!(0b0000_0000, encode_bcd( 0));
        assert_eq!(0b0000_0001, encode_bcd( 1));
        assert_eq!(0b0000_1001, encode_bcd( 9));
        assert_eq!(0b0001_0000, encode_bcd(10));
        assert_eq!(0b0001_0001, encode_bcd(11));
        assert_eq!(0b0001_1001, encode_bcd(19));
        assert_eq!(0b0010_0000, encode_bcd(20));
        assert_eq!(0b0010_0001, encode_bcd(21));
        assert_eq!(0b0101_1001, encode_bcd(59));
    }
}