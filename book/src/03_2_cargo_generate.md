# Generating new projects

We're now going to use [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) (a generic project wizard) to set up our first application.

> Most other exercises in this workshop already provide a project skeleton and don't require using `cargo-generate`.

✅ Change to the `intro` directory and run `cargo generate` with the `esp-idf` template:

```shell
$ cd intro
$ cargo generate --git https://github.com/esp-rs/esp-idf-template cargo
```

You'll be prompted for details regarding your new project. When given a choice between several options, navigate using cursor up/down and select with the Return key.

The first message you see will be:
`⚠️Unable to load config file: /home/$USER/.cargo/cargo-generate.toml`. You see this error because you do not have a favorite config file, but you don't need one and you can ignore this warning.

🔎 You can create a [favorite config file](https://cargo-generate.github.io/cargo-generate/favorites.html) that will be placed in `$CARGO_HOME/cargo-generate`, and override it with `-c, --config <config-file>`. 


> If you make a mistake, hit `Ctrl+C` and start anew.

✅ Configure your project:

(These items may appear in a different order)

* Project Name: `hello-world`
* Rust toolchain: `nightly`
* MCU: `esp32c3`
* ESP-IDF native build version: `4.4`
* STD support: `true`

You can also skip `cargo-generate` questions, by provinding all the needed information in one line:

```toml
cargo generate --vcs none --git https://github.com/esp- rs/esp-idf-template cargo --name rust-project --define mcu=esp32c3 --define toolchain=nightly --define espidfver=v4.4 --define std=true
```


We're going to build using the `native` variant of the Espressif build system.

✅ Enable the native build system by opening `Cargo.toml` in your new `hello-world` project and adding `"native"` as default feature:

```toml
[features]
default = ["native"] # add this line
native = ["esp-idf-sys/native"]
```

🔎 `.cargo/config.toml` contains local settings ([list of all settings](https://doc.rust-lang.org/cargo/reference/config.html)) for your package. 
`Cargo.toml` contains dependencies [import all your dependencies](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html).


Optional, but recommended: To save disk space and download time, set the toolchain directory to global - otherwise each new project/workspace will have its own instance of the toolchain installed on your computer.


✅ Open `hello-world/.cargo/config.toml` and add the following line to the bottom of the `[env]` section. Leave everything else unchanged.

```toml
[env]
# ... 
ESP_IDF_TOOLS_INSTALL_DIR = { value = "global" } # add this line
```

✅ Run your project by using the following command out of the `hello_world` directory.

```shell
$ cd hello-world
$ cargo espflash --release --monitor /dev/SERIAL_DEVICE
```

✅ The last lines of your output should look like this:

```shell
(...)
I (268) cpu_start: Starting scheduler.
Hello, world!
```

## Extra tasks
- If your main function exits, you have to reset the microcontroller to start it again. What happens when you put an infinite loop at the end instead? Test your theory by flashing a looping program.
- Can you think of a way to prevent what you're now seeing? (click for hint:[^hint])

## Troubleshooting
- `⛔ Git Error: authentication required`: your git configuration is probably set to override `https` github URLs to `ssh`. Check your global `~/.git/config` for `insteadOf` sections and disable them.
- `Error: Failed to generate bindings`: add `default = ["native"]` to `Cargo.toml`
- if you're using the deprecated `pio` build system, an [initial git commit of your project](https://github.com/espressif/esp-idf/issues/3920) will be required for a successful build.
- if `cargo espflash` is stuck on `Connecting...`, you might have another monitor process still running (e.g. from the initial `hardware-check` test). Try finding and terminating it. If this doesn't help, disconnect and reconnect the board's USB cable.

[^hint]: yield control back to the underlying operating system by `sleep`ing in a loop instead of busy waiting. (use `use std::thread::sleep`)