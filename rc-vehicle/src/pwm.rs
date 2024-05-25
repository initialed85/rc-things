const INPUT_MIN: f32 = -1.0;
const INPUT_MAX: f32 = 1.0;

const PERIOD_MIN_S: f32 = 0.0010;
const PERIOD_MID_S: f32 = 0.0015;
const PERIOD_MAX_S: f32 = 0.0020;

pub trait PwmSetHandler {
    fn set_throttle(&mut self, duty: u32) -> anyhow::Result<()>;
    fn set_steering(&mut self, duty: u32) -> anyhow::Result<()>;
}

pub struct PwmCar {
    throttle_scale: f32,
    throttle_translation: f32,
    throttle_pwm_mid: u32,
    steering_scale: f32,
    steering_translation: f32,
    steering_pwm_mid: u32,
    pwm_set_handler: Box<dyn PwmSetHandler>,
}

impl PwmCar {
    pub fn new(
        throttle_freq_hz: u32,
        throttle_max_duty: u32,
        throttle_invert: bool,
        steering_freq_hz: u32,
        steering_max_duty: u32,
        steering_invert: bool,
        pwm_set_handler: Box<dyn PwmSetHandler>,
    ) -> Self {
        assert_ne!(throttle_freq_hz, 0);
        assert_ne!(throttle_max_duty, 0);

        let throttle_freq_hz = throttle_freq_hz as f32;
        let throttle_max_duty = throttle_max_duty as f32;
        let throttle_pwm_min: f32 = PERIOD_MIN_S / (1.0 / throttle_freq_hz) * throttle_max_duty;
        let throttle_pwm_mid: u32 =
            (PERIOD_MID_S / (1.0 / throttle_freq_hz) * throttle_max_duty) as u32;
        let throttle_pwm_max: f32 = PERIOD_MAX_S / (1.0 / throttle_freq_hz) * throttle_max_duty;
        let mut throttle_scale: f32 =
            (throttle_pwm_max - throttle_pwm_min) / (INPUT_MAX - INPUT_MIN);
        let throttle_translation: f32 =
            throttle_pwm_max - ((throttle_pwm_max - throttle_pwm_min) / (INPUT_MAX - INPUT_MIN));

        if throttle_invert {
            throttle_scale = -throttle_scale;
        }

        println!(
            "throttle_freq_hz={:?}, throttle_max_duty={:?}, throttle_pwm_max={:?}, throttle_pwm_mid={:?}, throttle_pwm_min={:?}, throttle_scale={:?}, throttle_translation={:?}",
            throttle_freq_hz,
            throttle_max_duty,
            throttle_pwm_max,
            throttle_pwm_mid,
            throttle_pwm_min,
            throttle_scale,
            throttle_translation,
        );

        assert_ne!(steering_freq_hz, 0);
        assert_ne!(steering_max_duty, 0);

        let steering_freq_hz = steering_freq_hz as f32;
        let steering_max_duty = steering_max_duty as f32;
        let steering_pwm_min: f32 = PERIOD_MIN_S / (1.0 / steering_freq_hz) * steering_max_duty;
        let steering_pwm_mid: u32 =
            (PERIOD_MID_S / (1.0 / steering_freq_hz) * steering_max_duty) as u32;
        let steering_pwm_max: f32 = PERIOD_MAX_S / (1.0 / steering_freq_hz) * steering_max_duty;
        let mut steering_scale: f32 =
            (steering_pwm_max - steering_pwm_min) / (INPUT_MAX - INPUT_MIN);
        let steering_translation: f32 =
            steering_pwm_max - ((steering_pwm_max - steering_pwm_min) / (INPUT_MAX - INPUT_MIN));

        if steering_invert {
            steering_scale = -steering_scale;
        }

        println!(
            "steering_freq_hz={:?}, steering_max_duty={:?}, steering_pwm_max={:?}, steering_pwm_mid={:?}, steering_pwm_min={:?}, steering_scale={:?}, steering_translation={:?}",
            steering_freq_hz,
            steering_max_duty,
            steering_pwm_max,
            steering_pwm_mid,
            steering_pwm_min,
            steering_scale,
            steering_translation,
        );

        Self {
            // TODO: not sure if this is correct or if my wiring is incorrect
            throttle_scale,
            throttle_translation,
            throttle_pwm_mid,
            steering_scale,
            steering_translation,
            steering_pwm_mid,
            pwm_set_handler,
        }
    }
}

