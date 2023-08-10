# Random LED Color on Pushing a Button

✅ Modify the code so the RGB LED light changes to a different random color upon each button press. The LED shouldn't go out or change color if the button isn't pressed for some time.

Continue by adding to your previous solution or the code from `advanced/button-interrupt/src/main.rs`.

You can find the solution for this exercise in `advanced/button-interrupt/examples/solution.rs`. You can run it with the following command:

```console
cargo run --example solution_led
```

## 💡 Solving Help

* The necessary imports are already made, if you enter `cargo --doc --open` you will get helpful documentation regarding the LED.
* The LED's part number is WS2812RMT.
* It's a programmable RGB LED. This means there aren't single pins to set for red, green and blue, but we need to instantiate it to be able to send `RGB8` type values to it with a method.
* The board has a hardware random number generator. It can be called with `esp_random()`.
* Calling functions from the `esp-idf-sys` is unsafe in Rust terms and requires an `unsafe()` block. You can assume that these functions are safe to use, so no other measures are required.

