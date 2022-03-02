#![deny(unsafe_code)]
#![no_std]

use embedded_hal::blocking::i2c;

/// IMC42670P device driver.
/// Datasheet: https://3cfeqx1hf82y3xcoull08ihx-wpengine.netdna-ssl.com/wp-content/uploads/2021/07/DS-000451-ICM-42670-P-v1.0.pdf
#[derive(Debug)]
pub struct IMC42670P<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,

    /// Device address
    address: SlaveAddr,
}

// see Table 3.3.2
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlaveAddr {

    AD0 = 0b110_1000,
    AD1 = 0b110_1001,
}

impl<I2C, E>IMC42670P<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Creates a new instance of the sensor, taking ownership of the i2c peripheral
    pub fn new(i2c: I2C, address: SlaveAddr) -> Result<Self, E> {

        let imc42670p = IMC42670P { i2c, address };

        Ok(imc42670p)
    }

    /// Should return `0x67 (if it doesn't, something is amiss)
    /// public method that can be accessed from outside this file
    pub fn read_device_id_register(&mut self) -> Result<u16, E> {
        self.read_register(Register::WhoAmI)
    }

    /// reads a register using a `write_read` method.
    /// this method is not public as it is only needed inside this file
    fn read_register(&mut self, register: Register) -> Result<u16, E> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address as u8, &[register.address()], &mut data)?;
        Ok(u16::from_le_bytes(data))
    }
}

// Table 14.1
#[derive(Clone, Copy)]
pub enum Register {
    WhoAmI = 0x75,
}

impl Register {
    fn address(&self) -> u8 {
        *self as u8
    }
}
