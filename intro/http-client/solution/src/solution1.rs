use core::str;

use bsc::wifi::wifi;
use embedded_svc::{
    http::{
        client::{Client, Response, Request, RequestWrite},
       Status,
    }, io::Read,
};

use esp32_c3_dkc02_bsc as bsc;
use esp_idf_svc::http::client::{EspHttpClient};
use esp_idf_sys as _; 

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

    get("http://neverssl.com/")?;

    Ok(())
}

fn get(url: impl AsRef<str>) -> anyhow::Result<()> {
    // 1. Create a new EspHttpClient. (Check documentation) 
    let mut client = EspHttpClient::new_default()?;
   
    // 2. Open a GET request to `url`
    let request = client.get(url)?;

    // 3. Requests *may* send data to the server. Turn the request into a writer, specifying 0 bytes as write length
    // (since we don't send anything - but have to do the writer step anyway)
    // https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/protocols/esp_http_client.html
    // If this were a POST request, you'd set a write length > 0 and then writer.do_write(&some_buf);

    let writer = request.into_writer(0)?;

    // 4. Submit our write request and check the status code of the response. 
    // Successful http status codes are in the 200..=299 range.

    let response = writer.submit()?;
    let status = response.status();
    let mut total_size = 0;

    println!("response code: {}\n", status);

    match status {
        200..=299 => {
            // 5. if the status is OK, read response data chunk by chunk into a buffer and print it until done
            let mut buf = [0_u8;256];
            let mut reader = response.reader();
            loop {
                if let Ok(size) = Read::do_read(&mut reader, &mut buf){
                    if size == 0 { break; }
                    total_size += size;
                    // 6. try converting the bytes into a Rust (UTF-8) string and print it
                    let response_text = str::from_utf8(&buf[..size])?;
                    println!("{}", response_text);
                }
            } 
        }
        _ => anyhow::bail!("unexpected response code: {}", status),
    }
            
    
    Ok(())
}
