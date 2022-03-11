# Interrupts

An interrupt is a request for the processor to interrupt currently executing code, so that the event can be processed in a timely manner. If the request is accepted, the processor will suspend its current activities, save its state, and execute a function called an interrupt handler to deal with the event. Interrupts are commonly used by hardware devices to indicate electronic or physical state changes that require time-sensitive attention, for example pushing a button. 

## Challenges

The fact that interrupt handlers can be called at any time provides a challenge in embedded Rust as this requires the existence of statically allocated mutable memory that both the interrupt handler and the main code can refer to. In Rust such memory can be declared by defining a `static mut`. But reading and writing to such variables is always unsafe, as without precautions race conditions can be triggered. 

How do we handle this problem?

In our example the `static mut` holds the state of the event queue: was there an interrupt or was there no interrupt? We read this state in `main()` only. While this technically can trigger a race condition because a `write` from the interrupt handler can overlap the `read` from `main()`, this would in the worst case cause an unnoticed interrupt. This is unproblematic in a way as other ways to handle this memory safe for example with critical sections or atomic operations would cause the same problem: If they are in action an interrupt would go unnoticed. 

Read more about this in the [Embedded Rust Book](https://docs.rust-embedded.org/book/concurrency/index.html)

## `unsafe {}` blocks:

This code contains a lot of `unsafe {}` blocks. As a general rule, `unsafe` does not mean that the contained code is not memory safe, it means, that Rust can't make safety guarantees in this place and that it is in the responsibility of the programmer to ensure memory safety. For example Calling C Bindings is per se unsafe, as Rust can't make any safety guarantees for the underlaying C Code. 



