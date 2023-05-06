FROM ubuntu:22.04

RUN apt-get update && apt-get install -y curl build-essential

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > install.sh && \
  chmod +x install.sh && \
  ./install.sh -y

ENV PATH=${PATH}:/root/.cargo/bin/

RUN rustup update

WORKDIR /srv/

COPY rc-messaging /rc-messaging
COPY rc-vehicle /rc-vehicle
COPY pi-rc-tank/Cargo.lock /srv/Cargo.lock
COPY pi-rc-tank/Cargo.toml /srv/Cargo.toml
COPY pi-rc-tank/src /srv/src

RUN cargo update
RUN cargo build --profile release

ENTRYPOINT ["bash", "-c", "mkdir -p /srv/artifacts && cp -f /srv/target/release/pi-rc-tank /srv/artifacts"]

CMD []
