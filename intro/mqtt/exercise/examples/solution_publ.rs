use anyhow::Result;
use embedded_svc::mqtt::client::{Details::Complete, Event::Received, QoS};
use esp32_c3_dkc02_bsc::{
    led::{RGB8, WS2812RMT},
    wifi::wifi,
};
use esp_idf_hal::{
    delay,
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::mqtt::client::{EspMqttClient, EspMqttMessage, MqttClientConfiguration};
use log::{error, info, warn};
use mqtt_messages::{hello_topic, ColorData};
use shtcx::{self, shtc3, PowerMode};
use std::{convert::TryFrom, thread::sleep, time::Duration};
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;

const UUID: &'static str = get_uuid::uuid();

#[toml_cfg::toml_config]
pub struct Config {
    #[default("localhost")]
    mqtt_host: &'static str,
    #[default("")]
    mqtt_user: &'static str,
    #[default("")]
    mqtt_pass: &'static str,
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;

    // Connect to the Wi-Fi network
    let _wifi = wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    )?;

    info!("our UUID is:");
    info!("{}", UUID);

    let pins = peripherals.pins;
    let sda = pins.gpio10;
    let scl = pins.gpio8;
    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config)?;
    let mut temp_sensor = shtc3(i2c);
    let mut delay = delay::Ets;

    let mut led = WS2812RMT::new(pins.gpio2, peripherals.rmt.channel0)?;
    led.set_pixel(RGB8::new(1, 1, 0))?;

    // Client configuration:
    let broker_url = if app_config.mqtt_user != "" {
        format!(
            "mqtt://{}:{}@{}",
            app_config.mqtt_user, app_config.mqtt_pass, app_config.mqtt_host
        )
    } else {
        format!("mqtt://{}", app_config.mqtt_host)
    };

    let mqtt_config = MqttClientConfiguration::default();

    // Your Code:

    // 1. Create a client with default configuration and empty handler
    let mut client = EspMqttClient::new(broker_url, &mqtt_config, move |message_event| {
        // ... your handler code here - leave this empty for now
        // we'll add functionality later in this chapter
    })?;

    // 2. publish an empty hello message
    let payload: &[u8] = &[];
    client.publish(&hello_topic(UUID), QoS::AtLeastOnce, true, payload)?;

    loop {
        sleep(Duration::from_secs(1));
        let temp = temp_sensor
            .measure_temperature(PowerMode::NormalMode, &mut delay)
            .unwrap()
            .as_degrees_celsius();
        // 3. publish CPU temperature
        client.publish(
            &mqtt_messages::temperature_data_topic(UUID),
            QoS::AtLeastOnce,
            false,
            &temp.to_be_bytes() as &[u8],
        )?;
    }
}
