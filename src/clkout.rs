//! All clock output-related functions will be defined here

use super::{I2c, BitFlags, Control, Error, Register, PCF8563};


/// The four possible clock output frequency settings
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ClkoutFreq {
    /// Set frequency to 32768 Hz.
    Clkout_32768Hz = 0b0000_0000,
    /// Set frequency to 1024 Hz.
    Clkout_1024Hz = 0b0000_0001,
    /// Set frequency to 32 Hz.    
    Clkout_32Hz = 0b0000_0010,
    /// Set frequency to 1 Hz.
    Clkout_1Hz = 0b0000_0011,
}

impl ClkoutFreq {
    /// Converts the ClkoutFreq to an unsigned 8-bit value.
    pub fn bits(self) -> u8 {
        self as u8
    }
}

impl<I2C, E> PCF8563<I2C>
where
    I2C: I2c<Error = E>, 
{
    /// Set clock output frequency (does not alter the clkout enabled/disabled bit).
    pub fn set_clkout_frequency(&mut self, frequency: ClkoutFreq) -> Result<(), Error<E>> {
        let data = self.read_register(Register::CLKOUT_CTRL)?; // read current value
        let data = data & 0b1000_0000; // keep the FE bit as is
        let data = data | frequency.bits(); // set the lowest two bits
        self.write_register(Register::CLKOUT_CTRL, data)
    }

    /// Enable or disable clock output.
    pub fn control_clkout(&mut self, status: Control) -> Result<(), Error<E>> {
        match status {
            Control::On => self.set_register_bit_flag(Register::CLKOUT_CTRL, BitFlags::FE),
            Control::Off => self.clear_register_bit_flag(Register::CLKOUT_CTRL, BitFlags::FE),
        }
    }

    /// Is the clock output enabled?
    pub fn is_clkout_enabled(&mut self) -> Result<bool, Error<E>> {
        self.is_register_bit_flag_high(Register::CLKOUT_CTRL, BitFlags::FE)
    }

    //TO DO:
    //
    // pub fn get_clkout_frequency()
}
