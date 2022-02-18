use anyhow::Result;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{config::MasterConfig, Master, MasterPins, I2C0},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys::*;
use i2c_driver_exercise::imc42670p::{IMC42670P, SlaveAddr};

// Dont change this file. Work in the lib.rs and modify it so main.rs runs.

fn main() -> anyhow::Result<()> {
    link_patches();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio4;
    let scl = peripherals.pins.gpio5;

    let i2c = Master::<I2C0, _, _>::new(
        peripherals.i2c0,
        MasterPins { sda, scl },
        <MasterConfig as Default>::default().baudrate(400.kHz().into()),
    )?;

    let mut sensor = IMC42670P::new(i2c, SlaveAddr::B110_1001)?;
    println!("Sensor init");
    let serial_num = sensor.read_device_id_register()?;

    println!("Serial Number: {}", serial_num);

   loop {};


}

