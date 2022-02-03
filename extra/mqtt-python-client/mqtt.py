#!/usr/bin/env python

from time import sleep
from random import randint
from struct import unpack

import paho.mqtt.client as mqtt
import toml

uuid = None


def get_uuid():
    d = toml.load(open("../../common/lib/get-uuid/uuid.toml", "r"))
    return list(d.values())[0]["uuid"]


def get_config():
    d = toml.load(open("../../intro/mqtt/exercise/cfg.toml", "r"))
    return list(d.values())[0]


def on_connect(client, userdata, flags, rc):
    print("Connected with result code " + str(rc))
    data_topic = f"{uuid}/sensor_data/#"
    # Subscribing in on_connect() means that if we lose the connection and
    # reconnect then subscriptions will be renewed.

    # this would subscribe to a lot of system messages - interesting but noisy!
    # client.subscribe("$SYS/#")

    client.subscribe(data_topic)


def on_message(client, userdata, msg):
    [mcu_temp] = unpack(">f", msg.payload)
    print(f"{msg.topic} {mcu_temp:.1f}")


def msg():
    color = [randint(0, 255), randint(0, 255), randint(0, 255)]
    return f"{uuid}/command/board_led", bytearray(color)


def connect():
    global uuid
    uuid = get_uuid()
    config = get_config()
    client = mqtt.Client()
    client.username_pw_set(config["mqtt_user"], config["mqtt_pass"])
    client.on_connect = on_connect
    client.on_message = on_message
    client.connect(config["mqtt_host"], 1883, 60)
    client.loop_start()
    garbage = "1234567890abcdef01234567890abcdef" * 190 + "ZZZ"
    while True:
        # client.publish(topic, data)
        cmd_topic, data = msg()
        print(f"set color {list(data)}")
        client.publish(cmd_topic, data)
        client.publish(f"{uuid}/cmd/garbage", garbage)
        sleep(1)


if __name__ == "__main__":
    connect()
