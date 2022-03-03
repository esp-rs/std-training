# A simple HTTP server

We're now turning our board into a tiny web server that upon receiving a `GET` request serves data from the internal temperature sensor. 

## Setup

You can find a prepared project skeleton in `intro/http-server/exercise`. It includes establishing a WiFi connection, but you must configure it to use your network's credentials in `cfg.toml`.

## Serving requests

To connect to your board with your browser, you need to know the board's IP address. 


✅ Run the skeleton code. The output should yield the board's IP address like this:

```console
I (3862) esp_netif_handlers: sta ip: 192.168.178.54, mask: ...
```

The `sta ip` is the "station", the WiFi term for an interface connected to an access point. This is the address you'll put in your browser (or other http client like `curl`).

> esp-idf tries to register the hostname `espressif` in your local network, so often `http://espressif/` instead of `http://<sta ip>/` will also work.
>
> You can change the hostname by setting `CONFIG_LWIP_LOCAL_HOSTNAME` in `sdkconfig.defaults`, e.g.: `CONFIG_LWIP_LOCAL_HOSTNAME="esp32c3"`

Sending HTTP data to a client involves:
- creating an an instance of `EspHttpServer`
- looping in the main function so it doesn't terminate - termination would result in the server going out of scope and subsequently shutting down
- setting a separate request `handler` function for each requested path you want to serve content. Any unconfigured path will result in a `404` error. These handler functions are realized inline as Rust closures via:

```rust
server.set_inline_handler(path, method, |request, response| {
    // the `response` needs to write data to the client
    let mut writer = response.into_writer(request);

    // construct a response
    let some_buf = ...;

    // now you can write your desired data
    writer.do_write_all(&some_buf);

    // once you're done the handler expects a `Completion` as result,
    // this is achieved via:
    writer.complete()
});

```
 

✅ Create a `EspHttpServer` instance using a default `esp_idf_svc::http::server::Configuration`. The default configuration will cause it to listen on port 80 automatically. 

✅ Verify that a connection to `http://<sta ip>/` yields a `404` (not found) error stating `This URI does not exist`.

✅ Write a request handler for `GET` requests to the root path (`"/"`). The request handler sends a greeting message at `http://<sta ip>/`, using the provided `index_html()` function to generate the HTML String.

## Dynamic data

We can also report dynamic information to a client. The skeleton includes a configured `temp_sensor` that measures the board's internal temperature. 

✅ Write a second handler that reports the chip temperature at `http://<sta ip>/temperature`, using the provided `temperature(val: f32)` function to generate the HTML String.
## Hints
- if you want to send a response string, it needs to be converted into a `&[u8]` slice via `a_string.as_bytes()`
- the temperature sensor needs exclusive (mutable) access. Passing it as owned value into the handler will not work (since it would get dropped after the first invocation) - you can fix this by making the handler a `move ||` closure, wrapping the sensor in an `Arc<Mutex<_>>`, keeping one `clone()` of this `Arc` in your main function and moving the other into the closure.