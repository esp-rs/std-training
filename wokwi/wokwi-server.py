#!/usr/bin/env python

import asyncio
import base64
import json
import sys
import os
import websockets
import webbrowser


PORT = 9012


def base64_file(path: str):
    with open(path, 'rb') as file:
        return base64.b64encode(file.read()).decode('ascii')


async def hello(websocket, path):
    msg = await websocket.recv()
    print("Client connected! {}".format(msg))

    # Send the simulation payload
    await websocket.send(json.dumps({
        "type": "start",
        "elf": base64_file('{}/wokwi/dummy.elf'.format(os.getcwd())),
        "espBin": [
            [0x0000, base64_file('{}/wokwi/esp32c3_bootloader.bin'.format(os.getcwd()))],
            [0x8000, base64_file('{}/wokwi/esp32c3_partition-table.bin'.format(os.getcwd()))],
            [0x10000, base64_file('{}/app.bin'.format(os.getenv('CURRENT_PROJECT')))],
        ]
    }))

    while True:
        msg = await websocket.recv()
        msgjson = json.loads(msg)
        if msgjson["type"] == "uartData":
            sys.stdout.buffer.write(bytearray(msgjson["bytes"]))
            sys.stdout.flush()
        else:
            print("> {}".format(msg))

start_server = websockets.serve(hello, "127.0.0.1", PORT)
asyncio.get_event_loop().run_until_complete(start_server)

# ESP32-C3-DevKitC-02
# board = 325149339656651346
# ESP32C3 Rust Board
board = 328638850887844436
if "intro/hardware-check" in os.getenv('CURRENT_PROJECT') or "intro/mqtt" in os.getenv('CURRENT_PROJECT') or "advanced/button-interrupt" in os.getenv('CURRENT_PROJECT'):
    # ESP32C3 Rust Board with Neopixel
    board = 328904135759888980
if(os.getenv('USER') == "gitpod"):
    gp_url = subprocess.getoutput("gp url {}".format(PORT))
    gp_url = gp_url[8:]
    url = "https://wokwi.com/_alpha/wembed/{}?partner=espressif&port={}&data=demo&_host{}".format(board,PORT,gp_url)
else:
    url = "https://wokwi.com/_alpha/wembed/{}?partner=espressif&port={}&data=demo".format(board,PORT)
print("Web socket listening on port {}".format(PORT))
print("")
print("Please, open the following URL: {}".format(url))
webbrowser.open(url)
asyncio.get_event_loop().run_forever()