# Interrupts

An interrupt is a request for the processor to interrupt currently executing so that the event can be processed timely. If the request is accepted, the processor will suspend its current activities, save its state, and execute a function called an interrupt handler to deal with the event. Interrupts are commonly used by hardware devices to indicate electronic or physical state changes that require time-sensitive attention, for example, pushing a button.

The fact that interrupt handlers can be called at any time provides a challenge in embedded Rust: It requires the existence of statically allocated mutable memory that both the interrupt handler and the main code can refer to, and it also requires that this memory is always accessible.

## Challenges

### Flash Memory

Flash memory doesn't fulfill this requirement as it is out of action for example during write operations. Interrupts that occur during this time will go unnoticed. In our example, this would result in no reaction when the button is pushed. We solve this by moving the interrupt handler into RAM.
### Statically Mutable Memory

In Rust, such memory can be declared by defining a `static mut`. But reading and writing to such variables is always unsafe, as without precautions race conditions can be triggered.

How do we handle this problem?

In our example, the ESP-IDF framework provides a `Queue` type that handles the shared-mutable state for us. We simply get a `QueueHandle` which uniquely identifies the particular `Queue` being used. However, the main thread is given this `QueueHandle_t` at run-time, so we still need a small amount of shared-mutable state to share the `QueueHandle_t` with the interrupt routine. We use an `Option<QueueHandle_t>`, which we statically initialize to `None`, and later replace with `Some(queue_handle)` when the queue has been created by ESP-IDF.

In the interrupt routine, Rust forces us to handle the case where the `static mut` is still `None`. If this happens, we can either return early, or we can `unwrap()` the value, which will exit the program with an error if the value wasn't previously set to `Some(queue_handle)`.

There is still a risk that `main()` might be in the processing of changing the value of the variable (i.e. changing the `QueueHandle_t` value) just as the interrupt routine fires, leaving it in an inconsistent or invalid state. We mitigate this by making sure we only set the value once, and we do so before the interrupt is enabled. The compiler can't check that this is safe, so we must use the `unsafe` keyword when we read or write the value.

<!-- An alternative to the `static mut` variable is to convert the `QueueHandle_t` to an integer, and store it in an `AtomicU32` or similar. These atomic types guarantee they can never be read in an intermediate or invalid state. However, they require special hardware support which is not available on the ESP32-C3. You would also still need to distinguish between a valid `QueueHandle_t` and some value that indicates the queue has not yet been created (perhaps `0xFFFF_FFFF`).
Yet another option is to use a special data structure which disables interrupts automatically when the value is being access. This guarantees that no code can interrupt you when reading or writing the value. This does however increase interrupt latency and in this case, because the `QueueHandle_t` is only written once, this is not necessary. -->

Read more about this in the [Embedded Rust Book](https://docs.rust-embedded.org/book/concurrency/index.html)

## `unsafe {}` Blocks:

This code contains a lot of `unsafe {}` blocks. As a general rule, `unsafe` doesn't mean that the contained code isn't memory safe. It means, that Rust can't make safety guarantees in this place and that it is the responsibility of the programmer to ensure memory safety. For example, Calling C Bindings is per se unsafe, as Rust can't make any safety guarantees for the underlying C Code.



