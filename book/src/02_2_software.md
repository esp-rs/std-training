# Software

Follow the steps below for a default installation of the ESP32-C3 platform tooling. 

🔎 Should you desire a customized installation (e.g. building parts from source, or add support for Xtensa/ESP32-S3), instructions for doing so can be found in the [Installing Rust](https://esp-rs.github.io/book/dependencies/installing-rust.html) chapter of the *Rust on ESP* Book. 

## Rust toolchain

✅ If you haven't got Rust on your computer, obtain it via <https://rustup.rs/>

Furthermore, for ESP32-C3, a *nightly* version of the Rust toolchain is currently required.

✅ Install nightly Rust and add support for the target architecture using the following console commands:

TODO nightly is currently broken, so we're using one from 2021. Once this is resolved, change **ALL** occurrences of `nightly-2021-11-18` to `nightly` (verify with recursive grep)

TODO pin nightly version via `rust-toolchain.toml` (Ferrous internal: see rust-experts/ferrous/Target not found)

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

## Additional software (IDE + recommended plugins)

- TODO copy from <https://embedded-trainings.ferrous-systems.com/installation.html>? Maybe just leave it out…

- When using VSCode, the "Better TOML" extension is a useful addition for editing configuration files. 