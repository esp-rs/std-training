use anyhow::Result;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
use icm42670::{Address, Icm42670, PowerMode as imuPowerMode};
use shared_bus::BusManagerSimple;
use shtcx::{self, PowerMode as shtPowerMode};
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

    // 3. Instantiate the bus manager, pass the i2c bus.
    let bus = BusManagerSimple::new(i2c);

    // 4. Create two proxies. Now, each sensor can have their own instance of a proxy i2c, which resolves the ownership problem.
    let proxy_1 = bus.acquire_i2c();
    let proxy_2 = bus.acquire_i2c();

    // 5. Change your previous code, so that one of the proxies is passed to the SHTC3, instead of the original i2c bus.
    let mut sht = shtcx::shtc3(proxy_1);

    // 6. Read and print the device ID.
    let device_id = sht.device_identifier().unwrap();
    println!("Device ID SHTC3: {:#02x}", device_id);

    // 7. Create an instance of ICM42670p sensor. Pass the second proxy and the sensor's address.
    let mut imu = Icm42670::new(proxy_2, Address::Primary).unwrap();

    // 8. Read the device's ID register and print the value.
    let device_id = imu.device_id().unwrap();
    println!("Device ID ICM42670p: {:#02x}", device_id);

    // 9. Start the ICM42670p in low noise mode.
    imu.set_power_mode(imuPowerMode::GyroLowNoise).unwrap();

    loop {
        // 10. Read gyro data
        let gyro_data = imu.gyro_norm().unwrap();
        sht.start_measurement(shtPowerMode::NormalMode).unwrap();
        FreeRtos.delay_ms(100u32);
        let measurement = sht.get_measurement_result().unwrap();

        // 11. Print all values
        println!(
            "TEMP: {:.2} °C | HUM: {:.2} % | GYRO: X= {:.2}  Y= {:.2}  Z= {:.2}",
            measurement.temperature.as_degrees_celsius(),
            measurement.humidity.as_percent(),
            gyro_data.x,
            gyro_data.y,
            gyro_data.z,
        );

        FreeRtos.delay_ms(500u32);
    }
}
