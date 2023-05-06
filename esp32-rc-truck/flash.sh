#!/usr/bin/env bash

set -e

pushd "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" >/dev/null 2>&1

function finish() {
  popd >/dev/null 2>&1
}
trap finish EXIT

echo -e "\nflashing target code...\n"
espflash --speed 460800 /dev/cu.usbserial-* target/xtensa-esp32-espidf/debug/esp32-rc-truck
