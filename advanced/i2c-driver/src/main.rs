use anyhow::Result;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;

// Uncomment the following line to run the solution, check lib.rs for further instructions
use i2c_driver::icm42670p_solution::{DeviceAddr, ICM42670P};

// Comment out the following line to run the solution, check lib.rs for further instructions
// use i2c_driver::icm42670p::{DeviceAddr, ICM42670P};

// Dont change this file. Work in the icm42670p.rs and modify it so main.rs runs.

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;

    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;

    let mut sensor = ICM42670P::new(i2c, DeviceAddr::AD0)?;

    println!("Sensor init");
    let device_id = sensor.read_device_id_register()?;

    println!("Hello, world, I am sensor {:#02x}", device_id);

    loop {
        FreeRtos.delay_ms(500u32);
    }
}
