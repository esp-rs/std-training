# Interrupts

The goal of this exercise is to handle the interrupt that fires if the `BOOT` button is pushed. 
This exercise involves working with C bindings to the [esp-idf-sys](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/index.html) and other unsafe operations, as well as non-typical rust documentation. In a first step we will go line by line to build this interrupt handler. 

You can find a skeleton code for this exercise in `advanced/button-interrupt/exercise/src/main.rs.`
You can find the solution for this exercise in `advanced/button-interrupt/solution/src/main.rs`

## A note on `unsafe {}` blocks:

This code contains a lot of `unsafe {}` blocks. As a general rule, `unsafe` does not mean that the contained code is not memory safe, it means, that Rust can't make safety guarantees in this place and that it is in the responsibility of the programmer to ensure memory safety. For example Calling C Bindings is per se unsafe, as Rust can't make any safety guarantees for the underlaying C Code. 

