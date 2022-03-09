# Hello, board!

You're now ready to do a consistency check.

✅ Connect the USB-C port of the board to your computer and enter the hardware check directory in the workshop repository:

```console
espressif-trainings$ cd intro/hardware-check
```

To test Wi-Fi connectivity, you will have to provide your network name (SSID) and password (PSK). These credentials are stored in a dedicated `cfg.toml` file (which is `.gitignore`d) to prevent accidental disclosure by sharing source code or doing pull requests. An example is provided. 

✅ Copy `cfg.toml.example` to `cfg.toml` (in the same directory) and edit it to reflect your actual credentials:

❗️The 5GHz band is not supported according to [ESP32-C3 documentation](https://www.espressif.com/en/news/ESP32-C3_Wi-Fi_Certified#:~:text=ESP32%2DC3%20is%20a%20safe,wide%20range%20of%20IoT%20applications), you need to ensure you are using a WiFi with active 2.4GHz band.

```console
$ cp cfg.toml.example cfg.toml
$ $EDITOR cfg.toml
$ cat cfg.toml

[hardware-check]
wifi_ssid = "Your Wifi name"
wifi_psk = "Your Wifi password" 
```


✅ build, run and monitor the project, substituting the actual serial device name for `/dev/SERIAL_DEVICE`:
```console
$ cargo espflash --release --monitor /dev/SERIAL_DEVICE

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
I (4427) bsc::wifi: Wifi connected!
```

The board LED should turn yellow on startup, and then, depending on whether a Wifi connection could be established, either turn red (error) or blink, alternating green and blue. In case of a Wifi error, a diagnostic message will also show up at the bottom, e.g.:

```console
Error: could not connect to Wi-Fi network: ESP_ERR_TIMEOUT
```

## Extra information about building 

If you want to try to build without flashing, you can run:
 ```toml
 cargo build --target riscv32imc-esp-espidf
 ```

 Additionally, if `cargo espflash --release --monitor /dev/ttyUSB0` has been successful, you can exit with `ctrl+C`, and run only the monitor with `espmonitor /dev/ttyUSB0` without flashing (special thanks to Anton Mak, Bjoern Quentin!)
 This can save a lot of time as you do not need to re-flash the program in its entirety. 

# Troubleshooting

## Build errors

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
$ rm -rf ~/.espressif
```

## Connecting to Wifi

- You will get an `ESP_ERR_TIMEOUT` error also in case your network name or password are incorrect, so double-check those.