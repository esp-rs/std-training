# Interrupts

The goal of this exercise is to handle a button interrupt, so that the words "button pushed" get logged, if the `BOOT` button is pushed. 
This exercise involves working with C bindings to the esp-idf-sys and other unsafe operations, as well as non-typical rust documentation. In a first step we will go line by line to build this interrupt handler. 

You can find a skeleton code for this exercise in `advanced/button-interrupt/exercise/src/main.rs.`
You can find the solution for this exercise in `advanced/button-interrupt/solution/src/main.rs`

TODO Add points where it's safe to build to rule out basic mistakes.
TODO why are some functions called with some(...), esp!(...) or "normal"?

## Tasks

✅ Configure the button GPIO 9 with a c struct [`gpio_config_t`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/struct.gpio_config_t.html)the following settings (1. in the exercise/src/main.rs):
    - input mode
    - pull up
    - interrupt on positive edge
  
    
    
### Hints!

- Use bit shift to set a `1` in the right position with the constant GPIO_NUM.
- The fields `pull_up_en` and `pull_down_en` can be set with `true/false.into()`, or just `0/1`.
- The other two constants you need to set to true have been imported by `esp_idf_sys`, so use those or just 1. 

✅  Write the configuration into the register with [`unsafe extern "C" fn gpio_config`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/fn.gpio_config.html) (2. in the exercise/src/main.rs). This needs to happen in the unsafe block. To make these FFI calls we can use the macro `esp!($Cfunktion)`.


✅  Write a generic GPIO interrupt handler with [`unsafe extern "C" fn gpio_install_isr_service`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/fn.gpio_install_isr_service.html) (3. in the exercise/src/main.rs). This function takes `ESP_INTR_FLAG_IRAM` as argument (imported by `esp_idf_sys`).


✅  Create the event queue using [`pub unsafe extern "C" fn xQueueGenericCreate`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/fn.xQueueGenericCreate.html) (4. in the exercise/src/main.rs). This lets us safely pass events from an interrupt routine to our main thread.

```rust
EVENT_QUEUE = Some(xQueueGenericCreate(QUEUE_SIZE, ITEM_SIZE, QUEUE_TYPE_BASE));
```


🔎  The `static mut EVENT_QUEUE: Option<QueueHandle_t> = None` (5. in the exercise/src/main.rs) holds the queue handle we are going to get from `xQueueGenericCreate`. This is a number that uniquely identifies one particular queue, as opposed to any of the other queues in our program. The queue storage itself if managed by the Operating System. This variable is declared on line 13.


Look at the function that will be called whenever there is a GPIO interrupt on our button pin (6. in the exercise/src/main.rs). We put this function in a special block of RAM (`iram0`), so it will still be available even if the external flash is busy doing something else (like filesystem work). The function needs to get the queue handle from `EVENT_QUEUE` and call the `xQueueGiveFromISR` function with a `std::ptr::null_mut()` - the objects in our queue are of size zero, so we don't actually need a 'thing' to put on the queue. Instead, the act of pushing a 'nothing' is enough to wake up the other end!

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

✅ Pass the function `button_interrupt` (6.) to the generic GPIO interrupt handler (7. in the exercise/src/main.rs) we registered earlier, along with the number of the GPIO pin that should cause this function to be executed.

```rust
esp!(gpio_isr_handler_add(
    GPIO_NUM,
    Some(button_interrupt),
    std::ptr::null_mut()
))?;
```

✅  Inside a loop, wait until the queue has an item in it (8. in the exercise/src/main.rs). That is, until the `button_interrupt` function puts something in the queue.

```rust
let res = xQueueReceive(EVENT_QUEUE.unwrap(), ptr::null_mut(), QUEUE_WAIT_TICKS);
```
✅  Match the `res` with a `println!()` to the screen indicating the button was pushed. (9. in the exercise/src/main.rs).



