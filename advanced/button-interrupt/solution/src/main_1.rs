// reference:
// https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/freertos.html

use std::sync::mpsc;

// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported (`self as _`)
use esp_idf_sys::{
    self as _, c_types::c_void, esp, gpio_config, gpio_config_t, gpio_install_isr_service,
    gpio_int_type_t_GPIO_INTR_POSEDGE, gpio_isr_handler_add, gpio_mode_t_GPIO_MODE_INPUT,
};

// this static mut holds the state of the interrupt handler
static mut ISR_TX: Option<mpsc::Sender<()>> = None;

// TODO place code in ram using IRAM linker feature
// https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-guides/linker-script-generation.html
// uncommenting this "works", TODO test/verify

// button_interrupt sends a message (of type ()) over the tx ("transmitter") part of the channel. 
#[link_section = ".iram0.text"]
unsafe extern "C" fn button_interrupt(_: *mut c_void) {
    ISR_TX.as_mut().unwrap().send(());
}

fn main() -> anyhow::Result<()> {
    const GPIO_NUM: i32 = 9;

    // configures the button
    let io_conf = gpio_config_t {
        pin_bit_mask: 1 << GPIO_NUM,
        mode: gpio_mode_t_GPIO_MODE_INPUT,
        pull_up_en: true.into(),
        pull_down_en: false.into(),
        intr_type: gpio_int_type_t_GPIO_INTR_POSEDGE, // positive edge trigger = button down
    };

    // simple streaming channel for messages of the type ()
    let (tx, rx) = mpsc::channel::<()>();

    // 
    unsafe {
        // this moves tx into the interrupt handler
        ISR_TX = Some(tx);
        esp!(gpio_config(&io_conf))?;

        // Flag used to allocate the interrupt
        const ESP_INTR_FLAG_IRAM: i32 = 1 << 10;

        // This installs a global gpio interrupt handler at run time
        esp!(gpio_install_isr_service(ESP_INTR_FLAG_IRAM))?;
        // adds a dedicated pin, along with the 
        esp!(gpio_isr_handler_add(
            GPIO_NUM,
            Some(button_interrupt),
            std::ptr::null_mut()
        ))?;
    }

    // the loop waits until it gets a message through the rx ("receiver") part of the channel
    loop {
        match rx.recv() {
            Ok(_) => println!("button pressed!"),
            Err(_) => unreachable!("Err"),
        }
    }
}
