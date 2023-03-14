// Reference: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/freertos.html
use anyhow::Result;
use esp_idf_sys::{
    esp, esp_random, gpio_config, gpio_config_t, gpio_install_isr_service,
    gpio_int_type_t_GPIO_INTR_POSEDGE, gpio_isr_handler_add, gpio_mode_t_GPIO_MODE_INPUT,
    xQueueGenericCreate, xQueueGiveFromISR, xQueueReceive, QueueHandle_t, ESP_INTR_FLAG_IRAM,
};
use std::ptr;

// These imports are needed for part 2.
use rgb_led::{RGB8, WS2812RMT};

// 4. Create a `static mut` that holds the queue handle.
static mut EVENT_QUEUE: Option<QueueHandle_t> = None;

// 6. Define what the interrupt handler does, once the button is pushed. Button_interrupt sends a message into the queue.
#[link_section = ".iram0.text"]
unsafe extern "C" fn button_interrupt(_: *mut core::ffi::c_void) {
    xQueueGiveFromISR(EVENT_QUEUE.unwrap(), std::ptr::null_mut());
}

fn main() -> Result<()> {
    const GPIO_NUM: i32 = 9;

    // 1. Add GPIO configuration C struct
    // let io_conf = gpio_config_t {
    //     ...
    // };

    unsafe {
        // 2. Write the GPIO configuration into the register
        // esp!(...)?;

        // 3. Install the global GPIO interrupt handler
        // esp!(...)?;

        // Queue configurations
        const QUEUE_TYPE_BASE: u8 = 0;
        const ITEM_SIZE: u32 = 0;
        const QUEUE_SIZE: u32 = 1;

        // 5. Create an event queue
        // EVENT_QUEUE = Some(...);

        // 7. Add the button GPIO and the function to the interrupt handler
        // esp!(...)?;
    }

    // The loop in main waits until it gets a message through the rx ("receiver") part of the channel
    loop {
        unsafe {
            // Maximum delay
            const QUEUE_WAIT_TICKS: u32 = 1000;;

            // 8. Receive the event from the queue.
            // let res = ...;

            // 9. Handle the value of res.
            // ...
        }
    }
}
