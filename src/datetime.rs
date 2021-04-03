use super::{BitFlags, Error, Register, DEVICE_ADDRESS, PCF8563, hal, encode_bcd, decode_bcd};
//use embedded_hal as hal;
use hal::blocking::i2c::{Write, WriteRead};

/// Date and time
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct DateTime {    
    /// Year [0-99]
    pub year: u8,    
    /// Month [1-12]
    pub month: u8,    
    /// Weekday [0-6]
    pub weekday: u8,    
    /// Days [1-31]
    pub day: u8,    
    /// Hours [0-23]
    pub hours: u8,
    /// Minutes [0-59]
    pub minutes: u8,
    /// Seconds [0-59]
    pub seconds: u8,
}

impl<I2C, E> PCF8563<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>, 
{
    /* Reading single elements won't be implemented, following the recommendations
    of the NXP datasheet and application notes for the chip, to read and write the date
    and time in one go.

    /// Read the seconds.
    pub fn get_seconds(&mut self) -> Result<u8, Error<E>> {
        let data = self.read_register(Register::VL_SECONDS)?;
        Ok(decode_bcd(data))
    }
     */

    /// Read date and time all at once.
    pub fn get_datetime(&mut self) -> Result<DateTime, Error<E>> {
        let mut data = [0;7];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[Register::VL_SECONDS], &mut data)
            .map_err(Error::I2C)?;
        Ok(DateTime {            
            year: decode_bcd(data[6]),
            month: decode_bcd(data[5]),
            weekday: decode_bcd(data[4]) & 0x7, //make sure we get only the bits 2:0
            day: decode_bcd(data[3]),
            hours: decode_bcd(data[2]),            
            minutes: decode_bcd(data[1]),
            seconds: decode_bcd(data[0]),
        })
    }

    /// Set the date and time 
    ///
    /// Will return an 'Error::InvalidInputData' if any of the parameters is out of range
    pub fn set_datetime(&mut self, datetime: &DateTime) -> Result<(), Error<E>> {
        if datetime.year > 99 ||
           datetime.month < 1 || datetime.month > 12  ||
           datetime.month > 6  ||
           datetime.day < 1 || datetime.month > 31  ||
           datetime.hours > 23  ||
           datetime.minutes > 59  ||
           datetime.seconds > 59  {
            return Err(Error::InvalidInputData);
        }    
    let payload = [
        Register::VL_SECONDS, //first register
        encode_bcd(datetime.seconds),
        encode_bcd(datetime.minutes),
        encode_bcd(datetime.hours),
        encode_bcd(datetime.day),
        encode_bcd(datetime.weekday),
        encode_bcd(datetime.month) | 0x80, //century bit set to 1
        encode_bcd(datetime.year),
        ];
    self.i2c.write(DEVICE_ADDRESS, &payload).map_err(Error::I2C)
    }
}
