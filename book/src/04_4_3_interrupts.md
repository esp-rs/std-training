# Step by Step Guide to the Solution

1. Initialize the LED peripheral and switch the LED on with an arbitrary value just to see that it works.
   ```rust
    let mut led = WS2812RMT::new(peripherals.pins.gpio2, peripherals.rmt.channel0)?;

    led.set_pixel(RGB8::new(20, 0, 20)).unwrap(); // Remove this line after you tried it once
   ```
2. Light up the LED only when the button is pressed. You can do this for now by exchanging the print statement.
   ```rust
   1 => {
        led.set_pixel(arbitrary_color)?;

        },
    _ => {},
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

        unsafe {
            // ...
            match res {
                    1 => {
                        // Generates random rgb values
                        random_light(&mut led);

                    },
                    _ => {},
                };
            }
        }
    // ...
    fn random_light(led: &mut WS2812RMT) {
        let mut color = RGB8::new(0, 0, 0);
        unsafe {
            let r = esp_random() as u8;
            let g = esp_random() as u8;
            let b = esp_random() as u8;

            color = RGB8::new(r, g, b);
        }

        led.set_pixel(color).unwrap();
    }
    ```

