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

if [ "${USER}" == "gitpod" ];then
    gp_url=$(gp url 9012)
    echo "gp_url=${gp_url}"
    export WOKWI_HOST=${gp_url:8}
elif [ "${CODESPACE_NAME}" != "" ];then
    export WOKWI_HOST=${CODESPACE_NAME}-9012.githubpreview.dev
fi

export ESP_ARCH=riscv32imc-esp-espidf

WOKWI_PROJECT_ID=""
ELF_FILE=""
if [[ "$2" == "intro/hardware-check" ]]; then
    WOKWI_PROJECT_ID="334080865809203794"
    ELF_FILE="hardware-check"
elif [[ "$2" =~ "intro/http-client" ]]; then
    WOKWI_PROJECT_ID="333372159510446675"
    ELF_FILE="http-client"
elif [[ "$2" =~ "intro/http-server" ]]; then
    WOKWI_PROJECT_ID="334083021941506642"
    ELF_FILE="http-server"
elif [[ "$2" =~ "intro/mqtt" ]]; then
    WOKWI_PROJECT_ID="333374379294458451"
    ELF_FILE="mqtt"
elif [[ "$2" =~ "advanced/button-interrupt" ]]; then
    WOKWI_PROJECT_ID="333374799393849940"
    ELF_FILE="button-interrupt"
elif [[ "$2" =~ "advanced/i2c-driver" ]]; then
    WOKWI_PROJECT_ID="333375074521317970"
    ELF_FILE="i2c-driver-exercise"
elif [[ "$2" =~ "advanced/i2c-sensor-reading" ]]; then
    WOKWI_PROJECT_ID="333375908055351891"
    ELF_FILE="i2c-sensor-exercise"
fi


if [ "${WOKWI_PROJECT_ID}" == "" ]; then
    wokwi-server --chip esp32c3 $2/target/${ESP_ARCH}/${BUILD_MODE}/${ELF_FILE}
else
    wokwi-server --chip esp32c3 --id ${WOKWI_PROJECT_ID} target/${ESP_ARCH}/${BUILD_MODE}/${ELF_FILE}
fi
