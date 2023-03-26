FROM --platform=linux/amd64 ubuntu:22.04 AS base

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev \
    dbus-x11 libxv1 mesa-utils mesa-utils-extra psmisc procps libvulkan-dev

ENV LANG en_US.UTF-8
RUN echo $LANG UTF-8 > /etc/locale.gen && \
    apt-get install -y locales && \
    update-locale --reset LANG=$LANG

RUN apt-get install -y --no-install-recommends xauth xinit x11-xserver-utils && \
    apt-get install -y xwayland

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > install.sh && \
    chmod +x install.sh && \
    ./install.sh -y

ENV PATH=${PATH}:/root/.cargo/bin/

RUN rustup update

RUN apt-get install -y dbus-x11 libxv1 mesa-utils mesa-utils-extra psmisc procps

WORKDIR /srv/

COPY Cargo.lock /srv/Cargo.lock
COPY Cargo.toml /srv/Cargo.toml
COPY src /srv/src

RUN cargo update
RUN cargo build --bin car-server --features car-server --profile release
RUN cargo build --bin robot-server --features robot-server --profile release

RUN mkdir -p /srv/artifacts

ENTRYPOINT ["/bin/bash", "-c", "cp -frv target/release/car-server target/release/robot-server /srv/artifacts/ && ls -alR /srv/artifacts/"]
CMD []
