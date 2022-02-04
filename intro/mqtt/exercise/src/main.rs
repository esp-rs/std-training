use std::{borrow::Cow, convert::TryFrom, thread::sleep, time::Duration};

use bsc::{
    led::{RGB8, WS2812RMT},
    temp_sensor::BoardTempSensor,
    wifi::wifi,
};
use embedded_svc::mqtt::client::{
    Client,
    Details::{Complete, InitialChunk, SubsequentChunk},
    Event::{self, Received},
    Message, Publish, QoS,
};
use esp32_c3_dkc02_bsc as bsc;
use esp_idf_svc::{
    log::EspLogger,
    mqtt::client::{EspMqttClient, EspMqttMessage, MqttClientConfiguration},
};
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;
use log::{error, info};
use mqtt_messages::{cmd_topic_fragment, hello_topic, Command, RawCommandData};

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

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    EspLogger::initialize_default();

    let app_config = CONFIG;

    info!("our UUID is:");
    info!("{}", UUID);

    let mut temp_sensor = BoardTempSensor::new_taking_peripherals();

    let mut led = WS2812RMT::new()?;
    led.set_pixel(RGB8::new(1, 1, 0))?;

    let _wifi = wifi(app_config.wifi_ssid, app_config.wifi_psk)?;

    let broker_url = if app_config.mqtt_user != "" {
        format!(
            "mqtt://{}:{}@{}",
            app_config.mqtt_user, app_config.mqtt_pass, app_config.mqtt_host
        )
    } else {
        format!("mqtt://{}", app_config.mqtt_host)
    };

    // 1. create client with default configuration and empty handler
    // let mut client = EspMqttClient::new_with_callback( ... )?;

    // 2. publish an empty hello message
    let payload: &[u8] = &[];
    // client.publish(hello_topic(UUID), QoS::AtLeastOnce, true, payload)?;

    // 3. subscribe to *all* commands
    // client.subscribe( ... )?;

    loop {
        sleep(Duration::from_secs(1));
        let temp = temp_sensor.read_owning_peripherals();

        // 4. publish CPU temperature
        // client.publish( ... )?;
    }

    // 5. extend your client according to the remaining exercises.
}
