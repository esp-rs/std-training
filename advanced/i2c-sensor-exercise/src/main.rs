use anyhow::Result;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{config::MasterConfig, Master, MasterPins, I2C0},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys::*;
use lis3dh::{accelerometer::Accelerometer, Lis3dh, SlaveAddr};

fn main() -> Result<()> {
    link_patches();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio0;
    let scl = peripherals.pins.gpio1;

    let i2c = Master::<I2C0, _, _>::new(
        peripherals.i2c0,
        MasterPins { sda, scl },
        <MasterConfig as Default>::default().baudrate(400.kHz().into()),
    )?;
    let mut lis3dh = Lis3dh::new_i2c(i2c, SlaveAddr::Default).unwrap();
    lis3dh.enable_temp(true).unwrap();

    loop {
        let accel = lis3dh.accel_norm().unwrap();
        let temp = lis3dh.get_temp_outf().unwrap();
        let hum = 

        println!(
            "ACCEL: X: {:.4}  Y: {:.4}  Z: {:.4}\n
            TEMP: {}\n
            HUM: {}\n
            \n 
            "

            accel.x, accel.y, accel.y, temp, hum
        );

        FreeRtos.delay_ms(100u32);
    }
}

