# Step by Step Guide to the Solution

1. Initialize the LED peripheral and switch the LED on with an arbitrary value just to see that it works.
   ```rust
{{#include ../../advanced/button-interrupt/examples/solution_led.rs:led}}

    led.set_pixel(RGB8::new(20, 0, 20)).unwrap(); // Remove this line after you tried it once
   ```
2. Light up the LED only when the button is pressed. You can do this for now by adding the following line after the button pressed message:
   ```rust
   led.set_pixel(arbitrary_color)?;
   ```
3. Create random RGB values by calling `esp_random()`.
   * This function is `unsafe`.
   * It yields `u32`, so it needs to be cast as `u8`.

    ```rust
    unsafe {
    //...
    1 => {
        let r = esp_random() as u8;
        let g = esp_random() as u8;
        let b = esp_random() as u8;

        let color = RGB8::new(r, g, b);
        led.set_pixel(color)?;

        },
    _ => {},
   ```

4. **Optional**: If you intend to reuse this code in another place, it makes sense to put it into its own function. This lets us explore, in detail, which parts of the code need to be in `unsafe` blocks.

```rust
// ...
{{#include ../../advanced/button-interrupt/examples/solution_led.rs:loop}}

// ...
{{#include ../../advanced/button-interrupt/examples/solution_led.rs:random_light}}

```

