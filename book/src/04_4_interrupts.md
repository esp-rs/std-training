# Interrupts

The goal of this exercise is to handle a button interrupt, so that the words "button pushed" get logged, if the button is pushed. 
This exercise involves working with C wrappers and other unsafe operations, as well as non-typical documentation. In a first step we will go line by line to build this interrupt handler, in a second step, you can modify it. 

You can find a skeleton code for this exercise in advanced/button-interrupt.
The goal of this exercise is to log a message upon pressing the `BOOT` button on the board. 



## Tasks

✅ Configure the button (GPIO 9) with a c struct [`gpio_config_t`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/struct.gpio_config_t.html)the following settings:
    - input mode
    - pull up
    - interrupt on positive edge

✅ Write the configuration into the register with [`unsafe extern "C" fn gpio_config`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/fn.gpio_config.html). This needs to happen in the unsafe block. To make these FFI calls we can use the macro `esp!($Cfunktion)`.


✅ Install the interrupt handler with [`unsafe extern "C" fn gpio_install_isr_service`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/fn.gpio_install_isr_service.html). This function needs `const ESP_INTR_FLAG_IRAM` as argument.


✅ Before the `unsafe` block create an asynchronous streaming [channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) for the type `()`. 

✅ Before `fn main` define a `static mut` that holds the status of the interrupt handler. The interrupt handler is the sender so it has the type `Option<mpsc::Sender<()>>`. It's value is `None` for now.

✅ Add a function that determines what the interrupt handler does, once the button is pushed: It sends a message of type `()` over the `tx` ("transmitter") part of the channel. 

```rust
unsafe extern "C" fn button_interrupt(_: *mut c_void) {
    ISR_TX.as_mut().unwrap().send(());
}
```

✅ Install the global GPIO interrupt handler and add the button as individual pin. 

## How to call the C functions


- extra: hold button <-> LED on (might be good to not use semaphore xqueue then)