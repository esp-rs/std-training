# Advanced Folder Code Organization

Folders are listed in alphabetical order, not in the order they are taught in the course.

## Button Interrupt

[`button-interrupt/exercise/src/main.rs`](button-interrupt/src/main.rs) contains the project code skeleton
[`button-interrupt/examples/solution.rs`](button-interrupt/examples/solution.rs) contains the solution for the first step of the exercise.
[`button-interrupt/examples/solution_led.rs`]((button-interrupt/examples/solution_led.rs) ) contains the solution for the second step of the exercise.
## I2C Driver exercise (WIP)

[`i2c-driver/exercise/src/icm42670p.rs`](i2c-driver/src/icm42670p.rs) will be gap text of a very basic i2c IMU sensor driver. The task is to complete the file, so that running `main.rs` will log the device ID of the driver. The this gap text driver is based on the version of the same name that lives in common, but provides a little bit more functionality.

[`i2c-driver/exercise/src/main.rs`](i2c-driver/src/main.rs) will contain working code. Running it without modifying the driver file will yield errors.

[`i2c-driver/exercise/src/icm42670p_solution.rs`](i2c-driver/src/icm42670p_solution.rs) provides a solution to the task.

## I2C Sensor Reading Exercise (WIP)

[`i2c-sensor-reading/src/main.rs`](i2c-sensor-reading/src/main.rs) will be an exercise skeleton that will build. The task is to use an existing driver from crates.io to read out the temperature and humidity sensor over i2c. After that, a second sensor will be read out over the same i2c bus using `shared-bus`.

`i2c-sensor-reading/examples/part_1.rs` contains a working solution for Part 1.
`i2c-sensor-reading/examples/part_2.rs` contains a working solution for Part 2.

