use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use rppal::pwm::{Channel, Error as PwmError, Polarity, Pwm};
use tokio::net::UdpSocket;
use tokio::spawn;
use tokio::time::{sleep, Duration};

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

async fn watchdog(
    last_update: Arc<Mutex<SystemTime>>,
    throttle_and_brake_pwm: Arc<Mutex<Pwm>>,
    steering_pwm: Arc<Mutex<Pwm>>,
) {
    loop {
        {
            let mut last_update = last_update.lock().unwrap();

            let now = SystemTime::now();

            if now.duration_since(*last_update).unwrap() > Duration::from_secs_f32(0.2) {
                let mut steering_pwm = steering_pwm.lock().unwrap();
                steering_pwm
                    .set_duty_cycle(STEERING_DUTY_CYCLE_IDLE)
                    .unwrap();

                let mut throttle_and_brake_pwm = throttle_and_brake_pwm.lock().unwrap();
                throttle_and_brake_pwm
                    .set_duty_cycle(THROTTLE_AND_BRAKE_DUTY_CYCLE_IDLE)
                    .unwrap();

                *last_update = SystemTime::now();

                println!("last_update={:?}, watchdog fired!", last_update);
            }
        }

        sleep(Duration::from_secs_f32(0.1)).await;
    }
}

async fn run(
    socket: UdpSocket,
    last_update: Arc<Mutex<SystemTime>>,
    throttle_and_brake_pwm: Arc<Mutex<Pwm>>,
    steering_pwm: Arc<Mutex<Pwm>>,
) {
    let mut throttle_and_brake_scale = 0.33;

    let mut buf: Vec<u8> = vec![0; 65536];

    loop {
        let (n, _) = socket.recv_from(&mut buf).await.unwrap();

        let input_message_data = buf[0..n].to_vec();

        let input_message = deserialize::<InputMessage>(input_message_data);

        let steering_duty_cycle = ((input_message.steering as f64 + 1.0) / 2.0)
            * STEERING_DUTY_CYCLE_RANGE
            + STEERING_DUTY_CYCLE_RIGHT
            - STEERING_DUTY_CYCLE_RANGE;

        if input_message.up {
            throttle_and_brake_scale = 1.0;
        } else if input_message.down {
            throttle_and_brake_scale = 0.33;
        }

        let mut scaled_throttle = input_message.throttle * throttle_and_brake_scale;
        let mut scaled_brake = input_message.brake * throttle_and_brake_scale;

        let throttle_and_brake_duty_cycle = (((scaled_throttle - scaled_brake) as f64 + 1.0) / 2.0)
            * THROTTLE_AND_BRAKE_DUTY_CYCLE_RANGE
            + THROTTLE_AND_BRAKE_DUTY_CYCLE_REVERSE
            - THROTTLE_AND_BRAKE_DUTY_CYCLE_RANGE;

        {
            let mut steering_pwm = steering_pwm.lock().unwrap();
            steering_pwm.set_duty_cycle(steering_duty_cycle).unwrap();

            let mut throttle_and_brake_pwm = throttle_and_brake_pwm.lock().unwrap();
            throttle_and_brake_pwm
                .set_duty_cycle(throttle_and_brake_duty_cycle)
                .unwrap();

            let mut last_update = last_update.lock().unwrap();
            *last_update = SystemTime::now();
        }

        let last_update = last_update.lock().unwrap();

        println!(
             "last_update={:?}, input_message={:?}, steering_duty_cycle={:?}, throttle_and_brake_duty_cycle={:?}",
             last_update, input_message, steering_duty_cycle, throttle_and_brake_duty_cycle
         );
    }
}

#[tokio::main]
async fn main() {
    let throttle_and_brake_pwm = Arc::new(Mutex::new(
        Pwm::with_frequency(
            Channel::Pwm0,
            FREQUENCY,
            THROTTLE_AND_BRAKE_DUTY_CYCLE_IDLE,
            Polarity::Normal,
            true,
        )
        .unwrap(),
    ));

    let steering_pwm = Arc::new(Mutex::new(
        Pwm::with_frequency(
            Channel::Pwm1,
            FREQUENCY,
            STEERING_DUTY_CYCLE_IDLE,
            Polarity::Normal,
            true,
        )
        .unwrap(),
    ));

    let last_update = Arc::new(Mutex::new(SystemTime::now()));

    let local_addr = get_socket_addr_from_env();

    let socket = UdpSocket::bind(&local_addr).await.unwrap();

    spawn(watchdog(
        Arc::clone(&last_update),
        Arc::clone(&throttle_and_brake_pwm),
        Arc::clone(&steering_pwm),
    ));

    run(
        socket,
        Arc::clone(&last_update),
        Arc::clone(&throttle_and_brake_pwm),
        Arc::clone(&steering_pwm),
    )
    .await;
}
