// TODO: disabled to save cycles
// const MESSAGE_EARLY_HZ: f64 = 30.0;
// const MESSAGE_EARLY: std::time::Duration = std::time::Duration::from_millis((1.0 / MESSAGE_EARLY_HZ * 1000.0) as u64);

const MESSAGE_TIMEOUT_HZ: f64 = 5.0;
const MESSAGE_TIMEOUT: std::time::Duration =
    std::time::Duration::from_millis((1.0 / MESSAGE_TIMEOUT_HZ * 1000.0) as u64);

pub trait InputMessageHandler {
    fn handle_input_message(
        &mut self,
        input_message: rc_messaging::serialization::InputMessage,
    ) -> anyhow::Result<()>;
}

fn get_safe_input_message() -> rc_messaging::serialization::InputMessage {
    rc_messaging::serialization::InputMessage {
        throttle: 0.0,
        steering: 0.0,
        throttle_left: 0.0,
        throttle_right: 0.0,
        steering_left: 0.0,
        steering_right: 0.0,
        mode_up: false,
        mode_down: false,
        mode_left: false,
        mode_right: false,
        handbrake: true,
    }
}

pub struct Vehicle {
    incoming_input_message_receiver:
        std::sync::mpsc::Receiver<rc_messaging::serialization::InputMessage>,
    input_message_handler: Box<dyn InputMessageHandler>,
    closed: std::sync::Arc<std::sync::Mutex<bool>>,
    last_input_message: Option<rc_messaging::serialization::InputMessage>,
    throttle_min: f32,
    throttle_max: f32,
    steering_offset: f32,
}

impl Vehicle {
    pub fn new(
        incoming_input_message_receiver: std::sync::mpsc::Receiver<
            rc_messaging::serialization::InputMessage,
        >,
        input_message_handler: Box<dyn InputMessageHandler>,
        starting_throttle_min: f32,
        starting_throttle_max: f32,
        starting_steering_offset: f32,
    ) -> Self {
        Self {
            incoming_input_message_receiver,
            input_message_handler,
            closed: std::sync::Arc::new(std::sync::Mutex::new(false)),
            last_input_message: None,
            throttle_min: starting_throttle_min,
            throttle_max: starting_throttle_max,
            steering_offset: starting_steering_offset,
        }
    }

    pub fn get_closer(&self) -> impl Fn() {
        let closed = std::sync::Arc::clone(&self.closed);
        return move || {
            let mut closed = closed.lock().unwrap();
            *closed = true;
        };
    }

