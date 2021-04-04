//! All alarm-related functions will be defined here
//! 
//! As it is now, setting an alarm component (minutes, hours, day, weekday) enables alarm for this component
//! TO DO: Keep the enabled/disabled bit when setting the alarm components (minutes, hours, day, weekday)

use super::{PCF8563, DEVICE_ADDRESS, hal, Error, Register, BitFlags, Control, encode_bcd, decode_bcd};
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
    
    
    /// Control alarm minutes (reverse logic: 0 is enabled, 1 is disabled)
    pub fn control_alarm_minutes(&mut self, status: Control) -> Result<(), Error<E>> {
        match status {
            Control::Off => {
                self.set_register_bit_flag(Register::MINUTE_ALARM, BitFlags::AE)       
            }
            Control::On => {
                self.clear_register_bit_flag(Register::MINUTE_ALARM, BitFlags::AE)
            }
        }
    }
    
    /// Is alarm minutes enabled? (reverse logic: 1 disabled, 0 enabled) 
     pub fn is_alarm_minutes_enabled(&mut self) -> Result<bool, Error<E>> {
        let flag = self.is_register_bit_flag_high(Register::MINUTE_ALARM, BitFlags::AE)?;
        let flag = flag^true;
        Ok(flag)
    }

    /// Control alarm hours (reverse logic: 0 is enabled, 1 is disabled)
    pub fn control_alarm_hours(&mut self, status: Control) -> Result<(), Error<E>> {
        match status {
            Control::Off => {
                self.set_register_bit_flag(Register::HOUR_ALARM, BitFlags::AE)       
            }
            Control::On => {
                self.clear_register_bit_flag(Register::HOUR_ALARM, BitFlags::AE)
            }
        }
    }

     /// Is alarm hours enabled? (reverse logic: 1 disabled, 0 enabled) 
     pub fn is_alarm_hours_enabled(&mut self) -> Result<bool, Error<E>> {
        let flag = self.is_register_bit_flag_high(Register::HOUR_ALARM, BitFlags::AE)?;
        let flag = flag^true;
        Ok(flag)
    }

    /// Control alarm day (reverse logic: 0 is enabled, 1 is disabled)
    pub fn control_alarm_day(&mut self, status: Control) -> Result<(), Error<E>> {
        match status {
            Control::Off => {
                self.set_register_bit_flag(Register::DAY_ALARM, BitFlags::AE)       
            }
            Control::On => {
                self.clear_register_bit_flag(Register::DAY_ALARM, BitFlags::AE)
            }
        }
    }

    /// Is alarm day enabled? (reverse logic: 1 disabled, 0 enabled) 
    pub fn is_alarm_day_enabled(&mut self) -> Result<bool, Error<E>> {
        let flag = self.is_register_bit_flag_high(Register::DAY_ALARM, BitFlags::AE)?;
        let flag = flag^true;
        Ok(flag)
    }

    /// Control alarm weekday (reverse logic: 0 is enabled, 1 is disabled)
    pub fn control_alarm_weekday(&mut self, status: Control) -> Result<(), Error<E>> {
        match status {
            Control::Off => {
                self.set_register_bit_flag(Register::WEEKDAY_ALARM, BitFlags::AE)       
            }
            Control::On => {
                self.clear_register_bit_flag(Register::WEEKDAY_ALARM, BitFlags::AE)
            }
        }
    }

    /// Is alarm weekday enabled? (reverse logic: 1 disabled, 0 enabled) 
    pub fn is_alarm_weekday_enabled(&mut self) -> Result<bool, Error<E>> {
        let flag = self.is_register_bit_flag_high(Register::WEEKDAY_ALARM, BitFlags::AE)?;
        let flag = flag^true;
        Ok(flag)
    }

    /// Control alarm interrupt
    pub fn control_alarm_interrupt(&mut self, status: Control) -> Result<(), Error<E>> {
        match status {
            Control::On => {
                self.set_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::AIE)       
            }
            Control::Off => {
                self.clear_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::AIE)
            }
        }
    }

    /// Read the alarm minutes setting, returns minutes [0-59]        
    pub fn get_alarm_minutes(&mut self) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[Register::MINUTE_ALARM], &mut data)
            .map_err(Error::I2C)?;
        Ok(decode_bcd(data[0]))
    }

    /// Read the alarm hours setting, returns hours [0-23]        
    pub fn get_alarm_hours(&mut self) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[Register::HOUR_ALARM], &mut data)
            .map_err(Error::I2C)?;
        Ok(decode_bcd(data[0]))
    }

    /// Read the alarm day setting, returns day [1-31]        
    pub fn get_alarm_day(&mut self) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[Register::DAY_ALARM], &mut data)
            .map_err(Error::I2C)?;
        Ok(decode_bcd(data[0]))
    }

    /// Read the alarm weekday setting, returns weekday [0-6]        
    pub fn get_alarm_weekday(&mut self) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[Register::WEEKDAY_ALARM], &mut data)
            .map_err(Error::I2C)?;
        Ok(decode_bcd(data[0]))
    }

    /// Clear alarm flag
    pub fn clear_alarm_flag(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::AF)
    }

    /// Get the alarm flag (if true, alarm was triggered)
    pub fn get_alarm_flag(&mut self) -> Result<bool, Error<E>> {
        self.is_register_bit_flag_high(Register::CTRL_STATUS_2, BitFlags::AF)
    }

    /// Get the alarm interrupt status
    pub fn is_alarm_interrupt_enabled(&mut self) -> Result<bool, Error<E>> {
        self.is_register_bit_flag_high(Register::CTRL_STATUS_2, BitFlags::AIE)
    }

    /// Shut off the alarms 
    pub fn disable_all_alarms(&mut self) -> Result<(), Error<E>> {
        self.control_alarm_minutes(Control::Off)?;
        self.control_alarm_hours(Control::Off)?;
        self.control_alarm_day(Control::Off)?;
        self.control_alarm_weekday(Control::Off)?;
        Ok(())    
    }

}