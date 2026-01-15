//! All timer-related functions will be defined here

use super::{I2c, BitFlags, Control, Error, Register, DEVICE_ADDRESS, PCF8563};


/// Four possible timer frequency settings.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TimerFreq {
    /// Set timer frequency to 4096 Hz.
    Timer_4096Hz = 0b0000_0000,
    /// Set timer frequency to 64 Hz.
    Timer_64Hz = 0b0000_0001,
    /// Set timer frequency to 1 Hz.    
    Timer_1Hz = 0b0000_0010,
    /// Set timer frequency to 1/60 Hz. This should be used when timer is off to limit energy consumption.
    Timer_1_60Hz = 0b0000_0011,
}

impl TimerFreq {
    /// Converts the TimerFreq to an unsigned 8-bit value
    pub fn bits(self) -> u8 {
        self as u8
    }
}

/// Two possible timer interrupt output modes
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum InterruptOutput {
    /// Active when TF active (default setting).
    Continuous,
    /// Pulsating according to the datasheet.
    Pulsating,
}

impl<I2C, E> PCF8563<I2C>
where
    I2C: I2c<Error = E>, 
{
    /// Set the timer [0-255]
    pub fn set_timer(&mut self, time: u8) -> Result<(), Error<E>> {
        self.write_register(Register::TIMER, time)
    }

    /// Set timer frequency (does not alter the timer enabled/disabled bit).
    pub fn set_timer_frequency(&mut self, frequency: TimerFreq) -> Result<(), Error<E>> {
        let data = self.read_register(Register::TIMER_CTRL)?; // read current value
        let data = data & 0b1000_0000; // keep the TE bit as is
        let data = data | frequency.bits(); // set the lowest two bits
        self.write_register(Register::TIMER_CTRL, data)
    }

    /// Enable or disable the timer interrupt.
    pub fn control_timer(&mut self, flag: Control) -> Result<(), Error<E>> {
        match flag {
            Control::On => self.set_register_bit_flag(Register::TIMER_CTRL, BitFlags::TE),
            Control::Off => self.clear_register_bit_flag(Register::TIMER_CTRL, BitFlags::TE),
        }
    }

    /// Check if timer is enabled.
    pub fn is_timer_enabled(&mut self) -> Result<bool, Error<E>> {
        self.is_register_bit_flag_high(Register::TIMER_CTRL, BitFlags::TE)
    }

    /// Enable or disable timer interrupt.
    pub fn control_timer_interrupt(&mut self, flag: Control) -> Result<(), Error<E>> {
        match flag {
            Control::On => self.set_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::TIE),
            Control::Off => self.clear_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::TIE),
        }
    }

    /// Check if timer interrupt is enabled.
    pub fn is_timer_interrupt_enabled(&mut self) -> Result<bool, Error<E>> {
        self.is_register_bit_flag_high(Register::CTRL_STATUS_2, BitFlags::TIE)
    }

    /// Get the timer flag status (if true, timer was triggered).
    pub fn get_timer_flag(&mut self) -> Result<bool, Error<E>> {
        self.is_register_bit_flag_high(Register::CTRL_STATUS_2, BitFlags::TF)
    }

    /// Clear the timer flag.
    pub fn clear_timer_flag(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::TF)
    }

    /// Select the interrupt output mode when TF flag is set (continuous or pulsating).
    pub fn timer_interrupt_output(&mut self, output: InterruptOutput) -> Result<(), Error<E>> {
        match output {
            InterruptOutput::Continuous => {
                self.clear_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::TI_TP)
            }
            InterruptOutput::Pulsating => {
                self.set_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::TI_TP)
            }
        }
    }

    /// Read the current timer value.
    pub fn get_timer(&mut self) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[Register::TIMER], &mut data)
            .map_err(Error::I2C)?;
        Ok(data[0])
    }

    //TO DO:
    //
    // pub fn get_timer_interrupt_output()
    //
    // pub fn get_timer_frequency()
    //
    /* USE THIS EXAMPLE FOR GET_TIMER_FREQUENCY() ?

    pub fn get_square_wave_output_rate(&mut self) -> Result<SQWOUTRateBits, Error<E>> {
        let data = self.read_register(Register::SQWOUT)?;
        Ok(SQWOUTRateBits {
            rs0: (data & BitFlags::OUTRATERS0) != 0,
            rs1: (data & BitFlags::OUTRATERS1) != 0,
        })
    }
    */
}
