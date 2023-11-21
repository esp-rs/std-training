use anyhow::Result;
use esp_idf_svc::hal::{delay::FreeRtos, peripherals::Peripherals};
use log::info;
use rgb_led::WS2812RMT;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    // Onboard RGB LED pin
    // Rust ESP Board gpio2,  ESP32-C3-DevKitC-02 gpio8
    let led = peripherals.pins.gpio2;
    let channel = peripherals.rmt.channel0;
    let mut ws2812 = WS2812RMT::new(led, channel)?;
    loop {
        info!("Red!");
        ws2812.set_pixel(rgb::RGB8::new(255, 0, 0))?;
        FreeRtos::delay_ms(1000);
        info!("Green!");
        ws2812.set_pixel(rgb::RGB8::new(0, 255, 0))?;
        FreeRtos::delay_ms(1000);
        info!("Blue!");
        ws2812.set_pixel(rgb::RGB8::new(0, 0, 255))?;
        FreeRtos::delay_ms(1000);
    }
}
