# Introduction

This is Ferrous Systems' *Embedded Rust on Espressif* training material. It is divided into two workshops: introductory and advanced. The introductory trail will introduce you to basics of embedded development and how to make the embedded board interact with the outside world - reacting to commands and sending sensor data.

The advanced course takes it from there to dive deeper into topics like interrupt handling, low-level peripheral access and writing your own drivers.

<<<<<<< HEAD
An [Espressif Rust Board]([https://](https://github.com/esp-rs/esp-rust-board)) or [ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html) (intro part only!) is mandatory for working with this book - emulators like QEMU are not supported. Some exercises also require wireless internet access.
=======
An [Espressif Rust Board](https://github.com/esp-rs/esp-rust-board) - to be released soon - or [ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html) (intro part only!) is mandatory for working with this book - emulators like QEMU are not supported. Some exercises also require wireless internet access.
>>>>>>> 4634d20 (Fix broken link to ESP-RS board, make a remark about it not being released yet.)

Our focus lies primarily on the [ESP32-C3](https://www.espressif.com/en/products/socs/esp32-c3) platform, a [RISC-V](https://riscv.org/) based microcontroller with strong IoT capabilities, facilitated by integrated Wi-Fi and Bluetooth 5 (LE) functionality as well as large RAM + flash size for sophisticated applications. A substantial amount of this course is also applicable for Xtensa the other architecture Espressif uses, in particular the [ESP32-S3](https://www.espressif.com/en/products/socs/esp32-s3). For low-level access the general principles apply as well, but actual hardware access will differ in various ways - refer to the technical reference manuals ([C3](https://www.espressif.com/sites/default/files/documentation/esp32-c3_technical_reference_manual_en.pdf), [S3](https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf)) or [other available technical documents](https://www.espressif.com/en/support/documents/technical-documents)  as needed.

## Rust knowledge

Basic Rust like [The Rust Book](https://doc.rust-lang.org/book/) Chapters 1 - 6, Chapter 4 Ownership does not need to be fully understood.  
