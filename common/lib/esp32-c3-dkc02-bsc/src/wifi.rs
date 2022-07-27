// based on https://github.com/ivmarkov/rust-esp32-std-demo/blob/main/src/main.rs

use std::sync::Arc;

use anyhow::bail;
use embedded_svc::wifi::{
    self, AuthMethod, ClientConfiguration, ClientConnectionStatus, ClientStatus, Wifi as _,
};
use esp_idf_svc::{
    netif::EspNetifStack, nvs::EspDefaultNvs, sysloop::EspSysLoopStack, wifi::EspWifi,
};
use log::info;

const SLEEP_DURATION_BETWEEN_TRIES: std::time::Duration = std::time::Duration::from_millis(500);

#[allow(unused)]
pub struct Wifi {
    esp_wifi: EspWifi,
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
}

pub fn wifi(ssid: &str, psk: &str) -> anyhow::Result<Wifi> {
    let mut auth_method = AuthMethod::WPA2Personal; // Todo: add this setting - router dependent
    if ssid.is_empty() {
        anyhow::bail!("missing WiFi name")
    }
    if psk.is_empty() {
        auth_method = AuthMethod::None;
        info!("Wifi password is empty");
    }
    let netif_stack = Arc::new(EspNetifStack::new()?);
    let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    let default_nvs = Arc::new(EspDefaultNvs::new()?);
    let mut wifi = EspWifi::new(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    )?;

    info!("Searching for Wifi network {}", ssid);

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);

    let channel = if let Some(ours) = ours {
        info!(
            "Found configured access point {} on channel {}",
            ssid, ours.channel
        );
        Some(ours.channel)
    } else {
        info!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            ssid
        );
        None
    };

    info!("setting Wifi configuration");
    wifi.set_configuration(&wifi::Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        password: psk.into(),
        channel,
        auth_method,
        ..Default::default()
    }))?;

    info!("getting Wifi status");

    let mut status = wifi.get_status();
    // loop over the status' value until it is either connected or disconnected
    // catch all is to make sure that at least if more states are added, the enum here
    // remains exhaustive even though logically it might not stand up the test of time
    while let wifi::Status(ClientStatus::Started(ref client_connection_status), _) = status {
        match client_connection_status {
            ClientConnectionStatus::Connected(_) => {
                info!("Connected to Wifi");
                break;
            }
            ClientConnectionStatus::Disconnected => {
                bail!("Disconnected from Wifi; Current status is: {:?}", status)
            }
            _ => {
                info!(
                    "Retrying to connect to Wifi; Polling after sleeping {:?}; Current status is: {:?}",
                    SLEEP_DURATION_BETWEEN_TRIES, status
                );
                std::thread::sleep(SLEEP_DURATION_BETWEEN_TRIES);
                status = wifi.get_status();
            }
        }
    }

    let wifi = Wifi {
        esp_wifi: wifi,
        netif_stack,
        sys_loop_stack,
        default_nvs,
    };

    Ok(wifi)
}
