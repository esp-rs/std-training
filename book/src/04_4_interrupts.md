# Interrupts

The goal of this exercise is to handle a button interrupt, if the `BOOT` button is pushed. 
This exercise involves working with C bindings to the esp-idf-sys and other unsafe operations, as well as non-typical rust documentation. In a first step we will go line by line to build this interrupt handler. 

You can find a skeleton code for this exercise in `advanced/button-interrupt/exercise/src/main.rs.`
You can find the solution for this exercise in `advanced/button-interrupt/solution/src/main.rs`

This first part is not memory safe. We chose this route despite this fact, as it makes the general theme of dealing with the interrupt more obvious. We provide a safe variant for you to compare: `advanced/button-interrupt/solution/src/main_safe.rs`

TODO Add points where it's safe to build to rule out basic mistakes.
TODO why are some functions called with some(...), esp!(...) or "normal"?

## Tasks

1. Configure the button (GPIO 9) with a c struct [`gpio_config_t`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/struct.gpio_config_t.html)the following settings:
    - input mode
    - pull up
    - interrupt on positive edge
  
Possible ooptions
 Pins are configured with the `c struct` `gpio_config_t`. The struct has the following fields:

 * `pin_bit_mask`: represents the Pin number, 1  shifted by the number of the pin. 
 * `mode`: sets the mode of the pin, it can have the following settings:
   * `gpio_mode_t_GPIO_MODE_INPUT`
   * `gpio_mode_t_GPIO_MODE_OUTPUT`
   * `gpio_mode_t_GPIO_MODE_DISABLE`
   * `gpio_mode_t_GPIO_MODE_OUTPUT_OD`
   * `gpio_mode_t_GPIO_MODE_INPUT_OUTPUT`
   * `gpio_mode_t_GPIO_MODE_INPUT_OUTPUT_OD`

 They are constants with numbers representing the bit that must be set in the corresponding register. 

 * `pull_up_en`: true.into(), if the GPIO is pulled up,
 * `pull_down_en`: true.into(), if the GPIO is pulled down,
 * `intr_type`: sets the interrupt type, it can have the following settings:
   * `gpio_int_type_t_GPIO_INTR_MAX`
   * `gpio_int_type_t_GPIO_INTR_ANYEDGE`
   * `gpio_int_type_t_GPIO_INTR_DISABLE`
   * `gpio_int_type_t_GPIO_INTR_NEGEDGE`
   * `gpio_int_type_t_GPIO_INTR_POSEDGE`



 TODO Add verbal description of configuration


2. Write the configuration into the register with [`unsafe extern "C" fn gpio_config`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/fn.gpio_config.html). This needs to happen in the unsafe block. To make these FFI calls we can use the macro `esp!($Cfunktion)`.


3. Install a generic GPIO interrupt handler with [`unsafe extern "C" fn gpio_install_isr_service`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/fn.gpio_install_isr_service.html). This function takes `ESP_INTR_FLAG_IRAM` as argument.


4. Create a `static mut` that holds the queue handle we are going to get from `xQueueGenericCreate`. This is a number that uniquely identifies one particular queue, as opposed to any of the other queues in our program. The queue storage itself if managed by the Operating System.

```rust
static mut EVENT_QUEUE: Option<QueueHandle_t> = None;
```

5. Create the event queue using [`pub unsafe extern "C" fn xQueueGenericCreate`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/fn.xQueueGenericCreate.html). This lets us safely pass events from an interrupt routine to our main thread.

```rust
EVENT_QUEUE = Some(xQueueGenericCreate(QUEUE_SIZE, ITEM_SIZE, QUEUE_TYPE_BASE));
```

6. Add a function which that will be called whenever there is a GPIO interrupt on our button pin. We put this function in a special block of RAM (`iram0`), so it will still be available even if the external flash is busy doing something else (like filesystem work). The function needs to get the queue handle from `EVENT_QUEUE` and call the `xQueueGiveFromISR` function with a `std::ptr::null_mut()` - the objects in our queue are of size zero, so we don't actually need a 'thing' to put on the queue. Instead, the act of pushing a 'nothing' is enough to wake up the other end!

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

7. Pass the function we just wrote to the generic GPIO interrupt handler we registered earlier, along with the number of the GPIO pin that should cause this function to be executed.

```rust
esp!(gpio_isr_handler_add(
    GPIO_NUM,
    Some(button_interrupt),
    std::ptr::null_mut()
))?;
```

8. Inside a loop, wait until the queue has an item in it. That is, until the `button_interrupt` function puts something in the queue.

```rust
let res = xQueueReceive(EVENT_QUEUE.unwrap(), ptr::null_mut(), QUEUE_WAIT_TICKS);
```


## Modify

TODO


