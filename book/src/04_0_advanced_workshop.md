# Advanced Workshop

In this course we're going to dive deeper into topics that are embedded-only and/or close to the hardware, especially focussing on lower level i/o. Unlike in the first part, we'll not just use the higher level abstractions where for example something like pin configurations are hidden away. Instead you'll learn how to configure them yourself. You're also going to learn how to write directly into registers and how to find out which register is needed in the first place. We'll talk about ownership issues and memory safety issues in the context of exercises.

 This part consists of three exercises: 

 In the first one you'll learn how to handle a button interrupt, in the second you'll read sensor values from sensors via the I²C bus. Once you have used the drivers we have prepared, you'll learn how to get started writing your own. This is a necessary skill as Rust drivers are usually not provided by manufacturers. 

 ## Preparations
 
 We're going to assume you have completed the entire [preparations](./02_preparations.md) chapter and are familiar with the responsibilities of the different components TODO LINK of the `esp-rs` ecosystem. 

 ## Reference

 If you're new to embedded programming read our [reference](./04_7_reference.md) where we explain some terms in a basic manner. 