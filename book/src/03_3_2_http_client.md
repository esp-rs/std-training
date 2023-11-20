# HTTP Client

The goal of this exercise is to write a small HTTP client that connects to a website.

## Setup

✅ Go to `intro/http-client` directory.

✅ Open the prepared project skeleton in `intro/http-client`.

✅ Add your [network credentials](02_4_hello_board.md) to the `cfg.toml` as in the hardware test.

✅ Open the docs for this project with the following command:

```console
cargo doc --open
```

`intro/http-client/examples/http_client.rs` contains the solution. You can run it with the following command:

```console
cargo run --example http_client
```
## Making a Connection

By default, only unencrypted HTTP is available, which rather limits our options of hosts to connect to. We're going to use `http://neverssl.com/`.

In ESP-IDF, HTTP client connections are managed by `http::client::EspHttpClient` in the `esp-idf-svc` crate. It implements the `http::client::Client` trait from `embedded-svc`, which defines functions for [HTTP request methods](https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol#Request_methods) like `GET` or `POST`. This is a good time to have a look at the documentation you opened with `cargo doc --open` for `esp_idf_svc::http::client::EspHttpConnection` and `embedded_svc::http::client::Client`. See instantiation methods at your disposal.

✅ Create a new `EspHttpConnection` with default configuration. Look for a suitable constructor in the documentation.

✅ Get a client from the connection you just made.

Calling HTTP functions (e.g. `get(url)`) on this client returns an `embedded_svc::http::client::Request`, which must be submitted to reflect the client's option to send some data alongside its request.

The `get` function uses [as_ref()](https://doc.rust-lang.org/std/convert/trait.AsRef.html). This means that instead of being restricted to one specific type like just `String` or just `&str`, the function can accept anything that implements the `AsRef<str>` trait. That is any type where a call to `.as_ref()` will produce a `&str`. This works for `String` and `&str`, but also the `Cow<str>` enum type which contains either of the previous two.


```rust
let request = client.get(url.as_ref())?;
let response = request.submit()?;
```

A successful response has [a status code in the 2xx range](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes). Followed by the raw html of the website.

✅ Verify the connection was successful.

✅ Return an Error if the status isn't in the 2xx range.

```rust
match status {
        200..=299 => {
        }
        _ => bail!("Unexpected response code: {}", status),
    }
```
The status error can be returned with the [Anyhow](https://docs.rs/anyhow/latest/anyhow/index.html), crate which contains various functionality to simplify application-level error handling. It supplies a universal `anyhow::Result<T>`, wrapping the success (`Ok`) case in T and removing the need to specify the Err type, as long as every error you return implements `std::error::Error`.


✅ Read the received data chunk by chunk into an `u8` buffer using `Read::read(&mut reader,&mut buf)`. `Read::read` returns the number of bytes read - you're done when this value is `0`.

✅ Report the total number of bytes read.

✅ Log the received data to the console.
💡 The response in the buffer is in bytes, so you might need [a method](https://doc.rust-lang.org/std/str/fn.from_utf8.html) to convert from bytes to `&str`.

## Extra Tasks

✅ Handle 3xx, 4xx and 5xx status codes each in a separate match arm

✅ Write a custom `Error` enum to represent these errors. Implement the `std::error::Error` trait for your error.


## Simulation

This project is available for simulation through two methods:
- Wokwi projects:
  - [Exercise](https://wokwi.com/projects/360722140931768321?build-cache=disable)
  - [Solution](https://wokwi.com/projects/333372159510446675?build-cache=disable)
- Wokwi files are also present in the project folder to simulate it with Wokwi VS Code extension:
   1. Press F1, select `Wokwi: Select Config File` and choose `intro/http-client/wokwi.toml`
      - Edit the `wokwi.toml` file to select between exercise and solution simulation
   2. Build you project
   3. Press F1 again and select `Wokwi: Start Simulator`

## Troubleshooting

- `missing WiFi name/password`: ensure that you've configured `cfg.toml` according to `cfg.toml.example` - a common problem is that the package name and config section name don't match.

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
    This may be caused by an `.unwrap()` in your code. Try replacing those with question marks.
