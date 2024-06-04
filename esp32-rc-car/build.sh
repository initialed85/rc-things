#!/usr/bin/env bash

set -e

pushd "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" >/dev/null 2>&1

function finish() {
  popd >/dev/null 2>&1
}
trap finish EXIT

echo -e "\nrunning native build...\n"
CRATE_CC_NO_DEFAULTS=1 cargo build --release

echo -e "\nbuilt artifacts:\n"

find "$(pwd)/target/xtensa-esp32-espidf/debug/esp32-rc-car"
