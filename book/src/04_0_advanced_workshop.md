# Advanced Workshop

In this course, we're going to dive deeper into topics that are embedded-only and/or close to the hardware, especially focussing on lower level I/O. Unlike in the first part, we'll not just use the higher level abstractions where for example something like pin configurations are hidden away. Instead, you'll learn how to configure them yourself. You're also going to learn how to write directly into registers and how to find out which register is needed in the first place. We'll talk about ownership issues and memory safety issues in the context of exercises.

This part consists of three exercises:

In the first one, you'll learn how to handle a button interrupt, in the second you'll read sensor values from sensors via the I²C bus. Once you have used the drivers we have prepared, you'll learn how to get started writing your own. This is a necessary skill as Rust drivers are usually not provided by manufacturers.

## Preparations
Please go through the [preparations](./02_0_preparations.md) chapter to prepare for this workshop.

## Reference
If you're new to embedded programming, read our [reference](./05_reference.md) where we explain some terms in a basic manner.
