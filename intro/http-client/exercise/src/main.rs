use core::str;

use bsc::wifi::wifi;
use embedded_svc::{
    http::{
        client::{Client, Request, RequestWrite, Response},
        Headers, Status,
    },
    io::Read,
};
use esp32_c3_dkc02_bsc as bsc;
use esp_idf_svc::http::client::{EspHttpClient, EspHttpClientConfiguration};
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
    // 1. Create a new EspHttpClient. (Check documentation) 
   

    // 2. Open a GET request to `url`
  

    // 3. Requests *may* send data to the server. Turn the request into a writer, specifying 0 bytes as write length
    // (since we don't send anything - but have to do the writer step anyway)
    //
    // https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/protocols/esp_http_client.html
    // If this were a POST request, you'd set a write length > 0 and then writer.do_write(&some_buf);

    // let writer = request...;

    // 4. Rurn the writer into a response and check its status. Successful http status codes are in the 200..=299 range.

    // let response = writer...;
    // let status = ...;
    // println!("response code: {}\n", status);

    // 5. If the status is OK, read response data chunk by chunk into a buffer and print it until done.
    // 6. Try converting the bytes into a Rust (UTF-8) string and print it.
    

    Ok(())
}
