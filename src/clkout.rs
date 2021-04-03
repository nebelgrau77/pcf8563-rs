//! All clock output-related functions will be defined here

use super::{PCF8563, DEVICE_ADDRESS, hal, Error, Register, BitFlags, Control, encode_bcd, decode_bcd};
use hal::blocking::i2c::{Write, WriteRead};

/// The four possible clock output frequency settings
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ClkoutFreq {    
    /// 32768 Hz
    Clkout_32kHz        = 0b0000_0000, 
    /// 1024 Hz
    Clkout_1kHz         = 0b0000_0001, 
    /// 32 Hz    
    Clkout_32Hz         = 0b0000_0010, 
    /// 1 Hz 
    Clkout_1Hz          = 0b0000_0011, 
}

impl ClkoutFreq {
    /// Converts the ClkoutFreq to an unsigned 8-bit value
    pub fn bits(self) -> u8 {
        self as u8
    }
}

impl<I2C, E> PCF8563<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>, 
{
    /// Set clock output frequency
    /// (Does not alter the clkout enabled/disabled bit)
    pub fn set_clkout_frequency(
        &mut self,
        frequency: ClkoutFreq,
    ) -> Result<(), Error<E>> {        
        let data = self.read_register(Register::CLKOUT_CTRL)?; // read current value
        let data = data & 0b1000_0000; // keep the FE bit as is         
        let data = data | frequency.bits(); // set the lowest two bits
        self.write_register(Register::CLKOUT_CTRL, data)
    }
    
    /// Enable or disable clock output
    pub fn control_clkout(&mut self, status: Control) -> Result<(), Error<E>> {
        match status {
            Control::Enable => {
                self.set_register_bit_flag(Register::CLKOUT_CTRL, BitFlags::FE)       
            }
            Control::Disable => {
                self.clear_register_bit_flag(Register::CLKOUT_CTRL, BitFlags::FE)
            }
        }
    }

    /*
    /// Enable clock output
    pub fn enable_clkout(&mut self) -> Result<(), Error<E>> {
        self.set_register_bit_flag(Register::CLKOUT_CTRL, BitFlags::FE)
    }

    /// Disable clock output
       pub fn disable_clkout(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::CLKOUT_CTRL, BitFlags::FE)
    }
    */

}