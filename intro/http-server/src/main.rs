use core::str;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use bsc::{temp_sensor::BoardTempSensor, wifi::wifi};
use embedded_svc::{
    http::{
        server::{registry::Registry, Response},
        Method,
    },
    io::Write,
};
use esp32_c3_dkc02_bsc as bsc;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let _wifi = wifi(CONFIG.wifi_ssid, CONFIG.wifi_psk)?;

    let mut temp_sensor = BoardTempSensor::new_taking_peripherals();

    // TODO your code here:
    // let server_config = ...;
    // let mut server = EspHttpServer::new(...)?;

    // server.set_inline_handler("/", Method::Get, |request, response| {
    // TODO your code here:
    // ...
    //})?;

    // TODO this is not true until you actually create one
    println!("server awaiting connection");

    // prevent program from exiting
    loop {
        let current_temperature = temp_sensor.read_owning_peripherals();
        println!("board temperature: {:.2}", current_temperature);
        sleep(Duration::from_millis(1000));
    }
}

fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}

fn index_html() -> String {
    templated("Hello from mcu!")
}

fn temperature(val: f32) -> String {
    templated(format!("chip temperature: {:.2}°C", val))
}
