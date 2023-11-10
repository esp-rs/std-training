# Workshop Repository

The entire material can be found at <https://github.com/esp-rs/std-training>.

✅ Clone and change into the workshop repository:

```console
git clone "https://github.com/esp-rs/std-training.git"
cd std-training
```

❗ Windows users may have problems with [long path names][windows-long-path].

[windows-long-path]: https://esp-rs.github.io/book/misc/troubleshooting.html#long-path-names

## Repository Contents

- `advanced/` - code examples and exercises for the advanced course
- `book/` - markdown sources of this book
- `common/` - code shared between both courses
- `common/lib/` - support crates
- `intro/` - code examples and exercises for the introduction course


## A Word on Configuration

We use [`toml-cfg`](https://github.com/jamesmunns/toml-cfg) throughout this workshop as a more convenient and secure alternative to putting credentials or other sensitive information directly in source code. The settings are stored in a file called `cfg.toml` in the respective package root instead.

This configuration contains exactly one section header which has the same name as your package (`name = "your-package"` in `Cargo.toml`), and the concrete settings will differ between projects:

```toml
[your-package]
user = "example"
password = "h4ckm3"
```

❗ If you copy a `cfg.toml` to a new project, remember to change the header to `[name-of-new-package]`.
