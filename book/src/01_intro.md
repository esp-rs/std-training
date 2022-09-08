<p style="text-align:center;"><img src="./assets/esp-logo-black.svg" width="50%"></p>

# Introduction

## Content of this material

This is Ferrous Systems' *Embedded Rust on Espressif* training material. It is divided into two workshops: introductory and advanced. The introductory trail will introduce you to basics of embedded development and how to make the embedded board interact with the outside world - reacting to commands and sending sensor data.

The advanced course takes it from there to dive deeper into topics like interrupt handling, low-level peripheral access and writing your own drivers.

⚠️ You will be able to chat with our experts during the training if you are participating to the training. In addition, we strongly suggest to join the [community](https://matrix.to/#/#esp-rs:matrix.org) on the Matrix for all technical questions or delivery issues! The community is open to everyone.

## The board

An [Espressif Rust Board](https://github.com/esp-rs/esp-rust-board) is mandatory[^note] for working with this book - emulators like QEMU are not supported. 


The board design and images, pin layout and schematics can be also found in this repository.

If you subscribed to one of the trainings, a board will be provided to you directly by Espressif.
Some exercises also require wireless internet access.



Our focus lies primarily on the [ESP32-C3](https://www.espressif.com/en/products/socs/esp32-c3) platform, a [RISC-V](https://riscv.org/) based microcontroller with strong IoT capabilities, facilitated by integrated Wi-Fi and Bluetooth 5 (LE) functionality as well as large RAM + flash size for sophisticated applications. A substantial amount of this course is also applicable for Xtensa the other architecture Espressif uses, in particular the [ESP32-S3](https://www.espressif.com/en/products/socs/esp32-s3). For low-level access the general principles apply as well, but actual hardware access will differ in various ways - refer to the technical reference manuals ([C3](https://www.espressif.com/sites/default/files/documentation/esp32-c3_technical_reference_manual_en.pdf), [S3](https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf)) or [other available technical documents](https://www.espressif.com/en/support/documents/technical-documents)  as needed.


## Rust knowledge

Basic Rust like [The Rust Book](https://doc.rust-lang.org/book/) Chapters 1 - 6, Chapter 4 Ownership does not need to be fully understood.  


[^note]: It is possible to follow the intro part with [ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html) but we do not recommend it. It is inherently easier to follow the training when using the same hardware.