# Interrupts

The goal of this exercise is to handle the interrupt that fires if the `BOOT` button is pushed. 
This exercise involves working with C bindings to the [esp-idf-sys](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/index.html) and other unsafe operations, as well as non-typical rust documentation. In a first step we will go line by line to build this interrupt handler. 

You can find a skeleton code for this exercise in `advanced/button-interrupt/exercise/src/main.rs.`
You can find the solution for this exercise in `advanced/button-interrupt/solution/src/main.rs`

## A note on `unsafe {}` blocks:

This code contains a lot of `unsafe {}` blocks. As a general rule, `unsafe` does not mean that the contained code is not memory safe, it means, that Rust can't make safety guarantees in this place and that it is in the responsibility of the programmer to ensure memory safety. For example Calling C Bindings is per se unsafe, as Rust can't make any safety guarantees for the underlaying C Code. 

## Tasks

1. Configure the button (GPIO 9) with a c struct [`gpio_config_t`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/struct.gpio_config_t.html)the following settings:
    - input mode
    - pull up
    - interrupt on positive edge
  
The struct has the following fields:

 * `pin_bit_mask`: represents the Pin number, the value 1  shifted by the number of the pin. 
 * `mode`: sets the mode of the pin, it can have the following settings:
   * `gpio_mode_t_GPIO_MODE_INPUT` 
   * `gpio_mode_t_GPIO_MODE_OUTPUT`
   * `gpio_mode_t_GPIO_MODE_DISABLE` // disable gpio
   * `gpio_mode_t_GPIO_MODE_OUTPUT_OD` // open drain output
   * `gpio_mode_t_GPIO_MODE_INPUT_OUTPUT` // input and output
   * `gpio_mode_t_GPIO_MODE_INPUT_OUTPUT_OD` // open drain input and output

 * `pull_up_en`: true.into(), if the GPIO is pulled up,
 * `pull_down_en`: true.into(), if the GPIO is pulled down,
 * `intr_type`: sets the interrupt type, it can have the following settings:
   * `gpio_int_type_t_GPIO_INTR_ANYEDGE` // interrupt at any edge
   * `gpio_int_type_t_GPIO_INTR_DISABLE` // interrupt disabled
   * `gpio_int_type_t_GPIO_INTR_NEGEDGE` // interrupt at negative edge
   * `gpio_int_type_t_GPIO_INTR_POSEDGE` // interrupt at positive edge

They are constants with numbers representing the bit that must be set in the corresponding register. 


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


## Random LED color on pushing a button

✅ Modify the code so the RGB LED light changes to different random color upon each button press. The LED should not go out or change color if the button is not pressed for some time. 


### Solving Help

1. The necessary imports are already made, if you enter `cargo --doc --open` you will get helping documentation regarding the LED.
2. The board has a hardware random number generator. It can be called with `esp_random()`.

### Step by Step Guide to the Solution

1. Initialize the LED peripheral and switch the LED on with an arbitrary value just to see that it works.
   ```rust
   let mut led = WS2812RMT::new()?;
   led.set_pixel(20, 20, 20)?; // remove this line after you tried it once
   ```
2. Light up the LED only when the button is pressed. You can do this for now by exchanging the print statement. 
   ```rust
   1 => {
        led.set_pixel(20, 20, 20)?;
                    
        },
    _ => {},
   ```
3. Create random RGB values by calling `esp_random()`. 
   * This function is unsafe. 
   * It yields u32, so it needs to be cast as u8.

    ```rust
    unsafe {
    //...
    1 => {
        let r = esp_random() as u8;
        let g = esp_random() as u8;
        let b = esp_random() as u8;

        let color = RGB8::new(r, g, b);
        led.set_pixel(r, g, b)?;
                    
        },
    _ => {},
   ```


If you run the code now, the LED should change it's color upon every button press. But the LED is also only on as long until the queue timeout is reached. To avoid this, we need to keep the state of the LED separate from the condition that an event is in the queue. 

4. Create a new function that takes a mutable reference to the LED instance and a `RGB8` value as arguments. Change the color of the LED inside the function. Call the function in the match arm. 

```rust 
    unsafe {
        // ...
        match res {
                1 => {
                    // Generates random rgb values
                    let r = esp_random() as u8;
                    let g = esp_random() as u8;
                    let b = esp_random() as u8;

                    let color = RGB8::new(r, g, b);

                    light(&mut led, color);
                    
                },
                _ => {},
            };
        }
    }
}

fn light(led: &mut WS2812RMT, color: RGB8) {
    led.set_pixel(color).unwrap();
}
```
