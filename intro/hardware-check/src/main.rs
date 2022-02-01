use std::{thread::sleep, time::Duration};

use bsc::{
    led::{RGB8, WS2812RMT},
    wifi,
};
use config::CONFIG;
use esp32_c3_dkc02_bsc as bsc;
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;
use log::info;

mod config;

fn main() -> anyhow::Result<()> {
    info!("Hello, world!");

    let mut led = WS2812RMT::new()?;
    led.set_pixel(RGB8::new(50, 50, 0))?;

    let app_config = CONFIG;

    let wifi = Box::new(wifi::wifi(app_config.wifi_ssid, app_config.wifi_psk));
    match *wifi {
        Ok(_) => led.set_pixel(RGB8::new(0, 50, 0))?,
        Err(err) => {
            led.set_pixel(RGB8::new(50, 0, 0))?;
            anyhow::bail!("could not connect to Wi-Fi network: {:?}", err)
        }
    }

    let mut odd = false;
    loop {
        sleep(Duration::from_secs(1));
        match odd {
            true => led.set_pixel(RGB8::new(0, 50, 0))?,
            false => led.set_pixel(RGB8::new(0, 0, 50))?,
        }
        odd = !odd;
    }
}
