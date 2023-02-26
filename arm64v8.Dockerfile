FROM --platform=linux/arm64 arm64v8/rust:1.67.0-slim

WORKDIR /srv/

COPY Cargo.lock /srv/Cargo.lock
COPY Cargo.toml /srv/Cargo.toml
COPY src /srv/src

# RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/srv/target cargo update
# RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/srv/target cargo build --bin car-server --features car-server --profile release
# RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/srv/target cargo build --bin robot-server --features robot-server --profile release

RUN cargo update
RUN cargo build --bin car-server --features car-server --profile release
RUN cargo build --bin robot-server --features robot-server --profile release

RUN mkdir -p /srv/artifacts

ENTRYPOINT ["/bin/bash", "-c", "cp -frv target/release/car-server target/release/robot-server /srv/artifacts/ && ls -alR /srv/artifacts/"]
CMD []
