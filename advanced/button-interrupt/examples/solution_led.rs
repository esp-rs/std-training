// reference:
// https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/freertos.html

use anyhow::Result;
use esp32_c3_dkc02_bsc::led::{RGB8, WS2812RMT};
use esp_idf_sys::{
    c_types::c_void, esp, esp_random, gpio_config, gpio_config_t, gpio_install_isr_service,
    gpio_int_type_t_GPIO_INTR_POSEDGE, gpio_isr_handler_add, gpio_mode_t_GPIO_MODE_INPUT,
    xQueueGenericCreate, xQueueGiveFromISR, xQueueReceive, QueueHandle_t, ESP_INTR_FLAG_IRAM,
};
use std::ptr;

// This `static mut` holds the queue handle we are going to get from `xQueueGenericCreate`.
// This is unsafe, but we are careful not to enable our GPIO interrupt handler until after this value has been initialised, and then never modify it again
static mut EVENT_QUEUE: Option<QueueHandle_t> = None;

#[link_section = ".iram0.text"]
unsafe extern "C" fn button_interrupt(_: *mut c_void) {
    xQueueGiveFromISR(EVENT_QUEUE.unwrap(), std::ptr::null_mut());
}

fn main() -> Result<()> {
    let mut led = WS2812RMT::new()?;
    const GPIO_NUM: i32 = 9;

    // Configures the button
    let io_conf = gpio_config_t {
        pin_bit_mask: 1 << GPIO_NUM,
        mode: gpio_mode_t_GPIO_MODE_INPUT,
        pull_up_en: true.into(),
        pull_down_en: false.into(),
        intr_type: gpio_int_type_t_GPIO_INTR_POSEDGE, // positive edge trigger = button down
    };

    // Queue configurations
    const QUEUE_TYPE_BASE: u8 = 0;
    const ITEM_SIZE: u32 = 0; // we're not posting any actual data, just notifying
    const QUEUE_SIZE: u32 = 1;

    unsafe {
        // Writes the button configuration to the registers
        esp!(gpio_config(&io_conf))?;

        // Installs the generic GPIO interrupt handler
        esp!(gpio_install_isr_service(ESP_INTR_FLAG_IRAM as i32))?;

        // Instantiates the event queue
        EVENT_QUEUE = Some(xQueueGenericCreate(QUEUE_SIZE, ITEM_SIZE, QUEUE_TYPE_BASE));

        // Registers our function with the generic GPIO interrupt handler we installed earlier.
        esp!(gpio_isr_handler_add(
            GPIO_NUM,
            Some(button_interrupt),
            std::ptr::null_mut()
        ))?;
    }

    // Reads the queue in a loop.
    loop {
        unsafe {
            // maximum delay
            const QUEUE_WAIT_TICKS: u32 = 1000;

            // Reads the event item out of the queue
            let res = xQueueReceive(EVENT_QUEUE.unwrap(), ptr::null_mut(), QUEUE_WAIT_TICKS);

            // If the event has the value 0, nothing happens. if it has a different value, the button was pressed.
            // If the button was pressed, a function that changes the state of the LED is called.

            match res {
                1 => {
                    // Generates random rgb values and sets them in the led.
                    random_light(&mut led);
                }
                _ => {}
            };
        }
    }
}

#[allow(unused)]
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
