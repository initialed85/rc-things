#!/usr/bin/env bash

set -e

pushd "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" >/dev/null 2>&1

function finish() {
  popd >/dev/null 2>&1
}
trap finish EXIT

cd ../

echo -e "\nrunning docker build...\n"
docker build \
  -t initialed85/rc-things-pi-rc-tank:latest \
  -f docker/pi-rc-tank/Dockerfile \
  .

if test -e target; then
  rm -f target/*
fi

cd pi-rc-tank

echo -e "\nextracting built artifacts...\n"
rm -fr target
docker run --rm -it -v "$(pwd)/target:/srv/artifacts" initialed85/rc-things-pi-rc-tank:latest

echo -e "\nbuilt artifacts:\n"

find "$(pwd)/target/pi-rc-tank"
file "$(pwd)/target/pi-rc-tank"
