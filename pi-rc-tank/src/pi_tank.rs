pub struct PiTank {
    uart: std::sync::Arc<std::sync::Mutex<rppal::uart::Uart>>,
}

impl PiTank {
    pub fn new(
        uart: std::sync::Arc<std::sync::Mutex<rppal::uart::Uart>>
    ) -> Self {
        return Self {
            uart,
        };
    }
}

impl rc_vehicle::string::StringFormatHandler for PiTank {
    fn set_throttles(&mut self, throttle_left: f32, throttle_right: f32) -> anyhow::Result<()> {
        let output_message = format!("{:.20},{:.20}\r\n", throttle_left, throttle_right);

        {
            self.uart.lock()
                .unwrap()
                .write(output_message.as_bytes())
                .unwrap();
        }

        return Ok(());
    }
}
