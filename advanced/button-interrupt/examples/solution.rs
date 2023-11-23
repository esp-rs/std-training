use anyhow::Result;
use esp_idf_svc::hal::{
    gpio::{InterruptType, PinDriver, Pull},
    peripherals::Peripherals,
    task::notification::Notification,
};
use std::num::NonZeroU32;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;
    // Configures the button
    let mut button = PinDriver::input(peripherals.pins.gpio9)?;
    button.set_pull(Pull::Up)?;
    button.set_interrupt_type(InterruptType::PosEdge)?;

    // Configures the notification
    let notification = Notification::new();
    let notifier = notification.notifier();

    // Safety: make sure the `Notification` object is not dropped while the subscription is active
    unsafe {
        button.subscribe(move || {
            notifier.notify_and_yield(NonZeroU32::new(1).unwrap());
        })?;
    }

    loop {
        // enable_interrupt should also be called after each received notification from non-ISR context
        button.enable_interrupt()?;
        notification.wait(esp_idf_svc::hal::delay::BLOCK);
        println!("Button pressed!");
    }
}
