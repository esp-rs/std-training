# Python MQTT client for MQTT exercise
- sends random RGB LED color commands
- sends random long garbage packets
- logs temperature values sent by MCU

## Setup
- Configure MQTT credentials in `../../intro/mqtt/exercise` according to instructions provided in the workshop 
- set up a Python environment:

```shell
# setup (once):
$ python3 -m venv ve
$ source ./ve/bin/activate
$ pip3 install paho-mqtt toml

# setup (every time you open a new shell):
$ source ./ve/bin/activate
```

# Run
```shell
$ ./mqtt.py
```
