# Software

Follow the steps below for a default installation of the ESP32-C3 platform tooling. 

🔎 Should you desire a customized installation (e.g. building parts from source, or add support for Xtensa/ESP32-S3), instructions for doing so can be found in the [Installing Rust](https://esp-rs.github.io/book/dependencies/installing-rust.html) chapter of the *Rust on ESP* Book. 

## Rust toolchain

✅ If you haven't got Rust on your computer, obtain it via <https://rustup.rs/>

Furthermore, for ESP32-C3, a specific *nightly* version of the Rust toolchain is currently required.

✅ Install nightly Rust and add support for the target architecture using the following console commands:

```console
$ rustup install nightly nightly-2022-03-10
$ rustup component add rust-src --toolchain nightly-2022-03-10
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


## Docker

> ❗️ Please **note** using the *alternative* Docker setup is meant for users who have experience working with containers. Be aware that we cannot provide help for Docker specific issues during the training.

An alternative option to set up the environment & to build the exercises is to use Docker. The repository contains a `Dockerfile` that can be used to work in a virtualized environment.

✅ Install [`Docker`](https://docs.docker.com/get-docker/) for your operating system.

To build the Docker image run the following command from the root folder:

```console
$ docker image build --tag esp --file .devcontainer/Dockerfile .
```

This creates a new image, installs the Rust toolchain, all necessary packages & dependencies.
Building the image takes a while depending on the OS & hardware (20-30 minutes).

To start a new container run:

```console
$ docker run --user esp --mount type=bind,source="$(pwd)",target=/home/esp/workspace,consistency=cached -it esp /bin/bash
```

This starts an interactive shell in the Docker container. It also mounts the local repository to a folder
named `/home/esp/workspace` in the container. Changes on the host system are reflected inside the container.


## Additional Software

### VS Code

One editor with good Rust support is [VS Code](https://code.visualstudio.com/) which is available for most platforms.
When using VS Code we recommend the following extensions to help during the development.

* `Even Better TOML` for editing TOML based configuration files
* [`Rust Analyzer`](https://rust-analyzer.github.io/) to provide code completion & navigation

There are a few more useful extensions for advanced usage

* [`lldb`](https://github.com/vadimcn/vscode-lldb) a native debugger extension based on LLDB
* [`crates`](https://github.com/serayuzgur/crates) to help manage Rust dependencies

### VS Code & Devcontainer

One extension for VS Code that might be helpful to develop inside a Docker container is [`Remote Containers`](https://github.com/Microsoft/vscode-remote-release).
It uses the same `Dockerfile` as the Docker setup, but builds the image and connects to it from within VS Code.
Once the extension is installed VS Code recognizes the configuration in the `.devcontainer` folder. Use the `Remote Containers - Reopen in Container` command to connect VS Code to the container.
