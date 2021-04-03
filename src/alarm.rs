//! All alarm-related functions will be defined here
//! 
//! As it is now, setting an alarm component (minutes, hours, day, weekday) enables alarm for this component
//! TO DO: Keep the enabled/disabled bit when setting the alarm components (minutes, hours, day, weekday)

use super::{BitFlags, Error, Register, DEVICE_ADDRESS, PCF8563, hal, encode_bcd, decode_bcd};
use hal::blocking::i2c::{Write, WriteRead};

impl<I2C, E> PCF8563<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>, 
{

    /// Set the alarm minutes [0-59]
    pub fn set_alarm_minutes(&mut self, minutes: u8) -> Result<(), Error<E>> {
        if minutes > 59 {
            return Err(Error::InvalidInputData);
        }
        self.write_register(Register::MINUTE_ALARM, encode_bcd(minutes))
    }

    /// Set the alarm hours [0-23]
    pub fn set_alarm_hours(&mut self, hours: u8) -> Result<(), Error<E>> {
        if hours > 23 {
            return Err(Error::InvalidInputData);
        }
        self.write_register(Register::HOUR_ALARM, encode_bcd(hours))
    }

    /// Set the alarm day [1-31]
    pub fn set_alarm_day(&mut self, day: u8) -> Result<(), Error<E>> {
        if day < 1 || day > 31 {
            return Err(Error::InvalidInputData);
        }
        self.write_register(Register::DAY_ALARM, encode_bcd(day))
    }

    /// Set the alarm weekday [0-6]
    pub fn set_alarm_weekday(&mut self, weekday: u8) -> Result<(), Error<E>> {
        if weekday > 6 {
            return Err(Error::InvalidInputData);
        }
        self.write_register(Register::WEEKDAY_ALARM, encode_bcd(weekday))
    }
     
    /// Disable alarm minutes    
    pub fn disable_alarm_minutes(&mut self) -> Result<(), Error<E>> {
        self.set_register_bit_flag(Register::MINUTE_ALARM, BitFlags::AE)
    }

    /// Enable alarm minutes
    pub fn enable_alarm_minutes(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::MINUTE_ALARM, BitFlags::AE)
    }

    /// Disable alarm hours
    pub fn disable_alarm_hours(&mut self) -> Result<(), Error<E>> {
        self.set_register_bit_flag(Register::HOUR_ALARM, BitFlags::AE)
    }

    /// Enable alarm hours
    pub fn enable_alarm_hours(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::HOUR_ALARM, BitFlags::AE)
    }

    /// Disable alarm day
    pub fn disable_alarm_day(&mut self) -> Result<(), Error<E>> {
        self.set_register_bit_flag(Register::DAY_ALARM, BitFlags::AE)
    }

    /// Enable alarm day
    pub fn enable_alarm_day(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::DAY_ALARM, BitFlags::AE)
    }

    /// Disable alarm weekday
    pub fn disable_alarm_weekday(&mut self) -> Result<(), Error<E>> {
        self.set_register_bit_flag(Register::WEEKDAY_ALARM, BitFlags::AE)
    }

    /// Enable alarm weekday
    pub fn enable_alarm_weekday(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::WEEKDAY_ALARM, BitFlags::AE)
    }

     /// Enable alarm interrupt
     pub fn enable_alarm_interrupt(&mut self) -> Result<(), Error<E>> {
        self.set_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::AIE)
    }

    /// Disable alarm interrupt
    pub fn disable_alarm_interrupt(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::AIE)
    }

    /// Clear alarm flag
    pub fn clear_alarm_flag(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::AF)
    }

}