use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use rppal::pwm::{Channel, Error as PwmError, Polarity, Pwm};
use rppal::uart::{Parity, Uart};
use tokio::net::UdpSocket;
use tokio::spawn;
use tokio::time::{sleep, Duration};

use rc_things::{deserialize, get_socket_addr_from_env, InputMessage};

async fn watchdog(last_update: Arc<Mutex<SystemTime>>, uart: Arc<Mutex<Uart>>) {
    loop {
        {
            let mut last_update = last_update.lock().unwrap();

            let now = SystemTime::now();

            if now.duration_since(*last_update).unwrap() > Duration::from_secs_f32(0.2) {
                let output_message = format!("{:.20},{:.20}\r\n", 0.0, 0.0);

                {
                    uart.lock()
                        .unwrap()
                        .write(output_message.as_bytes())
                        .unwrap();
                }

                *last_update = SystemTime::now();

                println!("last_update={:?}, watchdog fired!", last_update);
            }
        }

        sleep(Duration::from_secs_f32(0.1)).await;
    }
}

async fn run(socket: UdpSocket, last_update: Arc<Mutex<SystemTime>>, uart: Arc<Mutex<Uart>>) {
    let mut drive_scale = 0.5;

    let mut buf: Vec<u8> = vec![0; 65536];

    loop {
        let (n, _) = socket.recv_from(&mut buf).await.unwrap();

        let input_message_data = buf[0..n].to_vec();

        let input_message = deserialize::<InputMessage>(input_message_data);

        if input_message.up {
            drive_scale = 1.0;
        } else if input_message.down {
            drive_scale = 0.5;
        }

        let output_message = format!(
            "{:.20},{:.20}\r\n",
            input_message.left_drive * drive_scale,
            input_message.right_drive * drive_scale
        );

        {
            uart.lock()
                .unwrap()
                .write(output_message.as_bytes())
                .unwrap();

            let mut last_update = last_update.lock().unwrap();
            *last_update = SystemTime::now();
        }

        let last_update = last_update.lock().unwrap();

        println!(
            "last_update={:?}, input_message={:?}, output_message={:?}",
            last_update, input_message, output_message,
        );
    }
}

#[tokio::main]
async fn main() {
    let uart = Arc::new(Mutex::new(
        Uart::with_path("/dev/ttyAMA0", 115200, Parity::None, 8, 1).unwrap(),
    ));

    let last_update = Arc::new(Mutex::new(SystemTime::now()));

    let local_addr = get_socket_addr_from_env();

    let socket = UdpSocket::bind(&local_addr).await.unwrap();

    spawn(watchdog(Arc::clone(&last_update), Arc::clone(&uart)));

    run(socket, Arc::clone(&last_update), Arc::clone(&uart)).await;
}
