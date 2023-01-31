#!/usr/bin/env bash

set -xe

BUILD_MODE=""
case "$1" in
    ""|"release")
        bash scripts/build.sh
        BUILD_MODE="release"
        ;;
    "debug")
        bash scripts/build.sh debug
        BUILD_MODE="debug"
        ;;
    *)
        echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
        exit 1;;
esac

export ESP_ARCH=riscv32imc-esp-espidf

# web-flash --chip esp32c3 target/${ESP_ARCH}/${BUILD_MODE}/air-quality-monitor-2
espflash flash -p /dev/tty.usbmodem101 --monitor target/${ESP_ARCH}/${BUILD_MODE}/try-rust-esp32c3-network
