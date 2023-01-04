use std::error::Error;
use std::io;
use std::net::SocketAddr;

use rppal::pwm::{Channel, Error as PwmError, Polarity, Pwm};
use tokio::net::UdpSocket;

use rc_things::{deserialize, get_socket_addr_from_env, InputMessage};

pub const PERIOD: f64 = 17.34 * 0.001;
pub const FREQUENCY: f64 = 1.0 / PERIOD;

pub const DUTY_CYCLE_PERIOD_MIN: f64 = 1100.0 * 0.000001; // forward / left
pub const DUTY_CYCLE_PERIOD_IDLE: f64 = 1500.0 * 0.000001; // idle / center
pub const DUTY_CYCLE_PERIOD_MAX: f64 = 1900.0 * 0.000001; // reverse / right

pub const DUTY_CYCLE_MIN: f64 = DUTY_CYCLE_PERIOD_MIN / PERIOD;
pub const DUTY_CYCLE_IDLE: f64 = DUTY_CYCLE_PERIOD_IDLE / PERIOD;
pub const DUTY_CYCLE_MAX: f64 = DUTY_CYCLE_PERIOD_MAX / PERIOD;

pub const STEERING_DUTY_CYCLE_IDLE: f64 = DUTY_CYCLE_IDLE;
pub const STEERING_DUTY_CYCLE_LEFT: f64 = DUTY_CYCLE_MIN;
pub const STEERING_DUTY_CYCLE_RIGHT: f64 = DUTY_CYCLE_MAX;
pub const STEERING_DUTY_CYCLE_RANGE: f64 = STEERING_DUTY_CYCLE_RIGHT - STEERING_DUTY_CYCLE_LEFT;

pub const THROTTLE_AND_BRAKE_DUTY_CYCLE_IDLE: f64 = DUTY_CYCLE_IDLE;
pub const THROTTLE_AND_BRAKE_DUTY_CYCLE_FORWARD: f64 = DUTY_CYCLE_MIN;
pub const THROTTLE_AND_BRAKE_DUTY_CYCLE_REVERSE: f64 = DUTY_CYCLE_MAX;
pub const THROTTLE_AND_BRAKE_DUTY_CYCLE_RANGE: f64 =
    THROTTLE_AND_BRAKE_DUTY_CYCLE_REVERSE - THROTTLE_AND_BRAKE_DUTY_CYCLE_FORWARD;

async fn run(socket: UdpSocket) {
    let throttle_and_brake_pwm = Pwm::with_frequency(
        Channel::Pwm0,
        FREQUENCY,
        THROTTLE_AND_BRAKE_DUTY_CYCLE_IDLE,
        Polarity::Normal,
        true,
    )
    .unwrap();

    let steering_pwm = Pwm::with_frequency(
        Channel::Pwm1,
        FREQUENCY,
        STEERING_DUTY_CYCLE_IDLE,
        Polarity::Normal,
        true,
    )
    .unwrap();

    let mut buf: Vec<u8> = vec![0; 65536];

    loop {
        let (n, _) = socket.recv_from(&mut buf).await.unwrap();

        if n == 0 {
            continue;
        }

        let input_message_data = buf[0..n].to_vec();
        // println!("input_message_data={:?}", input_message_data);

        let input_message = deserialize::<InputMessage>(input_message_data);
        println!("input_message={:?}", input_message);

        let steering_duty_cycle = ((input_message.steering as f64 + 1.0) / 2.0)
            * STEERING_DUTY_CYCLE_RANGE
            + STEERING_DUTY_CYCLE_RIGHT
            - STEERING_DUTY_CYCLE_RANGE;

        steering_pwm.set_duty_cycle(steering_duty_cycle).unwrap();

        println!("steering_duty_cycle={:?}", steering_duty_cycle);

        let throttle_and_brake_duty_cycle =
            (((input_message.throttle - input_message.brake) as f64 + 1.0) / 2.0)
                * THROTTLE_AND_BRAKE_DUTY_CYCLE_RANGE
                + THROTTLE_AND_BRAKE_DUTY_CYCLE_REVERSE
                - THROTTLE_AND_BRAKE_DUTY_CYCLE_RANGE;

        throttle_and_brake_pwm
            .set_duty_cycle(throttle_and_brake_duty_cycle)
            .unwrap();

        println!(
            "throttle_and_brake_duty_cycle={:?}",
            throttle_and_brake_duty_cycle
        );
    }
}

#[tokio::main]
async fn main() {
    let local_addr = get_socket_addr_from_env();
    println!("local_addr={:?}", local_addr);

    let socket = UdpSocket::bind(&local_addr).await.unwrap();

    run(socket).await;
}
