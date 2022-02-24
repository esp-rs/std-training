// reference:
// https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/freertos.html

use std::sync::mpsc;

// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported (`self as _`)
use esp_idf_sys::{
    self as _, c_types::c_void, esp, gpio_config, gpio_config_t, gpio_install_isr_service,
    gpio_int_type_t_GPIO_INTR_POSEDGE, gpio_isr_handler_add, gpio_mode_t_GPIO_MODE_INPUT,
};

// 5. Define a static mute that holds the status of the interrupt handler
static mut ISR_TX: Option<mpsc::Sender<()>> = None;


// 6. define what the interrupt handler does, once the button is pushed. button_interrupt sends a message (of type ()) over the tx ("transmitter") part of the channel. 
unsafe extern "C" fn button_interrupt(_: *mut c_void) {
    ISR_TX.as_mut().unwrap().send(());
}

fn main() -> anyhow::Result<()> {
    const GPIO_NUM: i32 = 9;

    // 1. Add GPIO configuration c struct
    // let io_conf = gpio_config_t {
    //     ...
    // };

    // 4. Add the simple streaming channel for messages of the type ()
    // let (tx, rx) = ...

    // 
    unsafe {

        // 2. write the GPIO configuration into the register
        // esp!(...)?;

        // Flag used to allocate the interrupt
        const ESP_INTR_FLAG_IRAM: i32 = 1 << 10;

        // 3. Install the global GPIO interrupt handler
        // esp!(...)?;
        
        
        // 7. Add our GPIO to the interrupt handler
        esp!(gpio_isr_handler_add(
            GPIO_NUM,
            Some(button_interrupt),
            std::ptr::null_mut()
        ))?;

        // 8. Moves tx into the interrupt handler
        ISR_TX = Some(tx);
    }

    // the loop in main waits until it gets a message through the rx ("receiver") part of the channel
    loop {
        match rx.recv() {
            Ok(_) => println!("button pressed!"),
            Err(_) => unreachable!(),
        }
    }
}
