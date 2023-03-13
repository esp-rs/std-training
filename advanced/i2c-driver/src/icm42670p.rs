#![deny(unsafe_code)]

use core::marker::PhantomData;
use embedded_hal::blocking::i2c;

/// ICM42670P device driver, represented by a struct with 2 fields.
/// Datasheet: https://invensense.tdk.com/wp-content/uploads/2021/07/DS-000451-ICM-42670-P-v1.0.pdf
#[derive(Debug)]
pub struct ICM42670P<I2C> {
    // The concrete I²C device implementation.
    //  TODO! field 1
    // Device address
    //  TODO! field 2
    // Remove the following line as soon as the I2C parameter is used.
    rec_type: PhantomData<I2C>,
}

// See Table 3.3.2 in Documentation
/// Contains the possible variants of the devices addesses as binary numbers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceAddr {
    AD0, // Add address
    AD1, // Add address
}

// impl block with methods
impl<I2C, E> ICM42670P<I2C>
where
    // This defines which error messages will be used
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Creates a new instance of the sensor, taking ownership of the i2c peripheral.
    pub fn new(i2c: I2C, address: DeviceAddr) -> Result<Self, E> {
        // Instantiates the ICM42670P struct
        // Returns the struct as Ok value
        todo!();
    }

    /// Returns the device's ID `0x67
    //(if it doesn't, something is amiss)
    // Public method that can be accessed from outside this file.
    pub fn read_device_id_register(&mut self) -> Result<u8, E> {
        // Reads the Device ID register
        todo!();
    }

    /// Writes into a register
    // This method is not public as it is only needed inside this file.
    #[allow(unused)]
    fn write_register(&mut self, register: Register, value: u8) -> Result<(), E> {
        // Value that will be written as u8
        // i2c write
        todo!();
    }

    /// Reads a register using a `write_read` method.
    // This method is not public as it is only needed inside this file.
    fn read_register(&mut self, register: Register) -> Result<u8, E> {
        // Buffer for values
        // i2c write_read
        // Return u8 from le bytes
        todo!();
    }
}

// See Table 14.1 in documentation
/// This enum represents the device's registers
#[derive(Clone, Copy)]
pub enum Register {
    // WhoAmI Register
}

impl Register {
    fn address(&self) -> u8 {
        // Returns Register as u8
        todo!();
    }
}
