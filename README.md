# rc-things

## Overview

- Car; control an RC car that's more or less a Tamiya Lunch Box using a PS4 controller
    - Server = Raspberry Pi 3 attached to an RC car that feels like a Tamiya Lunch Box
    - Client = Workstation w/ UI and PS4 controller
- Robot; control a hobby robot using a PS4 controller
    - Server = Raspberry Pi 3 with a Husarion Core2 driving a hobby robot
    - Client = Workstation w/ UI and PS4 controller
- Tello; control a DJI Tello drone using a PS4 controller
    - Server = A DJI Tello drone
    - Client = Workstation w/ UI and PS4 controller

## Usage

### Car

#### Server

##### Prerequisites

- Server
    - Raspberry Pi 3 w/ Ubuntu Server 22.04 (at least that's what I'm using)
- Client
    - A workstation with a UI (I'm using a MacBook Pro laptop)
    - Rust

You'll probably also need the following items for the Pi at `/boot/firmware/config.txt` so the PWM pins work
properly:

```
dtparam=audio=off
dtparam=i2c_arm=off
dtparam=spi=off
dtoverlay=pwm-2chan
```

#### Server

```shell
cargo build --bin car-server --features car-server && sudo bash -c "HOST=0.0.0.0 PORT=13337 ./target/debug/car-server"
```

#### Client

```shell
cargo build --bin car-client --features car-client && HOST=192.168.137.22 PORT=13337 ./target/debug/car-client
```

### Robot

#### Server

##### Prerequisites

- Server
    - Raspberry Pi 3 w/ Ubuntu Server 22.04 (at least that's what I'm using)
    - Rust
- Client
    - A workstation with a UI (I'm using a MacBook Pro laptop)
    - Rust

You'll probably also need the following items for the Pi at `/boot/firmware/config.txt` so the UART can be
used to talk to the Core2:

```
dtparam=audio=off
dtparam=i2c_arm=off
dtparam=spi=off
dtoverlay=disable-bt
```

#### Server

```shell
cargo build --bin robot-server --features robot-server && sudo bash -c "HOST=0.0.0.0 PORT=13337 ./target/debug/robot-server"
```

#### Client

```shell
cargo build --bin robot-client --features robot-client && HOST=192.168.137.26 PORT=13337 ./target/debug/robot-client
```

### Tello

#### Client

```shell
cargo build --bin tello-client --features tello-client && ./target/debug/robot-client

```
