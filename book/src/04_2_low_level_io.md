# Lower level I/O: How to manipulate Registers

In general there are two ways to write firmware for the esp32. One is the bare-metal using only `[no_std]` Rust, and the other using `[std]` Rust and C-Bindings to the esp-idf.
`[no_std]` Rust refers to Rust not using the standard library, only the core library, which is a subset of the standard library that does not depend on the existence of an operating system. 

## What do the ecosystems look like?

### `[std]` Rust and the esp-idf

The most established way to use Rust on esp32-c3 is using C bindings to the esp-idf. We can use Rust's standard library when going this route, as we can use an operating system: FreeRTOS. Being able to use the standard library comes with benefits: We can use all types no matter if they are stack or heap allocated. We can use threads, Mutexes and other synchronization primitives.

 The esp-idf is mostly written in C and as such is exposed to Rust in the canonical split crate style: 
- a `sys` crate to provide the actual `unsafe` bindings ([esp-idf-sys](https://github.com/esp-rs/esp-idf-sys))
- a higher level crate offering safe and comfortable Rust abstractions ([esp-idf-svc](https://github.com/esp-rs/esp-idf-svc/))

The final piece of the puzzle is low-level hardware access, which is again provided in a split fashion:
- [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal) implements the hardware-independent [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits like analog/digital conversion, digital I/O pins, or SPI communication - as the name suggests, it also uses `esp-idf` as a foundation

More information is available in the [ecosystem chapter](https://esp-rs.github.io/book/overview/using-the-standard-library.html) of the `esp-rs` book.

This is the way that currently allows the most possibilities on Espressif chips if you want to use Rust. Everything in this course is based on this approach. 

We're going to look at how to write values into Registers in this ecosystem in the context of the Interrupt exercise. 

### Bare metal Rust with `[no_std]`

As the name bare metal implies, we don't use an operating system. Because of this, we can't use language features that rely on one. The core library is a subset of the standard library that excludes features like heap allocated types and threads. Code that uses only the core library is labelled with `[no_std]`. `[no_std]` code can always run in a `[std]` environment, the reverse does not work. 
In Rust the mapping from Registers to Code works like this: Registers and their fields on a device are described in system view description (svd) files. `svd2rust` is used to generate peripheral access crates (PACs) from them. PACS provide a thin wrapper over the various memory-wrapper registers defined for the particular part-number of micro-controller you are using. While it is possible to write firmware with a PAC alone, some of it would unsafe or otherwise inconvenient as it only provides access to the most basic peripherals of the microcontroller. So there is another layer, the Hardware Abstraction Layer (HAL). HALs provide a more user friendly API for the chip, and often implement common traits defined in the embedded-hal. For some specific development Kits board support crates exist. They provide yet another layer of abstraction for specific development kits and additionally provide an api to sensors and LEDs that may be on the board. 

Although a [PAC](https://github.com/esp-rs/esp32c3) for the esp32c3 exists, bare-metal Rust is highly experimental on esp32c3 chips, so for now we will not work with it on the microcontroller directly. We will write a partial sensor driver in this approach as driver's should be platform agnostic. 



