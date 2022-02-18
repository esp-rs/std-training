#![deny(unsafe_code)]
#![no_std]

use embedded_hal::blocking::i2c;

/// IMC42670P device driver.
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
    /// ADP_AD0 = 0
    B110_1000 = 0b110_1000,
    /// ADP_AD0 = 1
    B110_1001 = 0b110_1001,
}

impl<I2C, E>IMC42670P<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Create new instance of the IMC42670P
    ///
    /// Configured to
    /// -TODO Name default configuration
    /// 
    ///
    /// 
    pub fn new(i2c: I2C, address: SlaveAddr) -> Result<Self, E> {

        let imc42670p = IMC42670P { i2c, address };

        Ok(imc42670p)
    }

    /// Should return `0x67 (if it doesn't, something is amiss)
    pub fn read_device_id_register(&mut self) -> Result<u16, E> {
        self.read_register(Register::DeviceId)
    }

    /// SHOULD be called every 500 milliseconds to get a new reading
    // pub fn read_temperature(&mut self) -> Result<Temperature, E> {
    //     let bits = self.read_register(Register::Temperature)?;
    //     Ok(Temperature { bits })
    // }

    /// Set a high temperature threshold.
    /// When a NEW temperature reading goes above this threshold:
    /// - the ALERT_High flag is set AND
    /// - the ALERT pin becomes active (LOW)
    // pub fn set_high_temperature_threshold(&mut self, threshold: Temperature) -> Result<(), E> {
    //     self.write_register(Register::HighLimit, threshold.bits)
    // }

    // fn write_configuration(&mut self, value: u16) -> Result<(), E> {
    //     self.write_register(Register::Configuration, value)
    // }

    /// Read the current configuration and resets all high temperature alerts.
    /// This operation:
    /// - clears the ALERT_High flag
    /// - deasserts the ALERT pin (HIGH)
    // pub fn read_configuration(&mut self) -> Result<Configuration, E> {
    //     let bits = self.read_register(Register::Configuration)?;
    //     Ok(Configuration { bits })
    // }

    fn write_register(&mut self, register: Register, value: u16) -> Result<(), E> {
        let msb = (value >> 8) as u8;
        let lsb = value as u8;
        self.i2c
            .write(self.address as u8, &[register.address(), msb, lsb])
    }

    fn read_register(&mut self, register: Register) -> Result<u16, E> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address as u8, &[register.address()], &mut data)?;
        Ok(u16::from_le_bytes(data))
    }
}

// pub struct Configuration {
//     bits: u16,
// }

// impl Configuration {
//     pub fn is_alert_high_flag_set(&self) -> bool {
//         const ALERT_HIGH_FLAG: u16 = 1 << 15;
//         self.bits & ALERT_HIGH_FLAG != 0
//     }
// }

// #[derive(Clone, Copy)]
// pub struct Temperature {
//     bits: u16,
// }

// impl Temperature {
//     pub fn celsius(degrees: f32) -> Self {
//         Self {
//             bits: float_to_integer(degrees) as u16,
//         }
//     }

//     pub fn as_f32(&self) -> f32 {
//         integer_to_float(self.as_i16())
//     }

//     pub fn as_f64(&self) -> f64 {
//         integer_to_float(self.as_i16()) as f64
//     }

//     pub fn as_i16(&self) -> i16 {
//         self.bits as i16
//     }
// }

// Table 7-3
#[derive(Clone, Copy)]
pub enum Register {
    // Temperature = 0x00,
    // HighLimit = 0x02,
    // LowLimit = 0x03,
    // Configuration = 0x01,
    DeviceId = 0x75,
}

impl Register {
    fn address(&self) -> u8 {
        *self as u8
    }
}

// fn integer_to_float(value: i16) -> f32 {
//     const RESOLUTION: f32 = 0.0078125;

//     value as f32 * RESOLUTION
// }

// fn float_to_integer(value: f32) -> i16 {
//     const RESOLUTION: f32 = 0.0078125;

//     (value / RESOLUTION) as i16
// }