# Random LED color on pushing a button

✅ Modify the code so the RGB LED light changes to different random color upon each button press. The LED should not go out or change color if the button is not pressed for some time. 

## Solving Help

1. The necessary imports are already made, if you enter `cargo --doc --open` you will get helping documentation regarding the LED.
2. The board has a hardware random number generator. It can be called with `esp_random()`.

