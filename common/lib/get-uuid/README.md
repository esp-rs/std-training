# get-uuid
This is a helper crate to generate a uuid *once* for several projects.

## How it works
Due to cargo's limitations: a bit complicated.
`build.rs` checks for the existence of `uuid.toml` in this package, and if it does not exists, 
creates `uuid.toml` and `_uuid.rs` - both containing the same UUID. The `toml` file is mostly useful
for programs *not* written in Rust (e.g. a Python mqtt client).

## Usage
`uuid()` is a `const` function, meaning you can (and often, have to!) evaluate it at compile time.
Put this in your code:

```rust
const UUID: &'static str = get_uuid::uuid();
```

and refer to `UUID` anywhere in your project.