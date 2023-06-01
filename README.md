# Embedded Rust Trainings for Espressif

[![CI](https://github.com/esp-rs/std-training/actions/workflows/ci.yml/badge.svg)](https://github.com/esp-rs/std-training/actions/workflows/ci.yml)

This repository contains Training Material for learning to use Embedded Rust with the Espressif ESP32-C3.

We suggest you start by [reading the book](https://esp-rs.github.io/std-training).

## Contents

There is:

* A book you can work through - ([Source](./book)) ([Published](https://esp-rs.github.io/std-training))
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
   * [`get-uuid`](./common/lib/get-uuid) - Provides a compile-time generated UUID
   * [`mqtt-messages`](./common/lib/mqtt-messages) - MQTT helper functions
   * [`rgb-led`](./common/lib/rgb-led) - Provides support for the RGB LED (WS2812)
   * [`wifi`](./common/lib/wifi) - Wifi helper functions

## Development

Each Rust example crate provided here can be built in the usual fashion. See
the [Embedded Rust Bookshelf](https://docs.rust-embedded.org) for general
details, or each crate's own README.md file for specifics.

The book is written in Markdown, using
[mdbook](https://crates.io/crates/mdbook). You can render a local copy by
running:

```console
~ $ cargo install mdbook
~ $ git clone https://github.com/esp-rs/std-training.git
~ $ cd std-training/book
~/std-training/book $ mdbook serve
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
<https://esp-rs.github.io/std-training> automatically.

## Licence

The material in this repository is licensed
[CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/). All
material is Copyright 2022 Ferrous Systems GmbH, unless otherwise stated.

You are free to __Share__ and __Adapt__ but you must give __Attribution__ and
__Share Alike__.

In addition, the source code contained within this repository (either in the
book, or as separate examples) is made available under either the
[MIT](./LICENSE-MIT.txt) or [Apache-2.0](./LICENSE-APACHE.txt) licenses, at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be licensed as above, without any additional terms or
conditions.

## Authors

The content of this training was created by Ferrous Systems GmbH and Espressif Systems.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct](https://www.rust-lang.org/policies/code-of-conduct), and the maintainers of this crate promises to intervene to
uphold that code of conduct.
