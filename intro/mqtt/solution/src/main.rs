use std::{convert::TryFrom, thread::sleep, time::Duration};

use bsc::{
    led::{RGB8, WS2812RMT},
    temp_sensor::BoardTempSensor,
    wifi::wifi,
};
use embedded_svc::mqtt::client::{
    Client,
    Details::{Complete, InitialChunk, SubsequentChunk},
    Event::Received,
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
use mqtt_messages::{hello_topic, ColorData};

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

    let mqtt_config = MqttClientConfiguration::default();

    let broker_url = if !app_config.mqtt_user.is_empty() {
        format!(
            "mqtt://{}:{}@{}",
            app_config.mqtt_user, app_config.mqtt_pass, app_config.mqtt_host
        )
    } else {
        format!("mqtt://{}", app_config.mqtt_host)
    };

    let mut inflight = vec![];
    let mut client =
        EspMqttClient::new_with_callback(broker_url, &mqtt_config, move |message_event| {
            if let Some(Ok(Received(message))) = message_event {
                process_message(message, &mut inflight, &mut led);
            }
        })?;

    let payload: &[u8] = &[];
    client.publish(hello_topic(UUID), QoS::AtLeastOnce, true, payload)?;

    client.subscribe(mqtt_messages::color_topic(UUID), QoS::AtLeastOnce)?;

    loop {
        sleep(Duration::from_secs(1));
        let temp = temp_sensor.read_owning_peripherals();
        client.publish(
            mqtt_messages::temperature_data_topic(UUID),
            QoS::AtLeastOnce,
            false,
            &temp.to_be_bytes() as &[u8],
        )?;
    }
}

fn process_message(message: EspMqttMessage, inflight: &mut Vec<u8>, led: &mut WS2812RMT) {
    match message.details() {
        Complete(token) => {
            info!("{}", message.topic(token));
            let message_data: &[u8] = &message.data();
            if let Ok(ColorData::BoardLed(color)) = ColorData::try_from(message_data) {
                info!("{}", color);
                if let Err(e) = led.set_pixel(color) {
                    error!("could not set board LED: {:?}", e)
                };
            }
        }
        InitialChunk(chunk_info) => {
            let data = message.data();
            inflight.extend(data.iter());
            info!(
                "received start of a partial packet: {}/{} bytes",
                inflight.len(),
                chunk_info.total_data_size
            );
        }
        SubsequentChunk(chunk_data) => {
            let mut complete = false;
            let data = message.data();

            inflight.extend(message.data().iter());
            info!(
                "received more partial data: {} bytes (buffer:{}/{})",
                data.len(),
                inflight.len(),
                chunk_data.total_data_size
            );

            if inflight.len() == chunk_data.total_data_size {
                complete = true;
                info!("big packet complete!");
            }

            if complete {
                /* further processing here */
                inflight.clear();
            }
        }
    }
}
