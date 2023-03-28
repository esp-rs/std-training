#!/bin/sh

set -ef

WORK_DIR=/home/esp/workspace/$1

echo "Compiling $1"

cd /home/esp/workspace/$1

if [ -f cfg.toml.example ]; then
    # Rename file to cfg.toml
    cp cfg.toml.example cfg.toml
    # Replace defaults
    sed -i 's/wifi_ssid = "FBI Surveillance Van"/wifi_ssid = "ssid"/g' cfg.toml
    sed -i 's/wifi_psk = "hunter2"/wifi_psk = "pass"/g' cfg.toml
    sed -i 's/mqtt_user = "horse"/mqtt_user = "user"/g' cfg.toml
    sed -i 's/mqtt_pass = "CorrectHorseBatteryStaple"/mqtt_pass = "pass"/g' cfg.toml
    sed -i 's/mqtt_host = "yourpc.local"/mqtt_host = "host"/g' cfg.toml
fi

$HOME/.cargo/bin/cargo clean
$HOME/.cargo/bin/cargo build

# Check examples
if [[ "$1" == advanced/button-interrupt ]]; then
    $HOME/.cargo/bin/cargo build --example solution
    $HOME/.cargo/bin/cargo build --example solution_led
fi

if [[ "$1" == advanced/i2c-sensor-reading ]]; then
    $HOME/.cargo/bin/cargo build --example part_1
    $HOME/.cargo/bin/cargo build --example part_2
fi

if [[ "$1" == intro/http-client ]]; then
    $HOME/.cargo/bin/cargo build --example http_client
    $HOME/.cargo/bin/cargo build --example https_client
fi

if [[ "$1" == intro/http-server ]]; then
    $HOME/.cargo/bin/cargo build --example http_server
fi

if [[ "$1" == intro/mqtt/exercise ]]; then
    $HOME/.cargo/bin/cargo build --example solution_publ_rcv
    $HOME/.cargo/bin/cargo build --example solution_publ
fi
