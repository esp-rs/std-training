#![deny(unsafe_code)]

use embedded_hal::blocking::i2c;

/// ICM42670P device driver, represented by a struct with 2 fields.
/// Datasheet: https://3cfeqx1hf82y3xcoull08ihx-wpengine.netdna-ssl.com/wp-content/uploads/2021/07/DS-000451-ICM-42670-P-v1.0.pdf
#[derive(Debug)]
pub struct ICM42670P<I2C> {
    /// The concrete I²C device implementation.
    //  TODO! field 1 
    /// Device address
    //  TODO! field 2 
}

// See Table 3.3.2 in Documentation
/// Contains the possible variants of the devices addesses as binary numbers.
#[derive(Debug, Clone, Copy, PartialEq)]
// TODO! Implement the public enum with the two adress variants here





// impl block with methods
impl<I2C, E>ICM42670P<I2C>
where
    // this defines which error messages will be used
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Creates a new instance of the sensor, taking ownership of the i2c peripheral.
    pub fn new(i2c: I2C, address: DeviceAddr) -> Result<Self, E> {
        // instantiates the ICM42670P struct 
        // returns the struct as Ok value
        // TODO!
    }

    /// Returns the device's ID `0x67 
    //(if it doesn't, something is amiss)
    // Public method that can be accessed from outside this file.
    pub fn read_device_id_register(&mut self) -> Result<u8, E> {
        // reads the Device ID register
        // TODO!
    }

    /// Writes into a register
    // This method is not public as it is only needed inside this file.
    #[allow(unused)]
    fn write_register(&mut self, register: Register, value: u8) -> Result<(), E> {
        // value that will be written as u8
        // i2c write 
        // TODO!
    }

    /// Reads a register using a `write_read` method.
    // This method is not public as it is only needed inside this file.
    fn read_register(&mut self, register: Register) -> Result<u8, E> {
        // buffer for values
        // i2c write_read
        // return u8 from le bytes
        // TODO!
}

// See Table 14.1 in documentation
/// This enum represents the device's registers
#[derive(Clone, Copy)]
pub enum Register {
    WhoAmI = // // TODO! Fill in the register
}

impl Register {
    fn address(&self) -> u8 {
        // TODO! Returns Register as u8
    }
}
