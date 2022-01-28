# Hello, board!

You're now ready to do a consistency check.

✅ Connect the USB-C port of the board to your computer and enter the hardware check directory in the workshop repository:

```console
espressif-trainings$ cd intro/hardware-check
```

To test Wi-Fi connectivity, you will have to provide your network name (SSID) and password (PSK). These credentials are stored in a dedicated `config.toml` file (which is `.gitignore`d) to prevent accidental disclosure by sharing source code or doing pull requests. An example is provided. 

✅ Copy `config.toml.example` to `config.toml` and edit it to reflect your actual credentials:

```console
$ cp config.toml.example config.toml
$ $EDITOR config.toml
$ cat config.toml
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