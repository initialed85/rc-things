FROM ubuntu:20.04

RUN apt-get update && apt-get install -y curl build-essential \
    g++-arm-linux-gnueabihf libc6-dev-armhf-cross

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > install.sh && \
  chmod +x install.sh && \
  ./install.sh -y

ENV PATH=${PATH}:/root/.cargo/bin/

RUN rustup update

RUN rustup target add armv7-unknown-linux-gnueabihf
RUN rustup toolchain install stable-armv7-unknown-linux-gnueabihf

WORKDIR /srv/

COPY rc-messaging /rc-messaging
COPY rc-vehicle /rc-vehicle
COPY pi-rc-tank/Cargo.lock /srv/Cargo.lock
COPY pi-rc-tank/Cargo.toml /srv/Cargo.toml
COPY pi-rc-tank/src /srv/src

ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
ENV CC_armv7_unknown_Linux_gnueabihf=arm-linux-gnueabihf-gcc
ENV CXX_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++

RUN cargo update
RUN cargo build --profile release --target armv7-unknown-linux-gnueabihf

ENTRYPOINT ["bash", "-c", "mkdir -p /srv/artifacts && cp -f /srv/target/armv7-unknown-linux-gnueabihf/release/pi-rc-tank /srv/artifacts"]

CMD []
