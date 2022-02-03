# A simple HTTP client

In this exercise, we'll retrieve some data over a HTTP connection to the internet.

## Setup
You can find a prepared project skeleton in `intro/http-client/exercise`. It includes establishing a WiFi connection, but you must configure it to use your network's credentials in `cfg.toml`.

## Making a connection

By default only unencrypted HTTP is available, which rather limits our options of hosts to connect to. We're going to use `http://neverssl.com/`.

In `esp-idf`, HTTP client connections are managed by `http::client::EspHttpClient` in the `esp-idf-svc` crate. It implements the `http::client::Client` trait from `embedded-svc`, which defines functions for [HTTP request methods](https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol#Request_methods) like `GET` or `POST`.

TODO currently no `docs.rs` documentation for the svc crates (ping Espressif about this, it's a known issue though)

calling HTTP functions (e.g. `get(url)`) on this client returns an `EspHttpRequest`, which must be turned into a `Writer` to reflect the client's option to send some data alongside its request. This makes more sense with `POST` and `PUT` but must still be performed with `GET`.

After this optional send step the `Writer` can be turned into a `Response` from which the received server output can be read:

```Rust
let request = client.get(...)
// the parameter passed to `into_writer` is the number of bytes the client intends to send
let writer = request.into_writer(0)?;
let response = writer.into_response()?;
```

A successful response has [a status code in the 2xx range](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes).

✅ Verify the connection was successful.
✅ Return an error if the status 
✅ Turn your `response` into a `embedded_svc::io::Read` reader by calling `response.reader()` and read the received data chunk by chunk into a `u8` buffer using `reader.do_read(&mut buf)`. `do_read` returns the number of bytes read - you're done when this value is `0`.
✅ Report the total number of bytes read.
✅ Log the received data to the console.

## Extra Tasks

✅ Handle 3xx, 4xx and 5xx status codes each in a separate match arm
✅ Write a custom `Error` enum to represent these errors. Implement the `std::error::Error` trait for your error.

## HTTPS

TODO check with Espressif: HTTPS support seems buggy - in my (Anatol) tests I get some unexpected HTTP header data before the actual body

To establish a secure, encrypted HTTPS connection, we first need to add some certificates so a server's identity can be verified.

✅ Enable basic TLS certificate support in your project's `sdkconfig.defaults` by deleting the existing `CONFIG_MBEDTLS...` lines and adding:
```cfg
CONFIG_MBEDTLS_CERTIFICATE_BUNDLE=y
CONFIG_MBEDTLS_CERTIFICATE_BUNDLE_DEFAULT_CMN=y
```

Now, we create a custom client configuration to use an `http::client::EspHttpClientConfiguration` which enables the use of these certificates and uses default values for everything else:

```rust
let client_config = EspHttpClientConfiguration {
    use_global_ca_store: true,
    crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
    ..Default::default()
}
```

✅ Initialize your HTTP client with this new configuration and verify HTTPS works by downloading from a `https` resource e.g. `https://espressif.com/`

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