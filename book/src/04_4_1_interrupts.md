# Building the Interrupt Handler

The goal of this exercise is to handle the interrupt that fires if the `BOOT` button is pushed.

You can find a skeleton code for this exercise in `advanced/button-interrupt/src/main.rs`.

You can find the solution for this exercise in `advanced/button-interrupt/examples/solution.rs`. You can run it with the following command:

```console
cargo run --example solution
```
## ✅ Tasks

1. Configure the [BOOT button](https://github.com/esp-rs/esp-rust-board#ios) (GPIO9), using the `PinDriver` struct with the following settings:
    - Input mode
    - Pull up
    - Interrupt on positive edge
2. Instantiate a new notification and notifier
    - See `hal::task::notification` documentation
3. In an `unsafe` block, create a subscription and its callback function.
    - See `PinDriver::subscribe` and `task::notify_and_yield`
    - The reasons for being `unsafe` are:
      - The callback function will run in the [ISR (Interrupt Service Routine)](https://en.wikipedia.org/wiki/Interrupt_handler), so we should avoid calling any blocking functions on it, this includes STD, `libc` or FreeRTOS APIs (except for a few allowed ones).
      - Callback closure is capturing its environment and you can use static variables inserted onto it. Captured variables need to outlive the subscription. You can also, use non-static variables, but that requires extra caution, see `esp_idf_hal::gpio::PinDriver::subscribe_nonstatic` documentation for more details.
4. In the loop, enable the interrupt, and wait for the notification
    - The interruption should be enabled after each received notification, from a non-ISR context
    - `esp_idf_svc::hal::delay::BLOCK` can be used for waiting
5.  Run the program, push the `BOOT` button, and see how it works!

🔎 In this exercise we are using notifications, which only give the latest value, so if the interrupt is triggered
multiple times before the value of the notification is read, you will only be able to read the latest one. Queues,
on the other hand, allow receiving multiple values. See `esp_idf_hal::task::queue::Queue` for more details.

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
