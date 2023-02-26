# rc-things

## Overview

- Car; control an RC car that's more or less a Tamiya Lunch Box using a PS4 controller
    - Server = Workstation w/ UI and PS4 controller
    - Client = Raspberry Pi 3 attached to an RC car that feels like a Tamiya Lunch Box
- Robot; control a hobby robot using a PS4 controller
    - Server = Workstation w/ UI and PS4 controller
    - Client = Raspberry Pi 3 with a Husarion Core2 driving a hobby robot
- Tello; control a DJI Tello drone using a PS4 controller
    - Client = Workstation w/ UI and PS4 controller

## Usage

### Car

#### Server

##### Prerequisites

- Server
    - Raspberry Pi 3 w/ Ubuntu Server 22.04 (at least that's what I'm using)
    - Rust
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

NOTE: The IP of your server is more than likely not `192.168.137.25`

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

NOTE: The IP of your server is more than likely not `192.168.137.26`

#### Android

```shell
# one time
cargo install --git=https://github.com/dodorare/crossbow crossbundle
brew install java
echo 'export PATH="/opt/homebrew/opt/openjdk/bin:$PATH"' >> ~/.bash_profile
sudo ln -sfn /opt/homebrew/opt/openjdk/libexec/openjdk.jdk /Library/Java/JavaVirtualMachines/openjdk.jdk
crossbundle install --preferred
crossbundle install command-line-tools
crossbundle install sdkmanager --install "build-tools;31.0.0" "ndk;23.1.7779620" "platforms;android-31"
crossbundle install bundletool

# per build
```
