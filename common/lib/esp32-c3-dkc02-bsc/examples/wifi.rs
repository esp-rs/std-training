use esp32_c3_dkc02_bsc::wifi::wifi;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use anyhow::Result;
use esp_idf_hal::prelude::Peripherals;
use log::info;

fn main () -> Result<()> {

    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    info!("Hello, world!");
    let _wifi = wifi("ssid", "pass", peripherals.modem, sysloop.clone())?;
    Ok(())
}
