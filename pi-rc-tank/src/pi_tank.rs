const THROTTLE_LEFT_SCALE: f32 = 1.0;
const THROTTLE_RIGHT_SCALE: f32 = 0.99;

pub struct PiTank {
    uart: std::sync::Arc<std::sync::Mutex<rppal::uart::Uart>>,
}

impl PiTank {
    pub fn new(uart: std::sync::Arc<std::sync::Mutex<rppal::uart::Uart>>) -> Self {
        return Self { uart };
    }
}

impl rc_vehicle::string::StringFormatHandler for PiTank {
    fn set_throttles(&mut self, throttle_left: f32, throttle_right: f32) -> anyhow::Result<()> {
        let output_message = format!(
            "{:.20},{:.20}\r\n",
            throttle_left * THROTTLE_LEFT_SCALE,
            throttle_right * THROTTLE_RIGHT_SCALE
        );

        {
            self.uart
                .lock()
                .unwrap()
                .write(output_message.as_bytes())
                .unwrap();
        }

        Ok(())
    }
}
