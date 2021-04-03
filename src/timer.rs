//! All timer-related functions will be defined here

use super::{BitFlags, Error, Register, DEVICE_ADDRESS, PCF8563, hal, encode_bcd, decode_bcd};
use hal::blocking::i2c::{Write, WriteRead};

/// The four possible timer frequency settings
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TimerFreq {    
    /// 4096 Hz
    Timer_4096Hz    = 0b0000_0000, 
    /// 64 Hz
    Timer_64Hz      = 0b0000_0001, 
    /// 1 Hz    
    Timer_1Hz       = 0b0000_0010, 
    /// 1/60 Hz - should be used when timer is off to limit energy usage
    Timer_1_60Hz    = 0b0000_0011, 
}

impl TimerFreq {
    /// Converts the TimerFreq to an unsigned 8-bit value
    pub fn bits(self) -> u8 {
        self as u8
    }
}

impl<I2C, E> PCF8563<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>, 
{

    /// Set the timer [0-255]
    pub fn set_timer(&mut self, time: u8) -> Result<(), Error<E>> {        
        self.write_register(Register::TIMER, time)
    }

    /// Set timer frequency
    /// (Does not alter the timer enabled/disabled bit)
    pub fn set_timer_frequency(
        &mut self,
        frequency: TimerFreq,
    ) -> Result<(), Error<E>> {        
        let data = self.read_register(Register::TIMER_CTRL)?; // read current value
        let data = data & 0b1000_0000; // keep the TE bit as is         
        let data = data | frequency.bits(); // set the lowest two bits
        self.write_register(Register::TIMER_CTRL, data)
    }

    /// Enable timer
    pub fn enable_timer(&mut self) -> Result<(), Error<E>> {
        self.set_register_bit_flag(Register::TIMER_CTRL, BitFlags::TE)
    }

    /// Disable timer
       pub fn disable_timer(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::TIMER_CTRL, BitFlags::TE)
    }

    /// Enable timer interrupt
     pub fn enable_timer_interrupt(&mut self) -> Result<(), Error<E>> {
        self.set_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::TIE)
    }

    /// Disable timer interrupt
    pub fn disable_timer_interrupt(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::TIE)
    }

    /// Clear timer flag
    pub fn clear_timer_flag(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::TF)
    }

}