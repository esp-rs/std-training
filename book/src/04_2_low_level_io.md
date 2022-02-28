# Low level I/O How to manipulate Registers

## How does the mapping from register to software work?

Registers and their fields on a device are described in system view description (svd) files. `svd2rust` is used to generate peripheral access crates (PACs) from them. PACS provide a thin wrapper over the various memory-wrapper registers defined for the particular part-number of micro-controller you are using. While it is possible to write code with a PAC alone, a lot of it would be unsafe or otherwise inconvenient as it only provides the basic perhipherals of the microcontroller and not any other that may be on your specific development kit. So there is another layer, the Hardware Abstraction Layer (HAL). HALs provide a more user friendly API for your specific device, and often implement common traits defined in the embedded-hal. 


TODO svd2rust, register manipulation edge vs level triggered interrupts, (atomics emulated via ISR -> can't use in own ISR)

## 

## Register Manipulation with C bindings: Configuring a GPIO

Pins are configured with the `c struct` `gpio_config_t`. The struct has the following fields:

* `pin_bit_mask`: represents the Pin number
* `mode`: sets the mode of the pin, it can have the following settings:
  * `gpio_mode_t_GPIO_MODE_INPUT`
  * `gpio_mode_t_GPIO_MODE_OUTPUT`
  * `gpio_mode_t_GPIO_MODE_DISABLE`
  * `gpio_mode_t_GPIO_MODE_OUTPUT_OD`
  * `gpio_mode_t_GPIO_MODE_INPUT_OUTPUT`
  * `gpio_mode_t_GPIO_MODE_INPUT_OUTPUT_OD`

They are constants with numbers representing the bit that must be set in the corresponding register. 

* `pull_up_en`: true.into(), if the GPIO is pulled up,
* `pull_down_en`: true.into(), if the GPIO is pulled down,
* `intr_type`: sets the interrupt type, it can have the following settings:
  * `gpio_int_type_t_GPIO_INTR_MAX`
  * `gpio_int_type_t_GPIO_INTR_ANYEDGE`
  * `gpio_int_type_t_GPIO_INTR_DISABLE`
  * `gpio_int_type_t_GPIO_INTR_NEGEDGE`
  * `gpio_int_type_t_GPIO_INTR_POSEDGE`



example pin configuration: 

TODO Change to a different example, not the one from interrupt exercise
TODO Add verbal description of configuration

```rust
pub struct gpio_config_t {
    pub pin_bit_mask: u64,
    pub mode: gpio_mode_t,
    pub pull_up_en: gpio_pullup_t,
    pub pull_down_en: gpio_pulldown_t,
    pub intr_type: gpio_int_type_t,
}

let io_conf = gpio_config_t {
        pin_bit_mask: 1 << 9,
        mode: gpio_mode_t_GPIO_MODE_INPUT,
        pull_up_en: true.into(),
        pull_down_en: false.into(),
        intr_type: gpio_int_type_t_GPIO_INTR_POSEDGE, // positive edge trigger = button down
    };
```

