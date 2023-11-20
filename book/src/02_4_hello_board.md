# Hello, Board!

You're now ready to do a consistency check!

✅ Connect the USB-C port of the board to your computer and enter the `hardware-check` directory in the workshop repository:

```console
cd intro/hardware-check
```

To test Wi-Fi connectivity, you will have to provide your network name (SSID) and password (PSK). These credentials are stored in a dedicated `cfg.toml` file (which is `.gitignore`d) to prevent accidental disclosure by sharing source code or doing pull requests. An example is provided.

✅ Copy `cfg.toml.example` to `cfg.toml` (in the same directory) and edit it to reflect your actual credentials:

> ⚠️The [5 GHz band isn't supported in ESP32-C3](https://www.espressif.com/en/news/ESP32-C3_Wi-Fi_Certified#:~:text=ESP32%2DC3%20is%20a%20safe,wide%20range%20of%20IoT%20applications), you need to ensure you are using a Wi-Fi with active 2.4 GHz band.

```console
$ cp cfg.toml.example cfg.toml
$ $EDITOR cfg.toml
$ cat cfg.toml

[hardware-check]
wifi_ssid = "Your Wifi name"
wifi_psk = "Your Wifi password"
```


✅ Build, flash and monitor the project:

```console
$ cargo run

Serial port: /dev/SERIAL_DEVICE
Connecting...

Chip type:         ESP32-C3 (revision 3)
(...)
Compiling hardware-check v0.1.0
Finished release [optimized] target(s) in 1.78s

[00:00:45] ########################################     418/418     segment 0x10000

Flashing has completed!
(...)
rst:0x1 (POWERON),boot:0xc (SPI_FAST_FLASH_BOOT)
(...)
(...)
(...)
I (4427) wifi::wifi: Wifi connected!
```

>🔎 If `cargo run` has been successful, you can exit with `ctrl+C`.

> 🔎 `cargo run` is [configured to use `espflash`](https://github.com/esp-rs/std-training/blob/main/intro/hardware-check/.cargo/config.toml#L6) as [custom runner](https://doc.rust-lang.org/cargo/reference/config.html#target). The same output can be achieved via:
> - Using `cargo-espflash`: `cargo espflash flash --release --monitor`
> - Building your project and flashing it with `espflash`: `cargo build --release && espflash target/riscv32imc-esp-espidf/release/hardware-check`
> This modification is applied to all the projects in the training for convenience.

The board LED should turn yellow on startup, and then, depending on whether a Wi-Fi connection could be established, either turn red (error) or blink, alternating green and blue, in case of succeeding. In case of a Wi-Fi error, a diagnostic message will also show up at the bottom, e.g.:

```console
Error: could not connect to Wi-Fi network: ESP_ERR_TIMEOUT
```
> ⚠️ You will get an `ESP_ERR_TIMEOUT` error also in case your network name or password are incorrect, so double-check those.

## Extra Information About Building, Flashing and Monitoring

If you want to try to build without flashing, you can run:

 ```console
 cargo build
 ```
You can also monitor the device without flashing it with the following command:

```console
espflash monitor
```

## Simulation

This project is available for simulation through two methods:
- [Wokwi project](https://wokwi.com/projects/360342886675214337?build-cache=disable)
- Wokwi VS Code extension:
  1. Press F1, select `Wokwi: Select Config File`, and choose `intro/hardware-check/wokwi.toml`.
  2. Build your project.
  3. Press F1 again and select `Wokwi: Start Simulator`.

## Troubleshooting

### Build Errors

```console
error[E0463]: can't find crate for `core`
= note: the `riscv32imc-esp-espidf` target may not be installed
```

You're trying to build with a `stable` Rust - you need to use `nightly`.
this error message is slightly misleading - this target *cannot* be installed. It needs to be built from source, using `build-std`, which is a feature available on nightly only.

---

```console
error: cannot find macro `llvm_asm` in this scope
```

You're using an incompatible version of nightly - configure a suitable one using `rust-toolchain.toml` or  `cargo override`.

---

```console
CMake Error at .../Modules/CMakeDetermineSystem.cmake:129 (message):
```

Your Espressif toolchain installation might be damaged. Delete it and rerun the build to trigger a fresh download:

```console
rm -rf ~/.espressif
```
On Windows, delete `%USERPROFILE%\.espressif` folder.

---

 ```console
Serial port: /dev/tty.usbserial-110
Connecting...

Unable to connect, retrying with extra delay...
Unable to connect, retrying with default delay...
Unable to connect, retrying with extra delay...
Error: espflash::connection_failed

× Error while connecting to device
╰─▶ Failed to connect to the device
help: Ensure that the device is connected and the reset and boot pins are not being held down
```

The board isn't accessible with a USB-C cable. A typical connection error looks like this:


Workarounds:
1. Press and hold boot button on the board, start flash command, release boot button after flashing process starts
2. Use a hub.

[Source](https://georgik.rocks/unable-to-flash-esp32-with-these-usb-c-cables/).
