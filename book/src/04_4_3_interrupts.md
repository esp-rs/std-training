# Step by Step Guide to the Solution

1. Initialize the LED peripheral and switch the LED on with an arbitrary value just to see that it works.
   ```rust
   let mut led = WS2812RMT::new()?;
   
    let arbitrary_color = RGB8::new(20, 0, 20);
    led.set_pixel(arbitrary_color).unwrap(); // remove this line after you tried it once
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
