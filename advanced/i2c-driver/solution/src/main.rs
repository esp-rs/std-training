use anyhow;
use esp_idf_hal::{
    i2c::{config::MasterConfig, Master, MasterPins, I2C0},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys::*;
use i2c_driver_exercise::icm42670p::{ICM42670P, SlaveAddr};

// Dont change this file. Work in the lib.rs and modify it so main.rs runs.

fn main() -> anyhow::Result<()> {
    link_patches();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;
    // If you are using an ESP32-C3-DevKitC-02, change to:
    // let sda = peripherals.pins.gpio4;
    // let scl = peripherals.pins.gpio5;

    let i2c = Master::<I2C0, _, _>::new(
        peripherals.i2c0,
        MasterPins { sda, scl },
        <MasterConfig as Default>::default().baudrate(400.kHz().into()),
    )?;

    let mut sensor = ICM42670P::new(i2c, SlaveAddr::AD0)?;
    // If you are using an ESP32-C3-DevKitC-02, change to:
    // let mut sensor = ICM42670P::new(i2c, SlaveAddr::AD1)?;
    println!("Sensor init");
    let device_id = sensor.read_device_id_register()?;

    assert_eq!(device_id, 103_u16);
    println!("Hello, world, I am sensor {}", device_id);

   loop {};


}

