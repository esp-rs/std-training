use anyhow::Result;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{config::MasterConfig, Master, MasterPins, I2C0},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys::*;

// Dont change this file. Work in the lib.rs and modify it so main.rs runs.

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

    
   let serial_num = ;

   println!("Serial Number: {}", serial_num)
}

```
fn read_serial_num(
        i2c: I2C0,
        register: Register,
    ) -> Result<u8, Error> {
        let mut data = [0];

        self.i2c
            .write_read(self.address, &[register.addr()], &mut data)
            .map_err(Error::Bus)
            .and(Ok(data[0]))
    }

    