//! Various functions related to the RTC control that are not specifically 
//! datetime-, timer-, alarm- or clock output-related will be defined here

use super::{BitFlags, Error, Register, PCF8563, hal, encode_bcd, decode_bcd};
use hal::blocking::i2c::{Write, WriteRead};

impl<I2C, E> PCF8563<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    // start and stop clock
    // "init" function
    //  
}