    fn handle_input_message(
        &mut self,
        input_message: rc_messaging::serialization::InputMessage,
    ) -> anyhow::Result<()> {
        self.input_message_handler
            .handle_input_message(input_message.clone())?;
        self.last_input_message = Some(input_message);

        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let safe_input_message = get_safe_input_message();

        // TODO: disabled to save cycles
        // let mut last_message_time = std::time::Instant::now();

        loop {
            let closed = { *self.closed.lock().unwrap() };

            if closed {
                println!(
                    "closed={:?}; sending safe input_message={:?}",
                    closed, safe_input_message
                );
                _ = self.handle_input_message(safe_input_message.clone());
                break;
            }

            let recv_timeout_result = self
                .incoming_input_message_receiver
                .recv_timeout(MESSAGE_TIMEOUT);
            if recv_timeout_result.is_err() {
                let err = recv_timeout_result.err().unwrap();
                _ = self.handle_input_message(safe_input_message.clone());
                match err {
                    std::sync::mpsc::RecvTimeoutError::Timeout => {
                        println!(
                            "err={:?}, MESSAGE_TIMEOUT={:?}, sending safe input_message={:?}",
                            err, MESSAGE_TIMEOUT, safe_input_message
                        );
                        continue;
                    }
                    std::sync::mpsc::RecvTimeoutError::Disconnected => {
                        println!(
                            "err={:?}, sending safe input_message={:?}",
                            err, safe_input_message
                        );
                        break;
                    }
                }
            }

            let mut input_message = recv_timeout_result?;

            if input_message.mode_up
                && (self.last_input_message.is_none()
                    || !self.last_input_message.as_ref().unwrap().mode_up)
            {
                self.throttle_max += 0.10;
                self.throttle_max = self.throttle_max.min(1.0);
                self.throttle_max = self.throttle_max.max(0.0);

                self.throttle_min -= 0.10;
                self.throttle_min = self.throttle_min.max(-1.0);
                self.throttle_min = self.throttle_min.min(0.0);
            }

            if input_message.mode_down
                && (self.last_input_message.is_none()
                    || !self.last_input_message.as_ref().unwrap().mode_down)
            {
                self.throttle_max -= 0.10;
                self.throttle_max = self.throttle_max.min(1.0);
                self.throttle_max = self.throttle_max.max(0.0);

                self.throttle_min += 0.10;
                self.throttle_min = self.throttle_min.max(-1.0);
                self.throttle_min = self.throttle_min.min(0.0);
            }

            if input_message.mode_left {
                self.steering_offset -= 0.01;
                self.steering_offset = self.steering_offset.min(1.0);
                self.steering_offset = self.steering_offset.max(-1.0);
            }

            if input_message.mode_right {
                self.steering_offset += 0.01;
                self.steering_offset = self.steering_offset.min(1.0);
                self.steering_offset = self.steering_offset.max(-1.0);
            }

            // TODO: this is the old throttle limit code
            // if input_message.throttle > 0.0 {
            //     input_message.throttle = input_message.throttle.min(self.throttle_max);
            // } else if input_message.throttle < 0.0 {
            //     input_message.throttle = input_message.throttle.max(self.throttle_min);
            // }
            //
            // if input_message.throttle_left > 0.0 {
            //     input_message.throttle_left = input_message.throttle_left.min(self.throttle_max);
            // } else if input_message.throttle_left < 0.0 {
            //     input_message.throttle_left = input_message.throttle_left.max(self.throttle_min);
            // }
            //
            // if input_message.throttle_right > 0.0 {
            //     input_message.throttle_right = input_message.throttle_right.min(self.throttle_max);
            // } else if input_message.throttle_right < 0.0 {
            //     input_message.throttle_right = input_message.throttle_right.max(self.throttle_min);
            // }

            // the new throttle scale code
            let throttle_adjusted = input_message.throttle.abs() * self.throttle_max / 1.0;
            if input_message.throttle > 0.0 {
                input_message.throttle = throttle_adjusted
            } else if input_message.throttle < 0.0 {
                input_message.throttle = -throttle_adjusted;
            }

            let throttle_left_adjusted =
                input_message.throttle_left.abs() * self.throttle_max / 1.0;
            if input_message.throttle_left > 0.0 {
                input_message.throttle_left = throttle_left_adjusted
            } else if input_message.throttle_left < 0.0 {
                input_message.throttle_left = -throttle_left_adjusted;
            }

            let throttle_right_adjusted =
                input_message.throttle_right.abs() * self.throttle_max / 1.0;
            if input_message.throttle_right > 0.0 {
                input_message.throttle_right = throttle_right_adjusted
            } else if input_message.throttle_right < 0.0 {
                input_message.throttle_right = -throttle_right_adjusted;
            }

            if self.steering_offset > 0.0 {
                if input_message.steering >= 0.0 && input_message.steering < self.steering_offset {
                    input_message.steering = self.steering_offset
                }
            } else if self.steering_offset < 0.0
                && input_message.steering <= 0.0
                && input_message.steering > self.steering_offset
            {
                input_message.steering = self.steering_offset
            }

            // TODO: disabled to save cycles
            // let now = std::time::Instant::now();
            // let message_interval = now - last_message_time;
            // if message_interval < MESSAGE_EARLY {
            //     println!("message_interval={:?} < MESSAGE_EARLY={:?}, ignoring input_message={:?}", message_interval, MESSAGE_EARLY, input_message);
            //     continue;
            // }
            // last_message_time = now;

            self.handle_input_message(input_message)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MESSAGE_INTERVAL_HZ: f64 = 20.0;
    const MESSAGE_INTERVAL: std::time::Duration =
        std::time::Duration::from_millis((1.0 / MESSAGE_INTERVAL_HZ * 1000.0) as u64);

    struct TestVehicle {
        input_messages:
            std::sync::Arc<std::sync::Mutex<Vec<rc_messaging::serialization::InputMessage>>>,
    }

    impl InputMessageHandler for TestVehicle {
        fn handle_input_message(
            &mut self,
            input_message: rc_messaging::serialization::InputMessage,
        ) -> anyhow::Result<()> {
            {
                let mut input_messages = self.input_messages.lock().unwrap();
                input_messages.push(input_message.clone());
            }

            Ok(())
        }
    }

    fn get_test_resources() -> (
        std::sync::mpsc::Sender<rc_messaging::serialization::InputMessage>,
        impl Fn() -> Vec<rc_messaging::serialization::InputMessage>,
        impl Fn(),
        std::thread::JoinHandle<()>,
        rc_messaging::serialization::InputMessage,
    ) {
        let (sender, receiver) = std::sync::mpsc::channel();

        let shareable_input_messages = std::sync::Arc::new(std::sync::Mutex::new(vec![]));

        let closure_input_messages = std::sync::Arc::clone(&shareable_input_messages);

        let drain_input_messages = move || {
            let mut input_messages = closure_input_messages.lock().unwrap();
            let cloned_input_messages = input_messages.clone();
            for x in cloned_input_messages.clone() {
                println!("{:?}", x);
            }
            input_messages.clear();
            cloned_input_messages
        };

        let test_vehicle = TestVehicle {
            input_messages: std::sync::Arc::clone(&shareable_input_messages),
        };

        let (vehicle_closer_sender, vehicle_closer_receiver) = std::sync::mpsc::channel();

        let input_message = rc_messaging::serialization::InputMessage {
            throttle: 1.0,
            steering: 0.0,
            throttle_left: 0.0,
            throttle_right: 0.0,
            steering_left: 0.0,
            steering_right: 0.0,
            mode_up: false,
            mode_down: false,
            mode_left: false,
            mode_right: false,
            handbrake: false,
        };

        let vehicle_handle = std::thread::spawn(move || {
            let mut vehicle = Vehicle::new(receiver, Box::new(test_vehicle), -1.0, 1.0, 0.0);
            let vehicle_closer = vehicle.get_closer();
            vehicle_closer_sender.send(vehicle_closer).unwrap();
            vehicle.run().unwrap();
        });

        let vehicle_closer = vehicle_closer_receiver.recv().unwrap();

        (
            sender,
            drain_input_messages,
            vehicle_closer,
            vehicle_handle,
            input_message,
        )
    }

    #[test]
    fn happy_path() -> anyhow::Result<()> {
        let (sender, drain_input_messages, vehicle_closer, vehicle_handle, mut input_message) =
            get_test_resources();

        std::thread::sleep(MESSAGE_INTERVAL);
        sender.send(input_message.clone()).unwrap();
        std::thread::sleep(MESSAGE_INTERVAL);
        let input_messages = drain_input_messages();
        assert!(input_messages.contains(&input_message));

        let mut expected_input_message = input_message.clone();
        expected_input_message.throttle = 0.8;
        input_message.mode_down = true;
        std::thread::sleep(MESSAGE_INTERVAL);
        sender.send(input_message.clone()).unwrap();
        std::thread::sleep(MESSAGE_INTERVAL);
        input_message.mode_down = false;
        std::thread::sleep(MESSAGE_INTERVAL);
        sender.send(input_message.clone()).unwrap();
        std::thread::sleep(MESSAGE_INTERVAL);
        let input_messages = drain_input_messages();
        assert!(input_messages.contains(&expected_input_message));

        let mut expected_input_message = input_message.clone();
        expected_input_message.throttle = 0.8;
        expected_input_message.steering = -0.01;
        input_message.mode_left = true;
        std::thread::sleep(MESSAGE_INTERVAL);
        sender.send(input_message.clone()).unwrap();
        std::thread::sleep(MESSAGE_INTERVAL);
        input_message.mode_left = false;
        std::thread::sleep(MESSAGE_INTERVAL);
        sender.send(input_message.clone()).unwrap();
        std::thread::sleep(MESSAGE_INTERVAL);
        let input_messages = drain_input_messages();
        assert!(input_messages.contains(&expected_input_message));

        let mut expected_input_message = input_message.clone();
        expected_input_message.throttle = 0.8;
        expected_input_message.steering = -0.09999999;

        for _ in 0..9 {
            input_message.mode_left = true;
            std::thread::sleep(MESSAGE_INTERVAL);
            sender.send(input_message.clone()).unwrap();
            std::thread::sleep(MESSAGE_INTERVAL);
            input_message.mode_left = false;
            std::thread::sleep(MESSAGE_INTERVAL);
            sender.send(input_message.clone()).unwrap();
            std::thread::sleep(MESSAGE_INTERVAL);
        }

        let input_messages = drain_input_messages();
        assert!(input_messages.contains(&expected_input_message));

        let mut expected_input_message = input_message.clone();
        expected_input_message.throttle = 0.8;
        expected_input_message.steering = 0.0;

        for _ in 0..10 {
            input_message.mode_right = true;
            std::thread::sleep(MESSAGE_INTERVAL);
            sender.send(input_message.clone()).unwrap();
            std::thread::sleep(MESSAGE_INTERVAL);
            input_message.mode_right = false;
            std::thread::sleep(MESSAGE_INTERVAL);
            sender.send(input_message.clone()).unwrap();
            std::thread::sleep(MESSAGE_INTERVAL);
        }

        let input_messages = drain_input_messages();
        assert!(input_messages.contains(&expected_input_message));

        vehicle_closer();
        vehicle_handle.join().unwrap();
        Ok(())
    }

    // TODO: disabled to save cycles
    // #[test]
    // fn too_early() -> anyhow::Result<()> {
    //     let (sender, drain_input_messages, vehicle_closer, vehicle_handle, input_message) = get_test_resources();
    //
    //     // this message will be ignored because it's too early
    //     sender.send(input_message.clone()).unwrap();
    //
    //     std::thread::sleep(MESSAGE_INTERVAL);
    //     let input_messages = drain_input_messages();
    //
    //     assert_eq!(input_messages.contains(&input_message), false);
    //
    //     vehicle_closer();
    //     vehicle_handle.join().unwrap();
    //     return Ok(());
    // }

    #[test]
    fn too_late() -> anyhow::Result<()> {
        let (_, drain_input_messages, vehicle_closer, vehicle_handle, input_message) =
            get_test_resources();

        // this will cause a timeout
        std::thread::sleep(MESSAGE_TIMEOUT);

        std::thread::sleep(MESSAGE_INTERVAL);
        let input_messages = drain_input_messages();

        assert!(!input_messages.contains(&input_message));

        vehicle_closer();
        vehicle_handle.join().unwrap();
        Ok(())
    }
}
