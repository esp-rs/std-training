# Lower Level I/O: How to Manipulate Registers

There are [two ways to write firmware for the ESP32-C3](https://esp-rs.github.io/book/overview/index.html):
 - One is the bare-metal using only `[no_std]` Rust.
 - The other using `[std]` Rust and C-Bindings to ESP-IDF.

> [`[no_std]` Rust](https://docs.rust-embedded.org/book/intro/no-std.html) refers to Rust not using the standard library, only the [core library](https://doc.rust-lang.org/core/), which is a subset of the [standard library](https://doc.rust-lang.org/std/) that doesn't depend on the existence of an operating system.

## What do the Ecosystems Look Like?

### `[std]` Rust and the ESP-IDF

This way relies on using C bindings to ESP-IDF. We can use Rust's standard library when going this route, as we can use an operating system: ESP-IDF, which is based on [FreeRTOS](https://www.freertos.org/). Being able to use the standard library comes with benefits: We can use all types, no matter if they are stack or heap allocated. We can use threads, Mutexes and other synchronization primitives.

 The ESP-IDF is mostly written in C and as such is exposed to Rust in the canonical split crate style:
- A `sys` crate to provide the actual `unsafe` bindings ([esp-idf-sys](https://github.com/esp-rs/esp-idf-sys))
- A higher level crate offering safe and comfortable Rust abstractions ([esp-idf-svc](https://github.com/esp-rs/esp-idf-svc/))

The final piece of the puzzle is low-level hardware access, which is again provided in a split fashion:
- [`esp-idf-hal`](https://github.com/esp-rs/esp-idf-hal) implements the hardware-independent [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits like analog/digital conversion, digital I/O pins, or SPI communication - as the name suggests, it also uses ESP-IDF as a foundation

More information is available in the [ecosystem chapter](https://esp-rs.github.io/book/overview/using-the-standard-library.html) of _The Rust on ESP Book_.

This is the way that currently allows the most possibilities on Espressif chips if you want to use Rust. **Everything in this course is based on this approach.**

We're going to look at how to write values into Registers in this ecosystem in the context of the Interrupts exercise.

### Bare Metal Rust with `[no_std]`

As the name bare metal implies, we don't use an operating system. Because of this, we can't use language features that rely on one. The core library is a subset of the standard library that excludes features like heap allocated types and threads. Code that uses only the core library is labelled with `#[no_std]`. `#[no_std]` code can always run in a `std` environment, but the reverse isn't true.
In Rust, the mapping from Registers to Code works like this:

Registers and their fields on a device are described in [_System View Description_ (SVD) files](http://www.disca.upv.es/aperles/arm_cortex_m3/curset/CMSIS/Documentation/SVD/html/index.html). [`svd2rust`](https://docs.rs/svd2rust/latest/svd2rust/) is used to generate _Peripheral Access Crates_ (PACs) from these SVD files. The PACs provide a thin wrapper over the various memory-mapped registers defined for the particular model of microcontroller you are using.

Whilst it is possible to write firmware with a PAC alone, some of it would prove unsafe or otherwise inconvenient as it only provides the most basic access to the peripherals of the microcontroller. So there is another layer, the _Hardware Abstraction Layer_ (HAL). HALs provide a more user-friendly API for the chip, and often implement common traits defined in the generic [`embedded-hal`](https://github.com/rust-embedded/embedded-hal).

Microcontrollers are usually soldered to some _Printed Circuit Board_ (or just _Board_), which defines the connections that are made to each pin. A _Board Support Crate_ (BSC, also known as a _Board Support Package_ or BSP) may be written for a given board. This provides yet another layer of abstraction and might, for example, provide an API to the various sensors and LEDs on that board - without the user necessarily needing to know which pins on your microcontroller are connected to those sensors or LEDs.

We will write a partial sensor driver in this approach, as driver's should be platform-agnostic.



