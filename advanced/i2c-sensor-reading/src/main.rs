use anyhow::Result;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
use icm42670p::{DeviceAddr, ICM42670P};
use shared_bus::BusManagerSimple;
use shtcx::{self, PowerMode};
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;

// goals of this exercise:
// instantiate i2c peripheral
// implement one sensor, print sensor values
// implement second sensor on same bus to solve an ownership problem

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // Instanciate the i2c peripheral, correct pins are in the training material.

    // Create an instance of the SHTC3 sensor, find help in the documentation.

    // Read and print the sensor's device ID, find the methods in the documentation.

    loop {
        // This loop initiates measurements, reads values and prints humidity in % and Temperature in °C.
        FreeRtos.delay_ms(500u32);
    }
}
