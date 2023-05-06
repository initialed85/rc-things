const INPUT_MIN: f32 = -1.0;
const INPUT_MAX: f32 = 1.0;

const PERIOD_MIN_S: f32 = 0.0010;
const PERIOD_MID_S: f32 = 0.0015;
const PERIOD_MAX_S: f32 = 0.0020;

pub struct Esp32Truck {
    throttle_driver: esp_idf_hal::ledc::LedcDriver<'static>,
    throttle_forward_enable: esp_idf_hal::gpio::PinDriver<'static, esp_idf_hal::gpio::Gpio17, esp_idf_hal::gpio::Output>,
    throttle_reverse_enable: esp_idf_hal::gpio::PinDriver<'static, esp_idf_hal::gpio::Gpio16, esp_idf_hal::gpio::Output>,
    throttle_max_duty: u32,
    steering_scale: f32,
    steering_translation: f32,
    steering_pwm_mid: u32,
    steering_driver: esp_idf_hal::ledc::LedcDriver<'static>,
    tray_driver: esp_idf_hal::ledc::LedcDriver<'static>,
    tray_forward_enable: esp_idf_hal::gpio::PinDriver<'static, esp_idf_hal::gpio::Gpio26, esp_idf_hal::gpio::Output>,
    tray_reverse_enable: esp_idf_hal::gpio::PinDriver<'static, esp_idf_hal::gpio::Gpio27, esp_idf_hal::gpio::Output>,
    tray_max_duty: u32,
}

impl Esp32Truck {
    pub fn new(
        throttle_driver: esp_idf_hal::ledc::LedcDriver<'static>,
        throttle_forward_enable: esp_idf_hal::gpio::PinDriver<'static, esp_idf_hal::gpio::Gpio17, esp_idf_hal::gpio::Output>,
        throttle_reverse_enable: esp_idf_hal::gpio::PinDriver<'static, esp_idf_hal::gpio::Gpio16, esp_idf_hal::gpio::Output>,
        throttle_max_duty: u32,
        steering_driver: esp_idf_hal::ledc::LedcDriver<'static>,
        steering_freq_hz: u32,
        steering_max_duty: u32,
        steering_invert: bool,
        tray_driver: esp_idf_hal::ledc::LedcDriver<'static>,
        tray_forward_enable: esp_idf_hal::gpio::PinDriver<'static, esp_idf_hal::gpio::Gpio26, esp_idf_hal::gpio::Output>,
        tray_reverse_enable: esp_idf_hal::gpio::PinDriver<'static, esp_idf_hal::gpio::Gpio27, esp_idf_hal::gpio::Output>,
        tray_max_duty: u32,
    ) -> Self {
        assert_ne!(throttle_max_duty, 0);
        assert_ne!(steering_freq_hz, 0);
        assert_ne!(steering_max_duty, 0);
        assert_ne!(tray_max_duty, 0);

        let steering_freq_hz = steering_freq_hz as f32;
        let steering_max_duty = steering_max_duty as f32;
        let steering_pwm_min: f32 = PERIOD_MIN_S / (1.0 / steering_freq_hz) * steering_max_duty;
        let steering_pwm_mid: u32 = (PERIOD_MID_S / (1.0 / steering_freq_hz) * steering_max_duty) as u32;
        let steering_pwm_max: f32 = PERIOD_MAX_S / (1.0 / steering_freq_hz) * steering_max_duty;
        let mut steering_scale: f32 = (steering_pwm_max - steering_pwm_min) / (INPUT_MAX - INPUT_MIN);
        let steering_translation: f32 = steering_pwm_max - ((steering_pwm_max - steering_pwm_min) / (INPUT_MAX - INPUT_MIN));

        if steering_invert {
            steering_scale = -steering_scale;
        }

        return Self {
            throttle_driver,
            throttle_forward_enable,
            throttle_reverse_enable,
            throttle_max_duty,
            steering_driver,
            steering_scale,
            steering_translation,
            steering_pwm_mid: steering_pwm_mid as u32,
            tray_driver,
            tray_forward_enable,
            tray_reverse_enable,
            tray_max_duty,
        };
    }
}

impl rc_vehicle::vehicle::InputMessageHandler for Esp32Truck {
    fn handle_input_message(&mut self, input_message: rc_messaging::serialization::InputMessage) -> anyhow::Result<()> {
        let mut throttle_pwm = 0;
        let mut throttle_forward_enable = false;
        let mut throttle_reverse_enable = false;

        if input_message.throttle > 0.0 {
            throttle_pwm = (input_message.throttle * self.throttle_max_duty as f32) as u32;
            throttle_forward_enable = true;
        } else if input_message.throttle < 0.0 {
            throttle_pwm = (-input_message.throttle * self.throttle_max_duty as f32) as u32;
            throttle_reverse_enable = true;
        }

        let mut steering_pwm = self.steering_pwm_mid;
        if input_message.steering != 0.0 {
            steering_pwm = (self.steering_scale * input_message.steering + self.steering_translation) as u32;
        }

        let mut tray_pwm = 0;
        let mut tray_forward_enable = false;
        let mut tray_reverse_enable = false;

        if input_message.throttle_right > 0.0 {
            tray_pwm = (input_message.throttle_right * self.tray_max_duty as f32) as u32;
            tray_forward_enable = true;
        } else if input_message.throttle_right < 0.0 {
            tray_pwm = (-input_message.throttle_right * self.tray_max_duty as f32) as u32;
            tray_reverse_enable = true;
        }
        println!(
            "throttle_pwm={:?}, throttle_forward_enable={:?}, throttle_reverse_enable={:?}, steering_pwm={:?}, tray_pwm={:?}, tray_forward_enable={:?}, tray_reverse_enable={:?},  for input_message={:?}",
            throttle_pwm,
            throttle_forward_enable,
            throttle_reverse_enable,
            steering_pwm,
            tray_pwm,
            tray_forward_enable,
            tray_reverse_enable,
            input_message,
        );

        self.throttle_driver.set_duty(throttle_pwm)?;

        if throttle_forward_enable {
            self.throttle_forward_enable.set_high()?;
        } else {
            self.throttle_forward_enable.set_low()?;
        }

        if throttle_reverse_enable {
            self.throttle_reverse_enable.set_high()?;
        } else {
            self.throttle_reverse_enable.set_low()?;
        }

        self.steering_driver.set_duty(steering_pwm)?;

        self.tray_driver.set_duty(tray_pwm)?;

        if tray_forward_enable {
            self.tray_forward_enable.set_high()?;
        } else {
            self.tray_forward_enable.set_low()?;
        }

        if tray_reverse_enable {
            self.tray_reverse_enable.set_high()?;
        } else {
            self.tray_reverse_enable.set_low()?;
        }

        return Ok(());
    }
}
