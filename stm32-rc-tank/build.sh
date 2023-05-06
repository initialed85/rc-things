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
  --platform=linux/amd64 \
  -t initialed85/rc-things-stm32-rc-tank:latest \
  -f docker/stm32-rc-tank/Dockerfile \
  .

if test -e target; then
  rm -f target/*
fi

cd stm32-rc-tank

echo -e "\nextracting built artifacts...\n"
docker run \
  --platform=linux/amd64 \
  --rm -it \
  -v "$(pwd)/target:/srv/artifacts" \
  initialed85/rc-things-stm32-rc-tank:latest

echo -e "\nbuilt artifacts:\n"

find "$(pwd)/target"
