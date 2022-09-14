use core::str;

use bsc::wifi::wifi;
use embedded_svc::{
    http::{
        client::{Client, Request, RequestWrite, Response},
        Status,
    },
    io::Read,
};
use esp_idf_svc::http::client::{EspHttpClient, EspHttpClientConfiguration};

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

    get("http://neverssl.com")?;

    get("https://espressif.com")?;

    Ok(())
}

fn get(url: impl AsRef<str>) -> anyhow::Result<()> {
    // 1. create a new EspHttpClient with SSL certificates enabled
    let mut client = EspHttpClient::new(&EspHttpClientConfiguration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),

        ..Default::default()
    })?;

    // 2. open a GET request to `url`
    let request = client.get(url.as_ref())?;

    // 3. requests *may* send data to the server. Turn the request into a writer, specifying 0 bytes as write length
    // (since we don't send anything - but have to do the writer step anyway)
    //
    // https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/protocols/esp_http_client.html
    // if this were a POST request, you'd set a write length > 0 and then writer.do_write(&some_buf);
    let writer = request.into_writer(0)?;
    // 4. submit our write request and check the status code of the response.
    // Successful http status codes are in the 200..=299 range.
    let mut response = writer.submit()?;
    let status = response.status();
    println!("response code: {}\n", status);
    match status {
        200..=299 => {
            // 5. if the status is OK, read response data chunk by chunk into a buffer and print it until done
            let mut buf = [0u8; 256];
            let mut total_size = 0;
            let mut reader = response.reader();
            loop {
                let size = reader.read(&mut buf)?;
                if size == 0 {
                    break;
                }
                total_size += size;
                // strictly speaking, we should check the response's encoding...

                // 6. try converting the bytes into a Rust (UTF-8) string and print it
                let response_text = str::from_utf8(&buf)?;
                print!("{}", response_text);
            }

            println!("\n\nDone! read {} bytes:", total_size);
        }
        _ => anyhow::bail!("unexpected response code: {}", status),
    }

    Ok(())
}
