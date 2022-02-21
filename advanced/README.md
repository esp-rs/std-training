# Advanced Folder Code Organization

Folders are listed in alphabetical order, not in the order they are taught in the course.

## Button Interrupt

TODO

## I2c driver exercise

TODO `exercise/src/imc42670p.rs` is a gap text of a very basic i2c IMU sensor driver. The task is to complete the file, so that running `main.rs` will log the device ID of the driver. The this gap text driver is based on the version of the same name that lives in common, but provides a little bit more functionality.

TODO `exercise/src/main.rs` contains working code. Running it without modifying the driver file will yield errors. 

`solution/src/imc42670p.rs` provides a solution to the task. 

## I2c sensor exercise

TODO `exercise/src/main.rs` is an exercise skeleton that will build. The task is to use an existing driver from crates.io to read out the temperature and humidity sensor over i2c. After that, a second sensor will be read out over the same i2c bus using `shared-bus`. The driver for the second sensor is available locally in `common/`.

`solution/src/main.rs` contains a working solution.

