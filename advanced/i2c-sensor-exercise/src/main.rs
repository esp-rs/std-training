use anyhow::Result;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use esp_idf_hal::ledc::{config::TimerConfig, Channel, Timer};
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{config::MasterConfig, Master, MasterPins, I2C0},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys::*;
use i2c_sensor_exercise::imc42670p::{IMC42670P, SlaveAddr};

use shtcx::{self, PowerMode, MeasurementDuration};

use shared_bus;

fn main() -> anyhow::Result<()>  {
    link_patches();

    let peripherals = Peripherals::take().unwrap();

    let config = TimerConfig::default().frequency(25.kHz().into());

    let sda = peripherals.pins.gpio4;
    let scl = peripherals.pins.gpio5;

    let i2c = Master::<I2C0, _, _>::new(
        peripherals.i2c0,
        MasterPins { sda, scl },
        <MasterConfig as Default>::default().baudrate(400.kHz().into()),
    )?;

    let bus = shared_bus::BusManagerSimple::new(i2c);

    let proxy_1 =bus.acquire_i2c();
    let proxy_2 =bus.acquire_i2c();

    let mut imu = IMC42670P::new(proxy_1, SlaveAddr::B110_1001)?;
    println!("Sensor init");
    let device_id = imu.read_device_id_register()?;
    println!("Device ID: {}", device_id);
    let conf = imu.read_configuration()?;
    println!("configuration: {}", conf.bits);

    imu.gyro_ln();
    println!("Gyro on");

    let conf = imu.read_configuration()?;
    println!("configuration: {}", conf.bits);





    let mut sht = shtcx::shtc3(proxy_2);
    let device_id = sht.device_identifier().unwrap();
 

    println!("Device ID: {}", device_id);

    
    

    loop {
        let gyro_data =imu.read_gyro()?;
        sht.start_measurement(PowerMode::NormalMode).unwrap();
        FreeRtos.delay_ms(100u32);
        let measurement = sht.get_measurement_result().unwrap(); 
        

        println!(
            " GYRO: X: {:.4}  Y: {:.4}  Z: {:.4}\n
            TEMP: {}\n
            HUM: {:?}\n
            \n 
            ",

            gyro_data.x, gyro_data.y, gyro_data.y, 
            measurement.temperature.as_degrees_celsius(), measurement.humidity.as_percent(),
        );

        FreeRtos.delay_ms(500u32);
    }
}

