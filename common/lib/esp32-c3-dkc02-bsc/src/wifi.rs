// based on https://github.com/ivmarkov/rust-esp32-std-demo/blob/main/src/main.rs

use std::sync::Arc;

use anyhow::bail;
use embedded_svc::wifi::{
    self, AuthMethod, ClientConfiguration, ClientConnectionStatus, ClientIpStatus, ClientStatus,
    Wifi as _,
};
use esp_idf_svc::{
    netif::EspNetifStack, nvs::EspDefaultNvs, sysloop::EspSysLoopStack, wifi::EspWifi,
};
use log::info;

#[allow(unused)]
pub struct Wifi {
    esp_wifi: EspWifi,
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
}

pub fn wifi(ssid: &str, psk: &str) -> anyhow::Result<Wifi> {
    let mut auth_method = AuthMethod::WPA2Personal;
    if ssid.len() == 0 {
        anyhow::bail!("missing WiFi name")
    }
    if psk.len() == 0 {
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
        auth_method: auth_method,
        ..Default::default()
    }))?;

    info!("getting Wifi status");

    let wifi::Status(mut client_status, _) = wifi.get_status();

    while ClientStatus::Started(ClientConnectionStatus::Connecting) == client_status {
        info!("WiFi connecting");
        std::thread::sleep(std::time::Duration::from_millis(500));
        wifi::Status(client_status, _) = wifi.get_status();
    }

    while ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Waiting))
        == client_status
    {
        info!("WiFi connected, waiting for IP");
        std::thread::sleep(std::time::Duration::from_millis(500));
        wifi::Status(client_status, _) = wifi.get_status();
    }

    if let ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(
        _client_settings,
    ))) = client_status
    {
        info!("Wifi connected!");
    } else {
        bail!("Unexpected Wifi status: {:?}", client_status);
    }

    let wifi = Wifi {
        esp_wifi: wifi,
        netif_stack,
        sys_loop_stack,
        default_nvs,
    };

    Ok(wifi)
}
