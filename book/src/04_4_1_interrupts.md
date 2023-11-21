# Building the Interrupt Handler

The goal of this exercise is to handle the interrupt that fires if the `BOOT` button is pushed.
This exercise involves working with C bindings to the ESP-IDF and other unsafe operations, as well as non-typical Rust documentation. In a first step we will go line by line to build this interrupt handler.

You can find a skeleton code for this exercise in `advanced/button-interrupt/src/main.rs`.

You can find the solution for this exercise in `advanced/button-interrupt/examples/solution.rs`. You can run it with the following command:

```console
cargo run --example solution
```
## ✅ Tasks

1. Configure the [BOOT button](https://github.com/esp-rs/esp-rust-board#ios) (GPIO9) with a C struct [`gpio_config_t`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/struct.gpio_config_t.html) and the following settings:
    - Input mode
    - Pull up
    - Interrupt on positive edge

The struct has the following fields:

 * `pin_bit_mask`: Represents the Pin number, the value 1  shifted by the number of the pin.
 * `mode`: Sets the mode of the pin, it can have the following settings:
   * `gpio_mode_t_GPIO_MODE_INPUT`
   * `gpio_mode_t_GPIO_MODE_OUTPUT`
   * `gpio_mode_t_GPIO_MODE_DISABLE` // Disable GPIO
   * `gpio_mode_t_GPIO_MODE_OUTPUT_OD` // Open drain output
   * `gpio_mode_t_GPIO_MODE_INPUT_OUTPUT` // Input and output
   * `gpio_mode_t_GPIO_MODE_INPUT_OUTPUT_OD` // Open drain input and output
 * `pull_up_en`: `true.into()`, if the GPIO is pulled up,
 * `pull_down_en`: `true.into()`, if the GPIO is pulled down,
 * `intr_type`: Sets the interrupt type, it can have the following settings:
   * `gpio_int_type_t_GPIO_INTR_ANYEDGE` // Interrupt at any edge
   * `gpio_int_type_t_GPIO_INTR_DISABLE` // Interrupt disabled
   * `gpio_int_type_t_GPIO_INTR_NEGEDGE` // Interrupt at negative edge
   * `gpio_int_type_t_GPIO_INTR_POSEDGE` // Interrupt at positive edge

They are constants with numbers representing the bit that must be set in the corresponding register.

2. Write the configuration into the register with [`unsafe extern "C" fn gpio_config`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/fn.gpio_config.html). This needs to happen in the unsafe block. To make these FFI calls, we can use the macro `esp!($Cfunktion)`.

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

9. Handle the value of `res`, so that "Button pushed!" is logged, if the button is pushed.

10. Run the program and push the `BOOT` button, so see how it works!

## Simulation

This project is available for simulation through two methods:
- Wokwi projects
  - [Exercise](https://wokwi.com/projects/360623288920412161?build-cache=disable)
  - [Solution](https://wokwi.com/projects/333374799393849940?build-cache=disable)
    - The Solution project contains solution for [Random LED Color on pushinig a Button](./04_4_2_interrupts.md)
- Wokwi files are also present in the project folder to simulate it with Wokwi VS Code extension:
   1. Press F1, select `Wokwi: Select Config File` and choose `advanced/button-interrupt/wokwi.toml`
      - Edit the `wokwi.toml` file to select between exercise and solution simulation
   2. Build you project
   3. Press F1 again and select `Wokwi: Start Simulator`
