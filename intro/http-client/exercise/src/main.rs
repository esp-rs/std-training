use core::str;

use bsc::{
    esp_idf_svc::http::client::{EspHttpClient, EspHttpClientConfiguration},
    wifi::wifi,
};
use embedded_svc::{
    http::{
        client::{Client, Request, RequestWrite, Response},
        Headers, Status,
    },
    io::Read,
};
use esp32_c3_dkc02_bsc as bsc;
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

    // TODO your code here
    //get(...)?;

    Ok(())
}

fn get(url: impl AsRef<str>) -> anyhow::Result<()> {
    // 1. create a new EspHttpClient
    // let mut client = EspHttpClient::new(...)?;

    // 2. open a GET request to `url`
    // let request = client.get(url.as_ref())?;

    // 3. requests *may* send data to the server. Turn the request into a writer, specifying 0 bytes as write length
    // (since we don't send anything - but have to do the writer step anyway)
    //
    // https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/protocols/esp_http_client.html
    // if this were a POST request, you'd set a write length > 0 and then writer.do_write(&some_buf);

    // let writer = request...;

    // 4. turn the writer into a response and check its status. Successful http status codes are in the 200..=299 range.

    // let response = writer...;
    // let status = ...;
    // println!("response code: {}\n", status);

    // TODO check if status is in the 2xx range and if yes, print the received data

    Ok(())
}
