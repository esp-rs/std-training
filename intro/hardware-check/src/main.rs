//! # Hardware Check
//!
//! This `libstd` program is for the ESP32-C3-DevKitC-02 board.

use anyhow::{bail, Result};
use esp32_c3_dkc02_bsc::{
    led::{RGB8, WS2812RMT},
    wifi::wifi,
};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use log::info;
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;

/// This configuration is picked up at compile time by `build.rs` from the
/// file `cfg.toml`.
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

/// Entry point to our application.
///
/// It sets up a Wi-Fi connection to the Access Point given in the
/// configuration, then blinks the RGB LED green/blue.
///
/// If the LED goes solid red, then it was unable to connect to your Wi-Fi
/// network.
fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    info!("Hello, world!");

    // Start the LED off yellow
    let mut led = WS2812RMT::new(peripherals.pins.gpio8, peripherals.rmt.channel0)?;
    led.set_pixel(RGB8::new(50, 50, 0))?;

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;

    // Connect to the Wi-Fi network
    let _wifi = match wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => inner,
        Err(err) => {
            // Red!
            led.set_pixel(RGB8::new(50, 0, 0))?;
            bail!("could not connect to Wi-Fi network: {:?}", err)
        }
    };

    loop {
        // Blue!
        led.set_pixel(RGB8::new(0, 0, 50))?;
        // Wait...
        std::thread::sleep(std::time::Duration::from_secs(1));
        info!("Hello, world!");

        // Green!
        led.set_pixel(RGB8::new(0, 50, 0))?;
        // Wait...
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
