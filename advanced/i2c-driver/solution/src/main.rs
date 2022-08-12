use anyhow;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{config::MasterConfig, Master, MasterPins, I2C0},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys::*;
<<<<<<< HEAD
use i2c_driver_exercise::icm42670p::{DeviceAddr, ICM42670P};
=======
use i2c_driver_exercise::icm42670p::{ICM42670P, SlaveAddr};
>>>>>>> correct sensor name

// Dont change this file. Work in the lib.rs and modify it so main.rs runs.

fn main() -> anyhow::Result<()> {
    link_patches();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;

    let i2c = Master::<I2C0, _, _>::new(
        peripherals.i2c0,
        MasterPins { sda, scl },
        <MasterConfig as Default>::default().baudrate(400.kHz().into()),
    )?;

<<<<<<< HEAD
    let mut sensor = ICM42670P::new(i2c, DeviceAddr::AD0)?;

=======
    let mut sensor = ICM42670P::new(i2c, SlaveAddr::AD0)?;
    // If you are using an ESP32-C3-DevKitC-02, change to:
    // let mut sensor = ICM42670P::new(i2c, SlaveAddr::AD1)?;
>>>>>>> correct sensor name
    println!("Sensor init");
    let device_id = sensor.read_device_id_register()?;

    assert_eq!(device_id, 96_u8);
    println!("Hello, world, I am sensor {}", device_id);

    loop {
        FreeRtos.delay_ms(500u32);
    }
}