impl crate::vehicle::InputMessageHandler for PwmCar {
    fn handle_input_message(
        &mut self,
        input_message: rc_messaging::serialization::InputMessage,
    ) -> anyhow::Result<()> {
        let mut throttle_pwm = self.throttle_pwm_mid;
        if input_message.throttle != 0.0 {
            throttle_pwm =
                (self.throttle_scale * input_message.throttle + self.throttle_translation) as u32;
        }

        let mut steering_pwm = self.steering_pwm_mid;
        if input_message.steering != 0.0 {
            steering_pwm =
                (self.steering_scale * input_message.steering + self.steering_translation) as u32;
        }

        println!(
            "throttle_pwm={:?}, steering_pwm={:?} from input_message={:?}",
            throttle_pwm, steering_pwm, input_message
        );

        self.pwm_set_handler.set_throttle(throttle_pwm)?;
        self.pwm_set_handler.set_steering(steering_pwm)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rc_messaging::serialization::InputMessage;

    use crate::vehicle::InputMessageHandler;

    use super::*;

    struct TestPwmCar {
        input_states: std::sync::Arc<std::sync::Mutex<Vec<(u32, u32)>>>,
    }

    impl PwmSetHandler for TestPwmCar {
        fn set_throttle(&mut self, duty: u32) -> anyhow::Result<()> {
            let mut input_states = self.input_states.lock().unwrap();

            let mut steering = 0;
            let state = input_states.last();
            if let Some(state) = state {
                steering = state.1;
            }

            input_states.push((duty, steering));

            Ok(())
        }

        fn set_steering(&mut self, duty: u32) -> anyhow::Result<()> {
            let mut input_states = self.input_states.lock().unwrap();

            let mut throttle = 0;
            let state = input_states.last();
            if let Some(state) = state {
                throttle = state.0;
            }

            input_states.push((throttle, duty));

            Ok(())
        }
    }

    fn get_test_resources(freq: u32, max_duty: u32) -> (impl Fn() -> Vec<(u32, u32)>, PwmCar) {
        let shareable_input_states = std::sync::Arc::new(std::sync::Mutex::new(vec![]));

        let closure_input_states = std::sync::Arc::clone(&shareable_input_states);

        let drain_input_states = move || {
            let mut input_states = closure_input_states.lock().unwrap();
            let cloned_input_states = input_states.clone();
            input_states.clear();
            cloned_input_states
        };

        let test_pwm_car = TestPwmCar {
            input_states: Arc::clone(&shareable_input_states),
        };

        let pwm_car = PwmCar::new(
            freq,
            max_duty,
            false,
            freq,
            max_duty,
            false,
            Box::new(test_pwm_car),
        );

        (drain_input_states, pwm_car)
    }

    fn get_input_message(throttle: f32, steering: f32) -> InputMessage {
        rc_messaging::serialization::InputMessage {
            throttle,
            steering,
            throttle_left: 0.0,
            throttle_right: 0.0,
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
    fn happy_path_50_hz_16_bits() -> anyhow::Result<()> {
        let (drain_input_states, mut pwm_car) = get_test_resources(50, 65535);

        pwm_car.handle_input_message(get_input_message(0.0, 0.0))?;
        let input_states = drain_input_states();
        let last_input_state = input_states.last().unwrap();
        assert_eq!(last_input_state, &(4915, 4915));

        pwm_car.handle_input_message(get_input_message(1.0, 1.0))?;
        let input_states = drain_input_states();
        let last_input_state = input_states.last().unwrap();
        assert_eq!(last_input_state, &(6553, 6553));

        pwm_car.handle_input_message(get_input_message(-1.0, -1.0))?;
        let input_states = drain_input_states();
        let last_input_state = input_states.last().unwrap();
        assert_eq!(last_input_state, &(3276, 3276));

        Ok(())
    }
}
