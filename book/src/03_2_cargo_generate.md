# Generating new projects

We're now going to use [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) (a generic project wizard) to set up our first application.

> Most other exercises in this workshop already provide a project skeleton and don't require using `cargo-generate`.

✅ Change to the `intro` directory and run `cargo generate` with the `esp-idf` template:

```shell
$ cd intro
$ cargo generate --git https://github.com/esp-rs/esp-idf-template cargo
```

You'll be prompted for details regarding your new project. When given a choice between several options, navigate using cursor up/down and select with the Return key.

> If you make a mistake, hit `Ctrl+C` and start anew.

✅ Configure your project:

1. Project Name: `hello-world`
2. ESP-IDF native build version: `4.4`
3. STD support: `true`
4. MCU: `esp32c3`
5. Rust toolchain: `nightly`

We're going to build using the `native` variant of the Espressif build system. 

✅ Enable the native build system by opening `Cargo.toml` in your new `hello-world` project and adding `"native"` as default feature:

```toml
[features]
default = ["native"] # add this line
native = ["esp-idf-sys/native"]
```

Optional, but recommended: save some disk space by setting the toolchain directory to global.

✅ Open `hello-world/.cargo/config.toml` and add the following line to the bottom of the `[env]` section, leaving everything else unchanged:

```toml
[env]
# ... 
ESP_IDF_TOOLS_INSTALL_DIR = { value = "global" } # add this line
```

✅ Run your project:
```shell
$ cd hello-world
$ cargo espflash --release --monitor /dev/SERIAL_DEVICE

(...)
I (268) cpu_start: Starting scheduler.
Hello, world!
```
## Troubleshooting
- `Error: Failed to generate bindings`: add `default = ["native"]` to `Cargo.toml`
- if you're using the deprecated `pio` build system, an [initial git commit of your project](https://github.com/espressif/esp-idf/issues/3920) will be required for a successful build.
- if `cargo espflash` is stuck on `Connecting...`, you might have another monitor process still running (e.g. from the initial `hardware-check` test). Try finding and terminating it.
