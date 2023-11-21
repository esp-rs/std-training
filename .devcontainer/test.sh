#!/bin/sh

set -ef

echo "Compiling $1"

cd /home/esp/workspace/$1

if [ -f cfg.toml.example ]; then
    # Rename file to cfg.toml
    cp cfg.toml.example cfg.toml
    # Replace defaults
    sed -i 's/wifi_ssid = "FBI Surveillance Van"/wifi_ssid = "Wokwi-GUEST"/g' cfg.toml
    sed -i 's/wifi_psk = "hunter2"/wifi_psk = ""/g' cfg.toml
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
    # Simulate with Wokwi
    sed -i 's/^[[:space:]]*firmware[[:space:]]*=[[:space:]]*["'"'"']\([^"'"'"']*\)["'"'"']\([[:space:]]*\)$/\nfirmware = "target\/riscv32imc-esp-espidf\/debug\/examples\/solution"/' wokwi.toml
fi

if [[ "$1" == advanced/i2c-sensor-reading ]]; then
    $HOME/.cargo/bin/cargo build --example part_1
    $HOME/.cargo/bin/cargo build --example part_2
fi

if [[ "$1" == intro/http-client ]]; then
    $HOME/.cargo/bin/cargo build --example http_client
    $HOME/.cargo/bin/cargo build --example https_client
    # Simulate with Wokwi
    sed -i 's/^[[:space:]]*firmware[[:space:]]*=[[:space:]]*["'"'"']\([^"'"'"']*\)["'"'"']\([[:space:]]*\)$/\nfirmware = "target\/riscv32imc-esp-espidf\/debug\/examples\/http_client"/' wokwi.toml
fi

if [[ "$1" == intro/http-server ]]; then
    $HOME/.cargo/bin/cargo build --example http_server
fi

if [[ "$1" == intro/mqtt/exercise ]]; then
    $HOME/.cargo/bin/cargo build --example solution_publ_rcv
    $HOME/.cargo/bin/cargo build --example solution_publ
fi
