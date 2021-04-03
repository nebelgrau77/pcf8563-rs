use super::{BitFlags, Error, Register, PCF8563, hal, encode_bcd, decode_bcd};
use hal::blocking::i2c::{Write, WriteRead};

// it's easy to make the setter, jsut writing the bitflags
// how to make the getter?


impl<I2C, E> PCF8563<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
 
}
