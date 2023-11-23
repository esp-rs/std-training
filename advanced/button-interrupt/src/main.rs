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
    const GPIO_NUM: i32 = 9;

    // 1. Configure the button using PinDriver
    // let mut button = PinDriver...

    // 2. Instantiate a new notification and notifier

    unsafe {
        // 3. Create a subscription and its callback function that notifies and yields.
    }

    loop {
        unsafe {
            // 4. Enable the interrupt for the button
            // 5. Wait for notification using `esp_idf_svc::hal::delay::BLOCK`
            // 6. Print a "button pressed" message
        }
    }
}
