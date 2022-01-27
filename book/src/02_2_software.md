# Software

Follow the steps below for a default installation of the ESP32-C3 platform tooling. 

🔎 Should you desire a customized installation (e.g. building parts from source, or add support for Xtensa/ESP32-S3), instructions for doing so can be found in the [Installing Rust](https://esp-rs.github.io/book/dependencies/installing-rust.html) chapter of the *Rust on ESP* Book. 

## Rust toolchain

✅ If you haven't got Rust on your computer, obtain it via <https://rustup.rs/>

Furthermore, for ESP32-C3, a *nightly* version of the Rust toolchain is currently required.

✅ Install nightly Rust and add support for the target architecture using the following console commands:

TODO nightly is currently broken, so we're using one from 2021. Once this is resolved, change **ALL** occurrences of `nightly-2021-11-18` to `nightly` (verify with recursive grep)

```console
$ rustup install nightly-2021-11-18
$ rustup target add riscv32imc-esp-espidf --toolchain nightly-2021-11-18
$ rustup component add rust-src --toolchain nightly-2021-11-18
```

🔎 Rust is capable of cross-compiling to any supported target (see `rustup target list`). By default, only the native architecture of your system is installed.
To build for the Xtensa architecture (*not* part of this material), a fork of the Rust compiler is required as of January 2022.

## Espressif toolchain

Several tools are required:
- `cargo-generate` - general purpose project setup wizard
- `cargo-espflash` - upload firmware to the microcontroller
- `espmonitor` - monitor firmware log messages

```console
$ cargo install cargo-generate cargo-espflash espmonitor
```


## Workshop repository

The entire training material can be found at <https://github.com/ferrous-systems/espressif-trainings>.

✅ Clone and change into the workshop repository:

```console
$ git clone "https://github.com/ferrous-systems/espressif-trainings.git"
$ cd espressif-trainings
```

### Repository contents

- `intro/` - code examples and exercises for the introduction course
- `advanced/` - code examples and exercises for the advanced course
- `common/` - code shared between both courses
- `common/vendor/` - third party crates that have been forked to add required support, pending upstream merges TODO: hopefully none required
- `book/` - markdown sources of this book

## Additional software (IDE + recommended plugins)

TODO copy from <https://embedded-trainings.ferrous-systems.com/installation.html>? Maybe just leave it out…