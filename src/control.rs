//! Various functions related to the RTC control that are not specifically 
//! datetime-, timer-, alarm- or clock output-related will be defined here

use super::{PCF8563, hal, Error, Register, BitFlags, Control, TimerFreq};
use hal::blocking::i2c::{Write, WriteRead};

impl<I2C, E> PCF8563<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    
    /// Enable or disable external clock test mode.
    pub fn control_ext_clk_test_mode(&mut self, flag: Control) -> Result<(), Error<E>> {
        match flag {            
            Control::On => {
                self.set_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::TEST1)       
            }            
            Control::Off => {
                self.clear_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::TEST1)
            }
        }
    }

    /// Is the external clock test mode enabled?
    pub fn is_ext_clk_mode_enabled(&mut self) -> Result<bool, Error<E>> {
        self.is_register_bit_flag_high(Register::CTRL_STATUS_1, BitFlags::TEST1)
    }

    /// Start/stop the internal clock.
    pub fn control_clock(&mut self, flag: Control) -> Result<(), Error<E>> {
        match flag {
            Control::On => {
                self.clear_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::STOP)       
            }
            Control::Off => {
                self.set_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::STOP)
            }
        }
    }

    /// Check if the internal clock is running.
    pub fn is_clock_running(&mut self) -> Result<bool, Error<E>> {
        let flag = self.is_register_bit_flag_high(Register::CTRL_STATUS_1, BitFlags::STOP)?;
        let flag = flag^true;
        Ok(flag)
    }

    /// Enable or disable power-on-reset override facility.
     pub fn control_power_on_reset_override(&mut self, flag: Control) -> Result<(), Error<E>> {
        match flag {            
            Control::On => {
                self.set_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::TESTC)       
            }            
            Control::Off => {
                self.clear_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::TESTC)
            }
        }
    }

    /// Check if power-on-reset override facility is enabled.
    pub fn is_power_on_reset_override_enabled(&mut self) -> Result<bool, Error<E>> {
        self.is_register_bit_flag_high(Register::CTRL_STATUS_1, BitFlags::TESTC)
    }

    /// Check the status of the Voltage Low detector flag
    pub fn get_voltage_low_flag(&mut self) -> Result<bool, Error<E>> {
       self.is_register_bit_flag_high(Register::VL_SECONDS, BitFlags::VL)
    }
   
    /// Clear the voltage low detector flag.
    pub fn clear_voltage_low_flag(&mut self) -> Result<(), Error<E>> {
       self.clear_register_bit_flag(Register::VL_SECONDS, BitFlags::VL)
    }

    /// Initialize the RTC by setting all the control flags to zero, disabling alarms and timer, and setting the timer to the lowest frequency for power saving.
    pub fn rtc_init(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::CTRL_STATUS_1, 0)?; // clear all the control bits 
        self.write_register(Register::CTRL_STATUS_2, 0)?;
        self.clear_voltage_low_flag()?; // clear the low voltage flag
        self.disable_all_alarms()?; // disable alarm for all the components
        self.set_timer_frequency(TimerFreq::Timer_1_60Hz)?; // set timer frequency to 1/60 Hz 
        Ok(())
    }

}


