# Workshop repository

The entire material can be found at <https://github.com/ferrous-systems/espressif-trainings>.

✅ Clone and change into the workshop repository:

```console
$ git clone "https://github.com/ferrous-systems/espressif-trainings.git"
$ cd espressif-trainings
```

❗ Windows users may have problems with long path names. Follow these steps to substitute the path:

```console
git clone git clone https://github.com/ferrous-systems/espressif-trainings.git
subst r:\ espressif-trainings
cd r:\
```

## Repository contents

- `advanced/` - code examples and exercises for the advanced course
- `book/` - markdown sources of this book
- `common/` - code shared between both courses
- `common/lib/` - support crates
- `common/lib/esp32-c3-dkc02-bsc` - board support crate (bsc) for the `ESP32-C3-DevKitC-02` board
- `common/vendor/` - third party crates that have been forked to add required support, pending upstream merges 
- `extra/` - tools not required for this training which might still be useful
- `intro/` - code examples and exercises for the introduction course


## A word on configuration

We use [toml-cfg](https://github.com/jamesmunns/toml-cfg) throughout this workshop as a more convenient and secure alternative to putting credentials or other sensitive information directly in source code: the settings are stored in a file called `cfg.toml` in the respective package root instead

This configuration contains exactly one section header which has the same name as your package (`name = "your-package"` in `Cargo.toml`), and the concrete settings will differ between projects:

```toml
[your-package]
user = "example"
password = "h4ckm3"
```

If you copy a `cfg.toml` to a new project, remember to change the header to `[name-of-new-package]`.