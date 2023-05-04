use anyhow::Result;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
use shtcx::{self, PowerMode};
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;

// Goals of this exercise:
// - Part1: Instantiate i2c peripheral
// - Part1: Implement one sensor, print sensor values
// - Part2: Implement second sensor on same bus to solve an ownership problem

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // 1. Instanciate the SDA and SCL pins, correct pins are in the training material.
    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;
    // 2. Instanciate the i2c peripheral
    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;

    // 3. Create an instance of the SHTC3 sensor.
    let mut sht = shtcx::shtc3(i2c);
    let device_id = sht.device_identifier().unwrap();

    // 4. Read and print the sensor's device ID.
    println!("Device ID SHTC3: {:#02x}", device_id);

    loop {
        // 5. This loop initiates measurements, reads values and prints humidity in % and Temperature in °C.
        sht.start_measurement(PowerMode::NormalMode).unwrap();
        FreeRtos.delay_ms(100u32);
        let measurement = sht.get_measurement_result().unwrap();

        println!(
            "TEMP: {:.2} °C | HUM: {:.2} %",
            measurement.temperature.as_degrees_celsius(),
            measurement.humidity.as_percent(),
        );

        FreeRtos.delay_ms(500u32);
    }
}
