use std::{convert::TryFrom, thread::sleep, time::Duration};

use anyhow::anyhow;
use bsc::{
    esp_idf_svc::{
        log::EspLogger,
        mqtt::client::{EspMqttClient, EspMqttMessage, MqttClientConfiguration},
    },
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
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;
use log::{error, info};
use mqtt_messages::{cmd_topic_fragment, Command, RawCommandData};

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
    let app_config = CONFIG;

    EspLogger::initialize_default();

    let mut low_level_peripherals =
        esp32c3::Peripherals::take().ok_or(anyhow!("could not take Peripherals"))?;

    let temp_sensor = BoardTempSensor::new(&mut low_level_peripherals);

    info!("our UUID is:");
    info!("{}", UUID);

    let mut led = WS2812RMT::new()?;
    led.set_pixel(RGB8::new(0, 0, 0))?;

    let _wifi = wifi(app_config.wifi_ssid, app_config.wifi_psk)?;

    let mqtt_config = MqttClientConfiguration::default();

    let uri = if app_config.mqtt_user != "" {
        format!(
            "mqtt://{}:{}@{}",
            app_config.mqtt_user, app_config.mqtt_pass, app_config.mqtt_host
        )
    } else {
        format!("mqtt://{}", app_config.mqtt_host)
    };

    let mut inflight = vec![];
    let mut client = EspMqttClient::new_with_callback(uri, &mqtt_config, move |message_event| {
        if let Some(Ok(message_event)) = message_event {
            process_message(message_event, &mut inflight, &mut led);
        }
    })?;

    client.subscribe(
        format!("{}#", mqtt_messages::cmd_topic_fragment(UUID)),
        QoS::AtLeastOnce,
    )?; // TODO define QoS

    loop {
        sleep(Duration::from_secs(1));
        let temp = temp_sensor.read(&mut low_level_peripherals.APB_SARADC);

        client.publish(
            mqtt_messages::temperature_data_topic(UUID),
            QoS::AtLeastOnce,
            false,
            &temp.to_be_bytes() as &[u8],
        )?;
    }
}

fn process_message(
    message_event: Event<EspMqttMessage>,
    inflight: &mut Vec<u8>,
    led: &mut WS2812RMT,
) {
    if let Received(message) = message_event {
        match message.details() {
            Complete(token) => {
                let topic = message.topic(token);
                // use `split()` to look for '{UUID}/cmd/' as leading part of `topic`
                // and if it matches, process the remaining part
                if let Some(cmd) = topic.split(&cmd_topic_fragment(UUID)).nth(1) {
                    // try and parse the remaining path and the data sent along as `BoardLed` command
                    let raw = RawCommandData {
                        path: std::borrow::Cow::Borrowed(cmd),
                        data: message.data(),
                    };

                    if let Ok(cmd) = Command::try_from(raw) {
                        match cmd {
                            Command::BoardLed(color) => match led.set_pixel(color) {
                                Err(e) => error!("could not set board LED: {:?}", e),
                                _ => {}
                            },
                        }
                    }
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
}
