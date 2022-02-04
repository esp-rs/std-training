# IoT using MQTT

❗️This exercise requires an MQTT server. If you're participating in a Ferrous Systems training, login credentials for a server operated by Espressif will be made available in the workshop, otherwise you can use one listed at <https://test.mosquitto.org/> or install one locally.

To conclude the introductory course, let's add some [IoT](https://en.wikipedia.org/wiki/Internet_of_things) functionality to the board. 
Our goal here is have it send out real-time updates of sensor values without having to poll repeatedly, like we would with an HTTP server, and also receive commands to change the board LED color.

This can be modeled using a [publish-subscribe architecture](https://en.wikipedia.org/wiki/Publish%E2%80%93subscribe_pattern), where multiple clients publish messages in certain channels/topics, and can also subscribe to these topics to receive messages sent by others. Dispatching of these messages is coordinated by a message broker - in our case, this is done an MQTT server.

## MQTT messages

An MQTT message consists of two parts - topic and payload.

The topic serves the same purpose as an email subject or a label on a filing cabinet, whereas the payload contains the actual data.  The payload data format is not specified, although JSON is common.

🔎 The most recent version of the MQTT standard (MQTT 5) supports content type metadata.

When sending a MQTT message, a Quality of Service (QoS) parameter needs to be defined, indicating delivery guarantees:
- at most once
- at least once
- exactly once.

For the purpose of this exercise it does not matter which quality you choose.
## MQTT topics

MQTT topics are UTF-8 strings representing a hierarchy, with individual levels separated by a `/` slash character. A leading slash is supported but not recommended. Some example topics are:

```code
home/garage/temperature
beacons/bicycle/position
home/alarm/enable
home/front door/lock
```

Here a sensor would periodically publish the garage temperature which then gets broadcast to every subscriber, just as the bicycle beacon publishes its GPS coordinates. The `alarm` and `lock` topics serve as a command sink for specific devices. However, nothing prevents additional subscribers from listening in on these commands, which might provide useful for auditing purposes.

🔎 Topics starting with `$` are reserved for statistics internal to the broker. Typically the topic will begin with `$SYS`. Clients cannot publish to these topics.

❗️Since all workshop participants will be sharing the same MQTT server, some measures are required to prevent crosstalk between different projects. The exercise skeleton will generate a unique, random ID (in the `UUID v4` format) for each repository checkout. You can also [manually generate your own online](https://www.uuidgenerator.net/version4). Your UUID should be used as leading part of the message topics sent between computer and board, roughly resembling this pattern:

```code
6188eec9-6d3a-4eac-996f-ac4ab13f312d/sensor_data/temperature
6188eec9-6d3a-4eac-996f-ac4ab13f312d/command/board_led
```

## Subscribing to topics

A client sends several subscribe messages to indicate they're interested in receiving certain topics. Wildcards are optionally supported, both for a single hierarchy level and as a catch-all:

- `home/garage/temperature` - subscribes only to this specific topic
- `home/#` - the hash character is used as multi-level wildcard and thus subscribes to every topic starting with `home/` - `home/garage/temperature`, `home/front door/lock` and `home/alarm/enable` would all match, but `beacons/bicycle/position` won't. The multi-level wildcard must be placed at the end of a subscription string.
- `home/+/temperature` - the plus character serves as single-level wildcard to subscribe to `home/garage/temperature`, `home/cellar/temperature`, etc.


## Setup

You can find a prepared project skeleton in `intro/mqtt/exercise`. Similar to the http exercises you need to configure your WiFi credentials in `cfg.toml`, but here you'll also need to add MQTT server details.

## Support tools & crates

To send LED commands to the board and also log the sensor values sent by it, a helper client is provided under `intro/mqtt/example_client`. It will send a new random LED color command and also subscribes to the board temperature topic.

The `mqtt_messages` crate (located in `common/lib`) supports handling messages, subscriptions and topics:
### Functions to generate topic strings
- `hello_topic(uuid)` - test topic for initially verifying a successful connection
- `temperature_data_topic(uuid)` - creates a whole "temperature" topic string 
- `cmd_topic_fragment(uuid)` - creates the leading part of a "command" topic (the `a-uuid/command/` part in `a-uuid/command/board_led`)

### Encoding and decoding message payloads


We're going to use a very rudimentary encoding scheme for our data:
- the board temperature `f32` float is converted to four "big endian" bytes using `temp.to_be_bytes()`.
- board LED commands are made of three bytes indicating red, green and blue
    - `enum Command`: represents all possible commands (here: just `BoardLed`)
    - `RawCommandData` stores the last part of a message topic (e.g. `board_led` in `a-uuid/command/board_led`). It can be converted into a `Command` using `try_from`

```rust
// temperature
let temperature_data = temp.to_be_bytes() as &[u8]; // board code
let decoded_temperature = f32::from_be_bytes(temperature_data); // example client code

// RGB LED command
let raw = RawCommandData {
    path: command,
    data: message.data(),
};

if let Ok(cmd) = Command::try_from(raw) { /* set new color here */ }
```
## Establishing a connection

Connections are managed by an instance of `esp_idf_svc::mqtt::client::EspMqttClient`.
It is constructed using
- a broker URL which in turn contains credentials, if necessary
- a configuration of the type `esp_idf_svc::mqtt::client::MqttClientConfiguration`
- a handler closure similar to the http server exercise

```rust
const url = format!("mqtt://{}:{}@{}", username, password, host);
let mqtt_config = MqttClientConfiguration::default();
let mut client = EspMqttClient::new_with_callback(url, &mqtt_config, move |message_event| { 
    // ... your handler code here - leave empty for the first 
    upcoming exercise
};
```
## Publish & Subscribe

`EspMqttClient` is also responsible for subscribing to topics and publishing messages under a given topic.
The `publish` function includes a `retain` parameter indicating whether this message should also be delivered to clients that connect after it has been published. 

```rust
let subscribe_topic = /* ... */;
client.subscribe(subscribe_topic, QoS::AtLeastOnce)

let publish_topic = /* ... */;
let payload: &[u8] = /* ... */ ;
client.publish(publish_topic, QoS::AtLeastOnce, false, payload)?;
```

✅ Create an `EspMqttClient` with a default configuration and an empty handler closure.
✅ Send an empty message under the `hello` topic to the broker. Use the `hello_topic(uuid)` utility function to generate a properly scoped topic.
✅ Verify a successful publish by having a client connected that logs these messages. The `example_client` implements this behavior.
✅ In the loop at the end of your main function, publish the board temperature every second. Verify this, too.

## Handling incoming messages


The `message_event` parameter in the handler closure is of type `Option<Result<Event<EspMqttMessage>>>`.
Since we're only interested in processing successfully received messages, we can make use of deep pattern matching:

```rust
if let Some(Ok(Received(message))) = message_event {
    match message.details() {
        Details::Complete(token) => {
            // all messages in this exercise will be of type `Complete`
            // the other variants of the `Details` enum are for larger message payloads

            let topic: Cow<str> = message.topic(token); // Cow<str> behaves a lot like other Rust strings (&str, String)

            // determine if we're interested in this topic and dispatch based on its content
            let is_command_topic: bool = /* ... */;
            if is_command_topic {
                let raw = RawCommandData { /* ... */ };
                if let Ok(Command::BoardLed(color)) = Command::try_from(raw) {
                    // set the LED to the newly received color
                }
            
            },
        _ => {}
        }
    }
}
```

✅ Subscribe to all "command" messages, combining `cmd_topic_fragment(uuid)` with a trailing `#` wildcard
✅ Verify your subscription is working by logging the received topic and running the `example_client` in parallel. You should receive a board LED command roughly every second.
✅ React to the LED commands by setting the newly received color

## Hints

- `split()` on a string returns an iterator. You can access a specific item from an iterator using [`nth()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth).
- TODO more hints here

## Extra tasks

- leverage [`serde_json`](https://docs.serde.rs/serde_json/) to encode/decode your message data as JSON.
- Send some messages with a large payload from the example client and process them on the microcontroller. Large messages will be delivered in parts instead of `Details::Complete`:
```rust
InitialChunk(chunk_info) => { /* first chunk */},
SubsequentChunk(chunk_data) => { /* all subsequent chunks */ }
```
You do not need to differentiate incoming chunks based on message ID, since at most one message will be in flight at any given time. 

## Troubleshooting

- `error: expected expression, found .` when building example client: update your stable Rust installation to 1.58 or newer
- MQTT messages not showing up? make sure all clients (board and workstation) use the same UUID (you can see it in the log output)
