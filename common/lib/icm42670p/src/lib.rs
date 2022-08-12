#![deny(unsafe_code)]
#![no_std]

use embedded_hal::blocking::i2c;

/// ICM42670P device driver.
/// Datasheet: https://3cfeqx1hf82y3xcoull08ihx-wpengine.netdna-ssl.com/wp-content/uploads/2021/07/DS-000451-ICM-42670-P-v1.0.pdf
/// 
#[derive(Debug)]
pub struct ICM42670P<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,

    /// Device address
    address: DeviceAddr,
}

/// see Table 3.3.2
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceAddr {
    /// ADP_AD0 = 0
    B110_1000 = 0b110_1000,
    /// ADP_AD0 = 1
    B110_1001 = 0b110_1001,
}

impl<I2C, E>ICM42670P<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Create a new instance of the ICM42670P.
    pub fn new(i2c: I2C, address: DeviceAddr) -> Result<Self, E> {

        let icm42670p = ICM42670P { i2c, address };

        Ok(icm42670p)
    }

    /// Reads device ID.
    /// Should return `0x67`. (if it doesn't, something is amiss)
    pub fn read_device_id_register(&mut self) -> Result<u8, E> {
        self.read_register(Register::WhoAmI)
    }

    /// Starts gyroscope sensor in low noise mode.
    pub fn gyro_ln(&mut self) -> Result<(), E> {
        let value: u8 = 0b11 << 2;
        self. write_pwr_mgmt(value)
    }

    /// Reads gyroscope sensor values.
    /// This may need some rework.
    pub fn read_gyro(&mut self) -> Result<Data, E> {
        

        let x0 = self.read_register(Register::GyroDataX0)?;
        let x1 = self.read_register(Register::GyroDataX1)?;
        let y0 = self.read_register(Register::GyroDataY0)?;
        let y1 = self.read_register(Register::GyroDataY1)?;
        let z0 = self.read_register(Register::GyroDataZ0)?;
        let z1 = self.read_register(Register::GyroDataZ1)?;

        let gyro_data = Data {
            x: i16::from_be_bytes([x1, x0]),
            y: i16::from_be_bytes([y1, y0]),
            z: i16::from_be_bytes([z1, z0]),
        };

        Ok(gyro_data)
        
    }

    /// Read PwrMgmt0 configuration
    pub fn read_pwr_configuration(&mut self) -> Result<PowerManagement, E> {
        let bits = self.read_register(Register::PwrMgmt0)?;
        Ok(PowerManagement { bits })
    }

    /// Write in PwrMgmt0 Register
    fn write_pwr_mgmt(&mut self, value: u8) -> Result<(), E> {
        self.write_register(Register::PwrMgmt0, value)
    }

    fn write_register(&mut self, register: Register, value: u8) -> Result<(), E> {
        let byte = value as u8;
        self.i2c
            .write(self.address as u8, &[register.address(), byte])
    }

    fn read_register(&mut self, register: Register) -> Result<u8, E> {
        let mut data = [0];
        self.i2c
            .write_read(self.address as u8, &[register.address()], &mut data)?;
        Ok(u8::from_le_bytes(data))
    }
}

pub struct PowerManagement {
    pub bits: u8,
}

pub struct Data {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

// Table 14.1
#[derive(Clone, Copy)]
pub enum Register {
    GyroDataX1 = 0x11,
    GyroDataX0 = 0x12,
    GyroDataY1 = 0x13,
    GyroDataY0 = 0x14,
    GyroDataZ1 = 0x15,
    GyroDataZ0 = 0x16,
    PwrMgmt0 = 0x1F,
    WhoAmI = 0x75,
}

impl Register {
    fn address(&self) -> u8 {
        *self as u8
    }
}
