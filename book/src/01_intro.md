<p style="text-align:center;"><img src="./assets/esp-logo-black.svg" width="50%"></p>

# Introduction

## Content of This Material

This is Ferrous Systems' *Embedded Rust on Espressif* training material. It is divided into two workshops: introductory and advanced. The introductory trail will introduce you to the basics of embedded development and how to make the embedded board interact with the outside world - reacting to commands and sending sensor data.

The advanced course takes it from there to dive deeper into topics like interrupt handling, low-level peripheral access and writing your own drivers.

You can join the [esp-rs community](https://matrix.to/#/#esp-rs:matrix.org) on Matrix for all technical questions and issues! The community is open to everyone.

## Translations

This book has been translated by generous volunteers. If you would like your translation listed here, please open a PR to add it.

- [简体中文](https://narukara.github.io/std-training-zh-cn/) ([repository](https://github.com/Narukara/std-training-zh-cn))

## The Board

A [Rust ESP Board](https://github.com/esp-rs/esp-rust-board) is mandatory[^note] for working with this book - emulators like QEMU aren't supported.


The board design and images, pin layout and schematics can also be found in this repository.

If you are subscribed to one of the trainings, a board will be provided to you directly by Espressif.

Our focus lies primarily on the [ESP32-C3](https://www.espressif.com/en/products/socs/esp32-c3) platform, a [`RISC-V`](https://riscv.org/)-based microcontroller with strong IoT capabilities, facilitated by integrated Wi-Fi and Bluetooth 5 (LE) functionality as well as large RAM + flash size for sophisticated applications. A substantial amount of this course is also applicable for `Xtensa`, the other architecture Espressif uses, in particular the [ESP32-S3](https://www.espressif.com/en/products/socs/esp32-s3). For low-level access, the general principles apply as well, but actual hardware access will differ in various ways - refer to the technical reference manuals ([C3](https://www.espressif.com/sites/default/files/documentation/esp32-c3_technical_reference_manual_en.pdf), [S3](https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf)) or [other available technical documents](https://www.espressif.com/en/support/documents/technical-documents) as needed.

## Rust Knowledge

- Basic Rust like [The Rust Book](https://doc.rust-lang.org/book/) Chapters 1 - 6, Chapter 4 Ownership doesn't need to be fully understood.
- [The Rust on ESP Book](https://esp-rs.github.io/book/) isn't required, but it is highly recommended, as it can help you understand the Rust on ESP ecosystem and many of the concepts that will be discussed during the training.

[^note]: It is possible to follow the intro part with [ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html) but, we don't recommend it. It is inherently easier to follow the training when using the same hardware.
