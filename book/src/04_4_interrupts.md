# Interrupts

The goal of this exercise is to handle a button interrupt, so that the words "button pushed" get logged, if the `BOOT` button is pushed. 
This exercise involves working with C bindings to the esp-idf-sys and other unsafe operations, as well as non-typical rust documentation. In a first step we will go line by line to build this interrupt handler. 

You can find a skeleton code for this exercise in `advanced/button-interrupt/exercise/src/main.rs.`
You can find the solution for this exercise in advanced/button-interrupt/solutuion/src/main.rs.


## Tasks

1. Configure the button (GPIO 9) with a c struct [`gpio_config_t`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/struct.gpio_config_t.html)the following settings:
    - input mode
    - pull up
    - interrupt on positive edge
  
  TODO add the variable names for the settings

2. Write the configuration into the register with [`unsafe extern "C" fn gpio_config`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/fn.gpio_config.html). This needs to happen in the unsafe block. To make these FFI calls we can use the macro `esp!($Cfunktion)`.


3. Install the global interrupt handler with [`unsafe extern "C" fn gpio_install_isr_service`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/fn.gpio_install_isr_service.html). This function needs `const ESP_INTR_FLAG_IRAM` as argument.


4. Create a `static mut` that holds the state of the queue.

```rust
static mut EVENT_QUEUE: Option<QueueHandle_t> = None;
```

5. Create the event queue using [`pub unsafe extern "C" fn xQueueGenericCreate`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/fn.xQueueGenericCreate.html)

```rust
EVENT_QUEUE = Some(xQueueGenericCreate(QUEUE_SIZE, ITEM_SIZE, QUEUE_TYPE_BASE));
```

6. Add a function to the RAM that determines what the interrupt handler does.

```rust
#[link_section = ".iram0.text"]
unsafe extern "C" fn button_interrupt(_: *mut c_void) {
    xQueueGiveFromISR(EVENT_QUEUE.unwrap(), std::ptr::null_mut());
}
```
If the interrupt fires, an event is added to the queue. 
TODO Add explanation
    - what is added to the queue
    - why is it in RAM

7. Add the button GPIO as well as the function that is executed upon the interrupt to the interrupt handler.

```rust
esp!(gpio_isr_handler_add(
    GPIO_NUM,
    Some(button_interrupt),
    std::ptr::null_mut()
))?;
```

8. Inside a loop, get the event item out of the queue.

```rust
let res = xQueueReceive(EVENT_QUEUE.unwrap(), ptr::null_mut(), PORT_MAX_DELAY);
```


## How to call the C functions


- extra: hold button <-> LED on (might be good to not use semaphore xqueue then)