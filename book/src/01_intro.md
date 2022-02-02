# Introduction

This is Ferrous Systems' *Embedded Rust on Espressif* training material. It is divided into two workshops: introductory and advanced. The introductory trail will introduce you to basics of embedded development (TODO describe in more detail) and how to make the embedded board interact with the outside world - reacting to commands and sending sensor data.

The advanced course takes it from there to dive deeper into topics like interrupt handling, low-level peripheral access and writing your own drivers.

An [Espressif Rust Board TODO link](https://) is mandatory for working with this book - emulators like QEMU are not supported. Some exercises also require wireless internet access.

Our focus lies primarily on the [ESP32-C3](https://www.espressif.com/en/products/socs/esp32-c3) platform, a [RISC-V](https://riscv.org/) based microcontroller with strong IoT capabilities, facilitated by integrated Wi-Fi and Bluetooth 5 (LE) functionality as well as large RAM + flash size for sophisticated applications. A substantial amount of this course is also applicable for Xtensa the other architecture Espressif uses, in particular the [ESP32-S3](https://www.espressif.com/en/products/socs/esp32-s3). For low-level access the general principles apply as well, but actual hardware access will differ in various ways - refer to the technical reference manuals ([C3](https://www.espressif.com/sites/default/files/documentation/esp32-c3_technical_reference_manual_en.pdf), [S3](https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf)) or [other available technical documents](https://www.espressif.com/en/support/documents/technical-documents)  as needed.

## Rust knowledge

A basic familiarity with the Rust programming language is assumed. 

TODO
- specify what kind of knowledge is required
- link learning resources (Ferrous teaching material, Rust book, awesome-rust-..)
