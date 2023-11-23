# Interrupts

An interrupt is a request for the processor to interrupt currently executing so that the event can be processed timely. If the request is accepted, the processor will suspend its current activities, save its state, and execute a function called an interrupt handler to deal with the event. Interrupts are commonly used by hardware devices to indicate electronic or physical state changes that require time-sensitive attention, for example, pushing a button.

The fact that interrupt handlers can be called at any time provides a challenge in embedded Rust: It requires the existence of statically allocated mutable memory that both the interrupt handler and the main code can refer to, and it also requires that this memory is always accessible.

## `unsafe {}` Blocks

This code contains a lot of [`unsafe {}` blocks][rust-unsafe]. As a general rule, `unsafe` doesn't mean that the contained code isn't memory safe. It means, that Rust can't make safety guarantees in this place and that it is the responsibility of the programmer to ensure memory safety. For example, Calling C Bindings is per se unsafe, as Rust can't make any safety guarantees for the underlying C Code.

[rust-unsafe]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
