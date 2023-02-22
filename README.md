# Embedded Rust Trainings for Espressif

This repository contains Training Material for learning to use Embedded Rust
with the Espressif ESP32-C3. 

⚠️ This material is based on unstable crates. It worked at the time of writing but following it today may not result in compiling code! However, it can be used as inspiration for getting started with Rust on Espressif. You can join the [esp-rs community](https://matrix.to/#/#esp-rs:matrix.org) on Matrix for all technical questions and issues! The community is open to everyone.

We suggest you start by [reading the book](https://esp-rs.github.io/espressif-trainings).


[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/esp-rs/espressif-trainings)

## Contents

There is:

* A book you can work through - ([Source](./book)) ([Published](https://esp-rs.github.io/espressif-trainings))
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
   * [`icm42670p`](./common/lib/icm42670p) - basic sensor driver
* Some extra bits:
   * [`mqtt-python-client`](./extra/mqtt-python-client) A Python MQTT client, for testing

Please note, much of this material remains a work in progress!

## Development

Each Rust example crate provided here can be built in the usual fashion. See
the [Embedded Rust Bookshelf](https://docs.rust-embedded.org) for general
details, or each crate's own README.md file for specifics.

The book is written in Markdown, using
[mdbook](https://crates.io/crates/mdbook). You can render a local copy by
running:

```console
~ $ cargo install mdbook
~ $ git clone https://github.com/esp-rs/espressif-trainings.git
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
<https://esp-rs.github.io/espressif-trainings> automatically.

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
Conduct][CoC], and the maintainers of this crate promises to intervene to
uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
