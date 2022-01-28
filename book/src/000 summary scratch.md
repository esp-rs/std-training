- Intro
    - Hello, world!
        - log msg
        - LED: show wifi status
    - Hello, template!
    - HTTP client
    - MQTT client
        - Hello, MQTT!
        - dispatch commands
- [Advanced](./0x_advanced.md)
    - GPIO
    - Low level I/O
    - I2C
    - Interrupts
    - RGB LED driver
    - Watchdog vs. blocking I/O, threads, ?async?

no_std: `$ rustup target add riscv32imc-unknown-none-elf`
pin nightly