//! This is a platform agonostic Rust driver for the NXP PCF8563 real-time clock,
//! based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.

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
    }   

const DEVICE_ADDRESS: u8 = 0x51;

/// Control states
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]

/// Two possible choices, used for various enable/disable bit flags
pub enum Control {    
    /// Enable some feature, eg. timer 
    Enable, 
    /// Disable some feature, eg. timer
    Disable,     
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
pub use datetime::DateTime;
pub use timer::TimerFreq;
pub use clkout::ClkoutFreq;
pub use control::InterruptOutput;

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

    // do I need WriteRead, or maybe just Read here?

    /// Read from a register 
    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0]; //need to figure out sending whole datetime
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