# HTTP client

The goal of this exercise is to write a small HTTP client that connects to a website.

## Setup

✅ Go to `intro/http-client/exercise` directory.

✅ Open the prepared project skeleton in `intro/http-client/exercise`. 

✅ Add your [network credentials](02_4_hello_board.md) to the `cfg.toml` as in the hardware test. 

✅ Open the docs for this project with the following command:

```
$ cargo doc --open
```

## Making a connection

By default only unencrypted HTTP is available, which rather limits our options of hosts to connect to. We're going to use `http://neverssl.com/`.

In `esp-idf`, HTTP client connections are managed by `http::client::EspHttpClient` in the `esp-idf-svc` crate. It implements the `http::client::Client` trait from `embedded-svc`, which defines functions for [HTTP request methods](https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol#Request_methods) like `GET` or `POST`. This is a good time to have a look at the documentation you opened with `cargo doc --open` for `http::client::EspHttpClient` and see instantiation methods at your disposal.

✅ Add the url `http://neverssl.com/` to the main function. This is the address we will query.

✅ Create a new `EspHttpClient` with default values. Look for a suitable constructor in the documentation.


Calling HTTP functions (e.g. `get(url)`) on this client returns an `EspHttpRequest`, which must be turned into a `Writer` to reflect the client's option to send some data alongside its request. 

After this optional send step the `Writer` can be turned into a `Response` from which the received server output can be read:

The `get` function uses [as_ref()](https://doc.rust-lang.org/std/convert/trait.AsRef.html). This means that instead of being restricted to one specific type like just `String` or just `&str`, the function can accept anything that implements the `AsRef<str>` trait - that is, any type where a call to `.as_ref()` will produce an `&str`. This works for `String` and `&str`, but also the `Cow<str>` enum type which contains either of the previous two.


```Rust
let request = client.get(some_url_ref)?;
// the parameter passed to `into_writer` is the number of bytes
// the client intends to send
let writer = request.into_writer(0)?;
let response = writer.into_response()?;
```
The parameter passed to `into_writer` is the number of bytes the client intends to send. Here we are not trying to send anything. 

A successful response has [a status code in the 2xx range](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes).

✅ Verify the connection was successful.

✅ Return an Error if the status is not in the 2xx range.

```rust
match status {
        200..=299 => {
        }
        _ => anyhow::bail!("unexpected response code: {}", status),
    }
```
The status error can be returned with the [Anyhow](https://docs.rs/anyhow/latest/anyhow/index.html), crate which contains various functionality to simplify application-level error handling. It supplies a universal `anyhow::Result<T>`, wrapping the success (`Ok`) case in T and removing the need to specify the Err type, as long as every error you return implements `std::error::Error`.


✅ Turn your `response` into a `embedded_svc::io::Read` reader by calling `response.reader()` and read the received data chunk by chunk into a `u8` buffer using `reader.do_read(&mut buf)`. `do_read` returns the number of bytes read - you're done when this value is `0`.

✅ Report the total number of bytes read.

✅ Log the received data to the console. Hint, the response in the buffert is in bytes, so you might need [a method](https://doc.rust-lang.org/std/str/fn.from_utf8.html) to convert from bytes to `&str`.

## Extra Tasks

✅ Handle 3xx, 4xx and 5xx status codes each in a separate match arm

✅ Write a custom `Error` enum to represent these errors. Implement the `std::error::Error` trait for your error.


## Troubleshooting

- `error: cannot find macro llvm_asm in this scope`: set `channel = "nightly-2021-11-18"` in `rust-toolchain.toml` (as of February 2022, nightly Rust and the RISC-V ecosystem are somewhat incompatible)
- `missing WiFi name/password`: ensure that you've configured `cfg.toml` according to `cfg.toml.example` - a common problem is that package name and config section name don't match. 

```toml
# Cargo.toml
#...
[package]
name = "http-client"
#...

# cfg.toml
[http-client]
wifi_ssid = "..."
wifi_psk = "..."
```

- `Guru Meditation Error: Core 0 panic'ed (Load access fault). Exception was unhandled.`
    This may caused by an `.unwrap()` in your code. Try replacing those by question marks.