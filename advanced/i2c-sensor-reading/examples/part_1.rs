use anyhow;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys::*;

use shtcx::{self, PowerMode};

// goals of this exercise:
// instantiate i2c peripheral
// implement one sensor, print sensor values
// implement second sensor on same bus to solve an ownership problem

fn main() -> anyhow::Result<()> {
    link_patches();

    let peripherals = Peripherals::take().unwrap();

    // Instanciate the i2c peripheral, correct pins are in the training material.
    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;
    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;

    // Create an instance of the SHTC3 sensor.
    let mut sht = shtcx::shtc3(i2c);
    let device_id = sht.device_identifier().unwrap();

    // Read and print the sensor's device ID.
    println!("Device ID SHTC3: {}", device_id);

    loop {
        // This loop initiates measurements, reads values and prints humidity in % and Temperature in °C.
        sht.start_measurement(PowerMode::NormalMode).unwrap();
        FreeRtos.delay_ms(100u32);
        let measurement = sht.get_measurement_result().unwrap();

        println!(
            "TEMP: {} °C\n
            HUM: {:?} %\n
            \n
            ",
            measurement.temperature.as_degrees_celsius(),
            measurement.humidity.as_percent(),
        );

        FreeRtos.delay_ms(500u32);
    }
}
