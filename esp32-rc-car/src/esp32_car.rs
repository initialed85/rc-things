pub struct Esp32Car {
    throttle_driver: esp_idf_hal::ledc::LedcDriver<'static>,
    steering_driver: esp_idf_hal::ledc::LedcDriver<'static>,
}

impl Esp32Car {
    pub fn new(
        throttle_driver: esp_idf_hal::ledc::LedcDriver<'static>,
        steering_driver: esp_idf_hal::ledc::LedcDriver<'static>,
    ) -> Self {
        return Self {
            throttle_driver,
            steering_driver,
        };
    }
}

impl rc_vehicle::pwm::PwmSetHandler for Esp32Car {
    fn set_throttle(&mut self, duty: u32) -> anyhow::Result<()> {
        self.throttle_driver.set_duty(duty)?;

        return Ok(());
    }

    fn set_steering(&mut self, duty: u32) -> anyhow::Result<()> {
        self.steering_driver.set_duty(duty)?;

        return Ok(());
    }
}
