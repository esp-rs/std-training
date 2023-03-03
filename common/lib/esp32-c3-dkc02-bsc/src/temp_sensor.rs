// use core::fmt::Debug;
// use embedded_hal::blocking::i2c::{Write, WriteRead};
// use icm42670::{Address, Icm42670};
// use shtcx::{self, shtc3, PowerMode};

// pub struct BoardTempSensor<I2C> {
//     sht: shtcx::ShtC3<I2C>,
//     imu: Icm42670<I2C>,
// }

// impl<I2C, E> BoardTempSensor<I2C>
// where
//     I2C: Write<Error = E> + WriteRead<Error = E>,
//     E: Debug,
// {
//     pub fn new(sht_i2c: I2C, imu_i2c: I2C) -> Self {
//         let icm42670p = Icm42670::new(imu_i2c, Address::Primary).unwrap();
//         let sht = shtc3(sht_i2c);
//         Self {
//             sht,
//             imu: icm42670p,
//         }
//     }

//     // pub fn read_sht(&mut self) -> f32 {
//     //     let temp = self.sht.read_temperature(PowerMode::Normal).unwrap();
//     //     temp
//     // }

//     // pub fn read_imu(&mut self) -> f32 {
//     //     let temp = self.imu.read_temperature().unwrap();
//     //     temp
//     // }
// }
