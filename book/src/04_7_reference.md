# Reference
## GPIO

GPIO is short for General Purpose Input Output. GPIOs are digital (or sometimes analogue) signal pins that can be used as interfaces to other systems or devices. Each pin can be in various states, but they will have a default state on power-up or after a system reset (usually a harmless one, like being a digital input). We can then write software to change them into the appropriate state for our needs.

We'll introduce a couple of concepts related to GPIOs:

### Pin Configurations

GPIOs can be configured to four different ways depending on their connectedness. 

Floating: A floating pin is neither connected VCC nor ground. The voltage will match the residual voltage.

Push-Pull-Output: A pin that is configured as push–pull output can switch between high and low voltage.

Open-Drain-Output: Open Drain outputs switch between "disconnected" and "connected to ground".

Pull-Up-Input: A pin that is configured as pull-up input is set to VCC, as long as it is not overwritten by an external source. This setting prevents the pin from floating, which can cause noise in the system. 

### Active high/low 

A digital signal can be in two states: `high` and `low`. This is usually represented by the voltage difference between the signal and ground. It is arbitrary which of these voltage levels represents which logic states: So both `high` and `low` can be defined as an active state. 

For example: An active high pin has voltage when the logic level is active. And active low pin has voltage when the logic level is set to inactive. 

In embedded Rust abstractions show the logic level and not the voltage level. So if you have an active low pin connected to an LED, you need to set it to inactive in order for the LED to light up. 

### Chip Select 

Chip select is a binary signal to an IC that can switch this IC on or off partially or entirely. It is usually a signal line connected to a GPIO. 

### Bitbangig

For protocols such as I2C or SPI we usually use peripherals of the MCU to convert the data we want to transmit into signals. In some cases, for example if the MCU does not support the protocol or if a non-standard form of the protocol is used, you need to write a program that turns the data into signals manually.  This is called bitbanging. 


