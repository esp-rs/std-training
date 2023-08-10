# Reference

## GPIO

GPIO is short for General Purpose Input Output. GPIOs are digital (or sometimes analog) signal pins that can be used as interfaces to other systems or devices. Each pin can be in various states, but they will have a default state on power-up or after a system reset (usually a harmless one, like being a digital input). We can then write software to change them into the appropriate state for our needs.

We'll introduce a couple of concepts related to GPIOs:

### Pin Configurations

GPIOs can be configured in one of several ways. The options available can vary depending on the design of the chip, but will usually include:

Floating: A floating pin is neither connected to VCC nor Ground. It just floats around at whatever voltage is applied. Note though, that your circuit should externally pull the pin either low or high, as CMOS silicon devices (such as microcontrollers) can fail to work correctly if you leave a pin higher than the 'low voltage threshold' or `Vtl`, but lower than the 'high voltage threshold' or `Vth` for more than a few microseconds.

Push-Pull-Output: A pin that is configured as push–pull output can then be set to either drive a high voltage onto the pin (i.e. connect it to VCC), or a low voltage onto the pin (i.e. connect it to Ground). This is useful for LEDs, buzzers or other devices that use small amounts of power.

Open-Drain-Output: Open Drain outputs switch between "disconnected" and "connected to ground." It is expected that some external resistor will weakly pull the line up to VCC. This type of output is designed to allow multiple devices to be connected together - the line is 'low' if any of the devices connected to the line drive it low. If two or more devices drive it low at the same time, no damage occurs (connecting Ground to Ground is safe). If none of them drive it low, the resistor will pull it high by default.

Floating-Input: A pin where the external voltage applied can be read, in software, as either a `1` (usually if the voltage is above some threshold voltage) or a `0` (if it isn't). The same warnings apply per the 'Floating' state.

Pull-Up-Input: Like a Floating-Input, except an internal 'pull-up' resistor weakly pulls the line up to VCC when nothing external is driving it down to Ground. Useful for reading buttons and other switches, as it saves you from needing an external resistor.

### Active High/Low

A digital signal can be in two states: `high` and `low`. This is usually represented by the voltage difference between the signal and ground. It is arbitrary which of these voltage levels represents which logic states: So both `high` and `low` can be defined as an active state.

For example, an active high pin has voltage when the logic level is active, and, an active low pin has voltage when the logic level is set to inactive.

In embedded Rust, abstractions show the logic level and not the voltage level. So if you have an active low pin connected to an LED, you need to set it to inactive for the LED to light up.

### Chip Select

Chip Select is a binary signal to another device that can switch that device on or off, either partially or entirely. It is frequently a signal line connected to a GPIO, and commonly used to allow multiple devices to be connected to the same SPI bus - each device only listens when its Chip Select line is active.

### Bit Banging

For protocols such as I2C or SPI, we usually use peripherals within the MCU to convert the data we want to transmit into signals. Sometimes, for example, if the MCU doesn't support the protocol or if a non-standard form of the protocol is used, you need to write a program that turns the data into signals manually. This is called bit-banging.


