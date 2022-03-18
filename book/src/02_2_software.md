# Software

Follow the steps below for a default installation of the ESP32-C3 platform tooling. 

🔎 Should you desire a customized installation (e.g. building parts from source, or add support for Xtensa/ESP32-S3), instructions for doing so can be found in the [Installing Rust](https://esp-rs.github.io/book/dependencies/installing-rust.html) chapter of the *Rust on ESP* Book. 

## Rust toolchain

✅ If you haven't got Rust on your computer, obtain it via <https://rustup.rs/>

Furthermore, for ESP32-C3, a *nightly* version of the Rust toolchain is currently required.

✅ Install nightly Rust and add support for the target architecture using the following console commands:

```console
$ rustup install nightly-2021-11-18
$ rustup component add rust-src --toolchain nightly-2021-11-18
```

🔎 Rust is capable of cross-compiling to any supported target (see `rustup target list`). By default, only the native architecture of your system is installed.
To build for the Xtensa architecture (*not* part of this material), a fork of the Rust compiler is required as of January 2022.

## Espressif toolchain

Several tools are required:
- `cargo-generate` - general purpose project setup wizard
- `cargo-espflash` - upload firmware to the microcontroller
- `espmonitor` - monitor firmware log messages
- `bindgen` - generate Rust bindings for C APIs
- `ldproxy` - Espressif build toolchain dependency

✅ Install them with the following command:

```console
$ cargo install cargo-generate cargo-espflash espmonitor bindgen ldproxy
```

## Toolchain dependencies

`bindgen` relies upon a few compiler packages. How to install these depends on your operating system - here are a few examples, further documentation (including Windows) can be found in the official [bindgen requirements](https://rust-lang.github.io/rust-bindgen/requirements.html) document.

### Debian/Ubuntu

```console
$ sudo apt install llvm-dev libclang-dev clang
```
### macOS

(when using the Homebrew package manager, which we recommend)
```console 
$ brew install llvm
```

### Troubleshooting

- Python 3 is a required dependency. It comes preinstalled on stock macOS and typically on desktop Linux distributions. An existing **Python 2** installation with the `virtualenv` add-on pointing to it is known to potentially cause build problems. 

- Error `failed to run custom build command for libudev-sys v0.1.4` or `esp-idf-sys v0.30.X`:

    At time of writing, this can be solved by 
    1. running [this line](https://github.com/esp-rs/rust-build/blob/f773036483333f3b4618d988f9a1eda051573cb2/support/esp-rs-rust/Containerfile#L13) from the `esp-rs` container:

    `apt-get update \
    && apt-get install -y vim nano git curl gcc ninja-build cmake libudev-dev python3 python3-pip libusb-1.0-0 libssl-dev \
    pkg-config libtinfo5`

    2. restarting the terminal

    3. If this is not working, try `cargo clean`, remove the `~/.espressif` folder and reinstall [according to esp instructions](
https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/linux-macos-setup.html).

## Additional software (IDE + recommended plugins)

- When using VSCode, the "Even Better TOML" extension is a useful addition for editing configuration files. 
