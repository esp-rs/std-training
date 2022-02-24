// reference:
// https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/freertos.html

use std::{ffi::CString, ptr, sync::mpsc};

// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported (`self as _`)
use esp_idf_sys::{
    self as _, c_types::c_void, esp, gpio_config, gpio_config_t, gpio_install_isr_service,
    gpio_int_type_t_GPIO_INTR_POSEDGE, gpio_isr_handler_add, gpio_mode_t_GPIO_MODE_INPUT,
    xQueueGenericCreate, xQueueGiveFromISR, xQueueReceive, xTaskCreatePinnedToCore, QueueHandle_t,
};

static mut EVENT_QUEUE: Option<QueueHandle_t> = None;
static mut THING: u32 = 0;
const PORT_MAX_DELAY: u32 = 0xffffffff;

// TODO place code in ram using IRAM linker feature
// https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-guides/linker-script-generation.html
// uncommenting this "works", TODO test/verify
#[link_section = ".iram0.text"]
unsafe extern "C" fn button_interrupt(_: *mut c_void) {
    xQueueGiveFromISR(EVENT_QUEUE.unwrap(), std::ptr::null_mut());
}

fn main() -> anyhow::Result<()> {
    const GPIO_NUM: i32 = 9;

    let io_conf = gpio_config_t {
        pin_bit_mask: 1 << GPIO_NUM,
        mode: gpio_mode_t_GPIO_MODE_INPUT,
        pull_up_en: true.into(),
        pull_down_en: false.into(),
        intr_type: gpio_int_type_t_GPIO_INTR_POSEDGE, // positive edge trigger = button down
    };

    const QUEUE_TYPE_BASE: u8 = 0;
    const ITEM_SIZE: u32 = 0; // we're not posting any actual data, just notifying
    const QUEUE_SIZE: u32 = 2; // 1 might be enough?

    unsafe {
        EVENT_QUEUE = Some(xQueueGenericCreate(QUEUE_SIZE, ITEM_SIZE, QUEUE_TYPE_BASE));

        esp!(gpio_config(&io_conf))?;

        const ESP_INTR_FLAG_IRAM: i32 = 1 << 10; // ISR will be executed even when caches are disabled
        esp!(gpio_install_isr_service(ESP_INTR_FLAG_IRAM))?;
        esp!(gpio_isr_handler_add(
            GPIO_NUM,
            Some(button_interrupt),
            std::ptr::null_mut()
        ))?;
    }

    loop {
        unsafe {
            let res = xQueueReceive(EVENT_QUEUE.unwrap(), ptr::null_mut(), PORT_MAX_DELAY);
            match res {
                _ => println!("button pressed!"),
                0 => {}
            };
        }
    }
}
