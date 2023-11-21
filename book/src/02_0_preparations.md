# Preparations

This chapter contains information about the course material, the required hardware and an installation guide.

## Icons and Formatting We Use

We use Icons to mark different kinds of information in the book:
* ✅ Call for action.
* ⚠️ Warnings, details that require special attention.
* 🔎 Knowledge that dives deeper into a subject but which you aren't required to understand, proceeding.
* 💡 Hints that might help you during the exercises

> Example note: Notes like this one contain helpful information

## Required Hardware

- [Rust ESP Board](https://github.com/esp-rs/esp-rust-board): available on Mouser, Aliexpress. [Full list of vendors](https://github.com/esp-rs/esp-rust-board#where-to-buy).
- USB-C cable suitable to connect the board to your development computer.
- Wi-Fi access point connected to the Internet.

> No additional debugger/probe hardware is required.

## Simulating Projects

Certain projects can be simulated with [Wokwi][wokwi]. Look for indications in the book to identify projects available for simulation. Simulation can be accomplished through two methods:
- Using wokwi.com: Conduct the build process and code editing directly through the browser.
- Using [Wokwi VS Code extension][wokwi-vscode]: Leverage VS Code to edit projects and perform builds. Utilize the Wokwi VS Code extension to simulate the resulting binaries.
    - This approach requires some [installation][wokwi-installation]
    - This approach assumes that the project is built in debug mode
    - This approach allows [debugging the project][wokwi-debug]

[wokwi]: https://wokwi.com/
[wokwi-vscode]: https://docs.wokwi.com/vscode/getting-started
[wokwi-installation]: https://docs.wokwi.com/vscode/getting-started#installation
[wokwi-debug]: https://docs.wokwi.com/vscode/debugging

## Ensuring a Working Setup
<!-- TODO: Update this comments -->

⚠️ If you are participating in a training led by Ferrous Systems, we urge you to do prepare for the workshop by following the instructions in this chapter, at least, one business day in advance to verify you're ready to go by the time it starts. Please, [contact us](https://ferrous-systems.com/contact/) should you encounter any issues or require any kind of support.

⚠️ If you are using a [ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html) a few pins and slave addresses are different, since the board is similar but not the same. This is relevant for the solutions in [advanced/i2c-sensor-reading/](/advanced/i2c-sensor-reading/examples) and [advanced/i2c-driver/](/advanced/i2c-driver/src/), where the pins and slave addresses for the ESP32-C3-DevKitC-02 are commented out.

## Companion Material

- [Official esp-rs book](https://esp-rs.github.io/book/introduction.html)
