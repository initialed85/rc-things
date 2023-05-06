#!/usr/bin/env bash

set -e

pushd "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" >/dev/null 2>&1

function finish() {
  popd >/dev/null 2>&1
}
trap finish EXIT

rc-client/build.sh
stm32-rc-tank/build.sh
esp32-rc-car/build.sh
pi-rc-tank/build.sh
