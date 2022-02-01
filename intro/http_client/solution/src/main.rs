use core::str;

use bsc::{esp_idf_svc, wifi::wifi};
use embedded_svc::{
    http::{
        client::{Client, Request, RequestWrite, Response},
        Headers, Status,
    },
    io::Read,
};
use esp32_c3_dkc02_bsc as bsc;
use esp_idf_svc::http::client::{self, EspHttpClient};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> anyhow::Result<()> {
    let _wifi = wifi(CONFIG.wifi_ssid, CONFIG.wifi_psk)?;

    // 1. create a new default EspHttpClient
    let mut client = EspHttpClient::new_default()?;

    // 2. open a GET request to http://neverssl.com
    let request = client.get("http://neverssl.com")?;

    // 3. requests *may* send data to the server. Turn the request into a writer, specifying 0 bytes as write length
    // (since we don't send anything - but have to do the writer step anyway)
    //
    // https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/protocols/esp_http_client.html
    // if this were a POST request, you'd set a write length > 0 and then:
    // writer.do_write(&some_buf);
    let writer = request.into_writer(0)?;

    // 4. turn the writer into a response and check its status. Successful http status codes are in the 200..=299 range.
    let response = writer.into_response()?;
    let status = response.status();
    println!("response status: {}", status);
    match status {
        200..=299 => {
            println!("ok!");

            // 5. if the status is OK, read some response data into a buffer (of e.g. size 100) and print the amount of bytes read
            let mut buf = [0u8; 100];
            let size = response.reader().do_read(&mut buf)?;
            println!("read {} bytes:", size);
            // strictly speaking, we should check the response's encoding...

            // 6. try converting the bytes into a Rust (UTF-8) string and print it
            let response_text = str::from_utf8(&buf)?;
            println!("{}\n", response_text);
        }
        _ => {}
    }

    Ok(())
}
