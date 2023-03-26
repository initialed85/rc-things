#!/usr/bin/env bash

set -e

if [[ -d artifacts ]]; then
  rm -fr artifacts
fi

mkdir -p artifacts

cargo build --bin car-client --features car-client --profile release && cargo build --bin robot-client --features robot-client --profile release && cargo build --bin tello-client --features tello-client --profile release &

docker build -t rc-things -f arm64v8.Dockerfile . && docker run --rm -v "$(pwd)/artifacts:/srv/artifacts" rc-things &

wait

mv target/release/car-client artifacts/car-client
mv target/release/robot-client artifacts/robot-client

ls -alR artifacts
