# rc-things

## Overview

- Car; control an RC car that's more or less a Tamiya Lunch Box using a PS4 controller
    - Server = Workstation w/ UI and PS4 controller
    - Client = Raspberry Pi 3 attached to an RC car that feels like a Tamiya Lunch Box
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

You'll probably also need the following items for the Pi at `/boot/firmware/config.txt`:

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
cargo build --bin car-client --features car-client && HOST=192.168.137.25 PORT=13337 ./target/debug/car-client
```

NOTE: The IP of your server is more than likely not `192.168.137.25`

## Notes

### Hosim 9155 w/ Tamiya Lunch Box pieces

#### Context

- Hosim has an integrated radio + speed controller
    - No practical way to interact with the speed controller without the radio (circuitry is fully potted)
- Hosim has a 5-wire servo
    - Relies on some circuitry in the integrated radio + speed controller to drive it

#### Approach

- Use the speed controller and battery from old Tamiya Lunch Box
- Use a 3-wire aircraft servo a friend gifted to me years ago

#### Discovery

##### Coarse test by just wiring it up

###### Findings

- Tamiya radio + Tamiya speed controller duty cycle range not correct for Hosim motor
    - Only operates at idle, maximum forward and maximum reverse
- 3-wire aircraft servo chatters a great deal

##### Hosim speed controller -> motor interface

###### Findings

- Idle
    - Vpp: 14.8
    - Vmax: 7.6
    - Vmin: -7.2
    - Freq: 57.6 Hz
    - Dut : 50%
- Max forward
    - Vpp: 0.12
    - Vmax: 7.52
    - Vmin: 7.44
    - Freq: 1010 Hz
    - Dut : 0%
- Min forward
    - Vpp: 0.12
    - Vmax: -7.16
    - Vmin: -7.22
    - Freq: 1010 Hz
    - Dut : 87%
- Max reverse
    - Vpp: 14.8
    - Vmax: 7.6
    - Vmin: -7.2
    - Freq: 57.6 Hz
    - Dut : 87%
- Min reverse
    - Vpp: 14.8
    - Vmax: 7.6
    - Vmin: -7.2
    - Freq: 57.6 Hz
    - Dut : 13%

###### Observations

- Idle
    - 0V DC bias
    - 50 Hz PWM
        - 50% = constant
- Forward
    - 7.4V DC bias
    - 1 kHz PWM
        - 0% = max
        - 100% = min
- Reverse
    - -7.4V DC bias
    - 1 kHz PWM
        - 100% = max
        - 0% = min

##### Tamiya radio -> Tamiya speed controller interface

###### Findings

- Idle
    - Vpp: 6.32
    - Vmax: 5.60
    - Vmin: -0.72
    - Freq: 57.6 Hz
    - Dut : 8.8%
- Max forward
    - Vpp: 6.32
    - Vmax: 5.60
    - Vmin: -0.72
    - Freq: 57.6 Hz
    - Dut : 6.36%
- Min forward
    - Vpp: 6.32
    - Vmax: 5.60
    - Vmin: -0.72
    - Freq: 1010 Hz
    - Dut : 8.41%
- Max reverse
    - Vpp: 6.32
    - Vmax: 5.60
    - Vmin: -0.72
    - Freq: 57.6 Hz
    - Dut : 11.3%
- Min reverse
    - Vpp: 6.32
    - Vmax: 5.60
    - Vmin: -0.72
    - Freq: 57.6 Hz
    - Dut : 8.97%

###### Observations

- Idle
    - 0V DC bias
    - 50 Hz PWM
        - 9% = constant
- Forward
    - 0V DC bias
    - 1 kHz PWM
        - 6.36% = max
        - 8.41% = min
- Reverse
    - 0V DC bias
    - 1 kHz PWM
        - 11.3% = max
        - 8.97% = min

##### Tamiya radio -> 3-wire aircraft servo interface

###### Findings

- Center
    - Vpp: 6.32
    - Vmax: 5.60
    - Vmin: -0.72
    - Freq: 57.6 Hz
    - Dut : 8.8%
- Max left
    - Vpp: 6.32
    - Vmax: 5.60
    - Vmin: -0.72
    - Freq: 57.6 Hz
    - Dut : 6.36%
- Min left
    - Vpp: 6.32
    - Vmax: 5.60
    - Vmin: -0.72
    - Freq: 1010 Hz
    - Dut : 8.41%
- Max right
    - Vpp: 6.32
    - Vmax: 5.60
    - Vmin: -0.72
    - Freq: 57.6 Hz
    - Dut : 11.3%
- Min right
    - Vpp: 6.32
    - Vmax: 5.60
    - Vmin: -0.72
    - Freq: 57.6 Hz
    - Dut : 8.97%

###### Observations

- Idle
    - 0V DC bias
    - 50 Hz PWM
        - 9% = constant
- Left
    - 0V DC bias
    - 1 kHz PWM
        - 6.36% = max
        - 8.41% = min
- Right
    - 0V DC bias
    - 1 kHz PWM
        - 11.3% = max
        - 8.97% = min

##### Challenges and concerns

- I wasn't able to introspect the interface between the Tamiya speed controller and the motor; it seems to
  have some protection against the load not drawing enough current
- At least on the Tamiya radio -> Tamiya speed controller it looks like the duty cycle approach is inverted   
