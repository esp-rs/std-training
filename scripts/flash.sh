#!/usr/bin/env bash

set -e

BUILD_MODE=""
case "$1" in
    ""|"release")
        bash /workspace/scripts/build.sh
        BUILD_MODE="release"
        ;;
    "debug")
        bash /workspace/scripts/build.sh debug
        BUILD_MODE="debug"
        ;;
    *)
        echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
        exit 1;;
esac

export ESP_ARCH=riscv32imc-esp-espidf

ELF_FILE=""
if [[ "$2" == "intro/hardware-check" ]]; then
    ELF_FILE="hardware-check"
elif [[ "$2" =~ "intro/http-client" ]]; then
    ELF_FILE="http-client"
elif [[ "$2" =~ "intro/http-server" ]]; then
    ELF_FILE="http-server"
elif [[ "$2" =~ "intro/mqtt" ]]; then
    ELF_FILE="mqtt"
elif [[ "$2" =~ "advanced/button-interrupt" ]]; then
    ELF_FILE="button-interrupt"
elif [[ "$2" =~ "advanced/i2c-driver" ]]; then
    ELF_FILE="i2c-driver-exercise"
elif [[ "$2" =~ "advanced/i2c-sensor-reading" ]]; then
    ELF_FILE="i2c-sensor-exercise"
fi
web-flash --chip esp32c3 target/${ESP_ARCH}/${BUILD_MODE}/${ELF_FILE}
