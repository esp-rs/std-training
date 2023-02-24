#!/bin/sh

set -ef

WORK_DIR=/workspace/$1

echo "Compiling $1"

cd /workspace/$1
$HOME/.cargo/bin/cargo clean
$HOME/.cargo/bin/cargo build

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
    # TODO: Update sdkconfig.defaults before buidling
    # $HOME/.cargo/bin/cargo build --example https_client
fi

if [[ "$1" == intro/http-server ]]; then
    $HOME/.cargo/bin/cargo build --example http_server
fi

if [[ "$1" == intro/mqtt/exercise ]]; then
    $HOME/.cargo/bin/cargo build --example solution_publ_rcv
    $HOME/.cargo/bin/cargo build --example solution_publ
fi
