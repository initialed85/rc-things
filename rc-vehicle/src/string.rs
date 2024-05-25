pub trait StringFormatHandler {
    fn set_throttles(&mut self, throttle_left: f32, throttle_right: f32) -> anyhow::Result<()>;
}

pub struct StringFormatTank {
    swap_throttles: bool,
    string_format_handler: Box<dyn StringFormatHandler>,
}

impl StringFormatTank {
    pub fn new(swap_throttles: bool, string_format_handler: Box<dyn StringFormatHandler>) -> Self {
        Self {
            swap_throttles,
            string_format_handler,
        }
    }
}

impl crate::vehicle::InputMessageHandler for StringFormatTank {
    fn handle_input_message(
        &mut self,
        input_message: rc_messaging::serialization::InputMessage,
    ) -> anyhow::Result<()> {
        let throttle_left;
        let throttle_right;

        if !self.swap_throttles {
            throttle_left = input_message.throttle_left;
            throttle_right = input_message.throttle_right;
        } else {
            throttle_left = input_message.throttle_right;
            throttle_right = input_message.throttle_left;
        }

        println!(
            "throttle_left={:?}, throttle_right={:?} from input_message={:?}",
            throttle_left, throttle_right, input_message
        );

        self.string_format_handler
            .set_throttles(throttle_left, throttle_right)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rc_messaging::serialization::InputMessage;

    use crate::vehicle::InputMessageHandler;

    use super::*;

    struct TestStringCar {
        input_states: std::sync::Arc<std::sync::Mutex<Vec<(f32, f32)>>>,
    }

    impl StringFormatHandler for TestStringCar {
        fn set_throttles(&mut self, throttle_left: f32, throttle_right: f32) -> anyhow::Result<()> {
            let mut input_states = self.input_states.lock().unwrap();

            input_states.push((throttle_left, throttle_right));

            Ok(())
        }
    }

    fn get_test_resources() -> (impl Fn() -> Vec<(f32, f32)>, StringFormatTank) {
        let shareable_input_states = std::sync::Arc::new(std::sync::Mutex::new(vec![]));

        let closure_input_states = std::sync::Arc::clone(&shareable_input_states);

        let drain_input_states = move || {
            let mut input_states = closure_input_states.lock().unwrap();
            let cloned_input_states = input_states.clone();
            input_states.clear();
            cloned_input_states
        };

        let test_string_car = TestStringCar {
            input_states: Arc::clone(&shareable_input_states),
        };

        let string_format_tank = StringFormatTank::new(false, Box::new(test_string_car));

        (drain_input_states, string_format_tank)
    }

    fn get_input_message(throttle_left: f32, throttle_right: f32) -> InputMessage {
        rc_messaging::serialization::InputMessage {
            throttle: 0.0,
            steering: 0.0,
            throttle_left,
            throttle_right,
            steering_left: 0.0,
            steering_right: 0.0,
            mode_up: false,
            mode_down: false,
            mode_left: false,
            mode_right: false,
            handbrake: false,
        }
    }

    #[test]
    fn happy_path() -> anyhow::Result<()> {
        let (drain_input_states, mut string_format_tank) = get_test_resources();

        string_format_tank.handle_input_message(get_input_message(0.0, 0.0))?;
        let input_states = drain_input_states();
        let last_input_state = input_states.last().unwrap();
        assert_eq!(last_input_state, &(0.0, 0.0));

        string_format_tank.handle_input_message(get_input_message(1.0, 1.0))?;
        let input_states = drain_input_states();
        let last_input_state = input_states.last().unwrap();
        assert_eq!(last_input_state, &(1.0, 1.0));

        string_format_tank.handle_input_message(get_input_message(-1.0, -1.0))?;
        let input_states = drain_input_states();
        let last_input_state = input_states.last().unwrap();
        assert_eq!(last_input_state, &(-1.0, -1.0));

        Ok(())
    }
}
