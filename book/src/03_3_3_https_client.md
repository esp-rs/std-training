# HTTPS Client

You will now make changes to your HTTP client files so that it also works for encrypted connections.

`intro/http-client/examples/http_client.rs` contains the solution. You can run it with the following command:

```shell
cargo run --example https_client
```

Create a custom client configuration to use an `esp_idf_svc::http::client::EspHttpConnection` which enables the use of these certificates and uses default values for everything else:

```rust
let connection = EspHttpConnection::new(&Configuration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    }
```

✅ Initialize your HTTP client with this new configuration and verify HTTPS works by downloading from an `https` resource, e.g. `https://espressif.com/`. the download will show as raw HTML in the terminal output.

## Troubleshooting (Repeated from Previous Section)

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
