use anyhow::Result;
use esp_idf_svc::{
    hal::{
        gpio::{InterruptType, PinDriver, Pull},
        peripherals::Peripherals,
        task::notification::Notification,
    },
    sys::esp_random,
};
use rgb_led::{RGB8, WS2812RMT};
use std::num::NonZeroU32;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;
    // ANCHOR: led
    let mut led = WS2812RMT::new(peripherals.pins.gpio2, peripherals.rmt.channel0)?;
    // ANCHOR_END: led

    // Configures the button
    let mut button = PinDriver::input(peripherals.pins.gpio9)?;
    button.set_pull(Pull::Up)?;
    button.set_interrupt_type(InterruptType::PosEdge)?;

    // Configures the notification
    let notification = Notification::new();
    let notifier = notification.notifier();

    // Subscribe and create the callback
    // Safety: make sure the `Notification` object is not dropped while the subscription is active
    unsafe {
        button.subscribe(move || {
            notifier.notify_and_yield(NonZeroU32::new(1).unwrap());
        })?;
    }

    // ANCHOR: loop
    loop {
        // Enable interrupt and wait for new notificaton
        button.enable_interrupt()?;
        notification.wait(esp_idf_svc::hal::delay::BLOCK);
        println!("Button pressed!");
        // Generates random rgb values and sets them in the led.
        random_light(&mut led);
    }
    // ANCHOR_END: loop
}

#[allow(unused)]
// ANCHOR: random_light
fn random_light(led: &mut WS2812RMT) {
    let mut color = RGB8::new(0, 0, 0);
    unsafe {
        let r = esp_random() as u8;
        let g = esp_random() as u8;
        let b = esp_random() as u8;

        color = RGB8::new(r, g, b);
    }

    led.set_pixel(color).unwrap();
}
// ANCHOR_END: random_light
