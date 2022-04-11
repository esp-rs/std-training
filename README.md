# Embedded Rust Trainings for Espressif

This repository contains Training Material for learning to use Embedded Rust
with the Espressif ESP32-C3.

We suggest you start by [reading the book](https://espressif-trainings.ferrous-systems.com).

## Contents

There is:

* A book you can work through - ([Source](./book)) ([Published](https://espressif-trainings.ferrous-systems.com))
* Some introductory level examples:
   * A basic hardware-check ([Source](./intro/hardware-check))
   * An HTTP Client ([Source](./intro/http-client))
   * An HTTP Server ([Source](./intro/http-server))
   * An MQTT Client ([Source](./intro/mqtt))
* Some advanced level examples:
   * Low-level GPIO
   * Interrupts in General
   * I2c Driver ([Source](./advanced/i2c-driver))
   * I2c Sensor Reading ([Source](./advanced/i2c-sensor-reading))
   * GPIO/Button Interrupts ([Source](./advanced/button-interrupt))
   * Driving an RGB LED
* Some useful common crates:
   * [`esp32-c3-dkc02-bsc`](./common/lib/esp32-c3-dkc02-bsc) - Board-Support for the ESP32-C3-DKC02
   * [`get-uuid`](./common/lib/get-uuid) - provides a compile-time generated UUID
   * [`mqtt-messages`](./common/lib/mqtt-messages) - MQTT helper functions
   * [`imc42670p`](./common/lib/imc42670p) - basic sensor driver
* Some extra bits:
   * [`mqtt-python-client`](./extra/mqtt-python-client) A Python MQTT client, for testing

Please note, much of this material remains a work in progress!

## Devcontainer

The repository includes the option to use [Visual Studio Code remote
containers](https://code.visualstudio.com/docs/remote/containers), the
development container includes all the necessary dependencies alongside helpful
VS Code extension and settings. For more information on how to use it, refer to
[esp-rs-devcontainer](https://github.com/SergioGasquez/esp-rs-devcontainer).
> If using Podman, follow along the [Podman setup section](https://github.com/SergioGasquez/esp-rs-devcontainer#optional-podman).

The repository offers the choice to use a [built image, hosted on Dockerhub](https://hub.docker.com/repository/docker/sergiogasquez/esp-rs-env)
,or to build it from a Dockerfile. The default choice is using the Dockerhub image
since it's faster, in order to change it, edit `devcontainer.json`or `.gitpod.yml`.

Developing in an online dev environment is also available with [Gitpod](https://www.gitpod.io/):

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/SergioGasquez/espressif-trainings/tree/main)

When using an online Gitpod environment, flashing and monitoring can be done
with online tools, for more information have a look at [Flash](https://github.com/SergioGasquez/esp-rs-devcontainer#adafruit-esptool)
and [Monitor](https://github.com/SergioGasquez/esp-rs-devcontainer#online-serial-monitor)
sections of [esp-rs-devcontainer](https://github.com/SergioGasquez/esp-rs-devcontainer) repository.

### Wokwi Simulator
The devcontainer includes the option of simulating the exercises with [Wokwi](https://wokwi.com/).
In order to build and run a Wokwi simulation, several task are provided via `.vscode/tasks.json`:
- `Build and run Wokwi simulation: Focused project` is the default build task. 
  To use it, open the `main.rs` file of your project you would like to compile 
  & run. Please ensure the file is in focus in VS Code. 
> Default build task can be run from: 
> - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Build Task` command 
> - `Terminal`-> `Run Build Task` in the menu.
> - With `Ctrl-Shift-B` or `Cmd-Shift-B`
- `Build and run Wokwi simulation: <project>` Builds and run simulation for the
  selected `<project>`. 
> Task can be run from: 
> - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command 
> - `Terminal`-> `Run Task` in the menu.

A script, `run.sh`, under the `wokwi` folder, is also provided with the same purpose, build and run the Wokwi simulation for the different projects, in order
to use it:
1. Set the `CURRENT_PROJECT` environment variable:
   `$ export CURRENT_PROJECT=<project>`. Possible values of `<project>` are:
   - `intro/hardware-check`
   - `intro/http-client/exercise`
   - `intro/http-client/solution`
   - `intro/http-server/exercise`
   - `intro/http-server/solution`
   - `intro/mqtt/exercise`
   - `intro/mqtt/host-client`
   - `intro/mqtt/solution`
   - `advanced/button-interrupt/exercise`
   - `advanced/button-interrupt/solution`
   - `advanced/i2c-driver/solution`
   - `advanced/i2c-sensor-reading/solution`
2. Run the bash script: `$ bash wokwi/run.sh`

> When using Gitpod online enviroment, using the script is the reccomended 
> method since VScode task are not available

## Development

Each Rust example crate provided here can be built in the usual fashion. See
the [Embedded Rust Bookshelf](https://docs.rust-embedded.org) for general
details, or each crate's own README.md file for specifics.

The book is written in Markdown, using
[mdbook](https://crates.io/crates/mdbook). You can render a local copy by
running:

```console
~ $ cargo install mdbook
~ $ git clone https://github.com/ferrous-systems/espressif-trainings.git
~ $ cd espressif-trainings/book
~/espressif-trainings/book $ mdbook serve
```

A local web-server will be started on <http://127.0.0.1:3000> where you can
view the rendered book. It will update automatically as you modify Markdown
pages on disk.

Note that you __must__ not push to the `main` branch. Instead undertake any
changes in a branch, either in this repository (if you have access) or in a
fork. Please do then feel free to open a Pull Request in Github to merge the
changes to our `main` branch.

This work is continually updated and as such there are no 'releases'. Every
commit to `main` gets published to
<https://espressif-trainings.ferrous-systems.com> automatically.

## Licence

The material in this repository is licensed
[CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/). All
material is Copyright 2022 Ferrous Systems GmbH, unless otherwise stated.

You are free to __Share__ and __Adapt__ but you must give __Attribution__ and
__Share Alike__.

In addition, the source code contained within this repository (either in the
book, or as separate examples) is made available under either the
[MIT](./LICENSE-MIT.txt) or [Apache-2.0](./LICENSE_APACHE.txt) licenses, at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be licensed as above, without any additional terms or
conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], and the maintainer of this crate, Ferrous Systems GmbH, promises
to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
