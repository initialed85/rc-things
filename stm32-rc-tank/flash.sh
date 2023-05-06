#!/usr/bin/env bash

set -e

pushd "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" >/dev/null 2>&1

function finish() {
  popd >/dev/null 2>&1
}
trap finish EXIT

echo -e "\nflashing bootloader...\n"
tools/mac-core2-flasher --speed 460800 bootloader/bootloader_1_0_0_core2.hex

echo -e "\nflashing target code...\n"
tools/mac-core2-flasher --speed 460800 target/main.hex
