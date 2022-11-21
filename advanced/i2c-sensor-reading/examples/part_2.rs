use anyhow;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{config::MasterConfig, Master, MasterPins, I2C0},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys::*;
use icm42670p::{ICM42670P, DeviceAddr};

use shtcx::{self, PowerMode};

use shared_bus;

// goals of this exercise:
// instantiate i2c peripheral
// implement one sensor, print sensor values
// implement second sensor on same bus to solve an ownership problem

fn main() -> anyhow::Result<()>  {
    link_patches();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;

    let i2c = Master::<I2C0, _, _>::new(
        peripherals.i2c0,
        MasterPins { sda, scl },
        <MasterConfig as Default>::default().baudrate(400.kHz().into()),
    )?;

    // Instantiate the bus manager, pass the i2c bus. 
    let bus = shared_bus::BusManagerSimple::new(i2c);

    // Create two proxies. Now, each sensor can have their own instance of a proxy i2c, which resolves the ownership problem. 
    let proxy_1 =bus.acquire_i2c();
    let proxy_2 =bus.acquire_i2c();

    // Change your previous code, so that one of the proxies is passed to the SHTC3, instead of the original i2c bus. 
    let mut sht = shtcx::shtc3(proxy_1);

    // Read and print the device ID.
    let device_id = sht.device_identifier().unwrap();
    println!("Device ID SHTC3: {}", device_id);

    // Create an instance of ICM42670p sensor. Pass the second proxy and the sensor's address. 
    let mut imu = ICM42670P::new(proxy_2, DeviceAddr::B110_1000)?;

    // Read the device's ID register and print the value. 
    let device_id = imu.read_device_id_register()?;
    println!("Device ID ICM42670p: {}", device_id);
  
    // Start the ICM42670p in low noise mode.
    imu.gyro_ln()?;



    loop {
        // Read gyro data
        let gyro_data =imu.read_gyro()?;
        sht.start_measurement(PowerMode::NormalMode).unwrap();
        FreeRtos.delay_ms(100u32);
        let measurement = sht.get_measurement_result().unwrap(); 
        
        // Print all values
        println!(
            " GYRO: X: {:.2}  Y: {:.2}  Z: {:.2}\n
            TEMP: {} °C\n
            HUM: {:?} %\n
            \n 
            ",

            gyro_data.x, gyro_data.y, gyro_data.z, 
            measurement.temperature.as_degrees_celsius(), measurement.humidity.as_percent(),
        );

        FreeRtos.delay_ms(500u32);
    }
}

