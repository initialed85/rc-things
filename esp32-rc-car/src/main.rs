use anyhow::Context;
use embedded_svc::wifi::{AccessPointConfiguration, AuthMethod, Configuration, Protocol};
use esp_idf_hal::gpio::*;
use esp_idf_hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};
use esp_idf_svc::log::EspLogger;
use esp_idf_sys::*;

mod esp32_car;

// TODO: can't remove import of esp_idf_sys + link_patches call as of 4.4

const PWM_FREQ_HZ: u32 = 76;

fn main() -> anyhow::Result<()> {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().context("failed Peripherals::take()")?;
    let sys_loop = EspSystemEventLoop::take().context("failed EspSystemEventLoop::take()")?;
    let nvs = EspDefaultNvsPartition::take().context("failed EspDefaultNvsPartition::take()")?;

    //
    // io
    //

    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    let throttle_and_steering_timer_config = TimerConfig {
        frequency: PWM_FREQ_HZ.Hz().into(),
        resolution: Resolution::Bits20,
        ..Default::default()
    };

    let throttle_and_steering_timer_driver = std::sync::Arc::new(LedcTimerDriver::new(peripherals.ledc.timer0, &throttle_and_steering_timer_config)?);
    let throttle_driver = LedcDriver::new(peripherals.ledc.channel0, std::sync::Arc::clone(&throttle_and_steering_timer_driver), peripherals.pins.gpio4)?;
    let steering_driver = LedcDriver::new(peripherals.ledc.channel1, std::sync::Arc::clone(&throttle_and_steering_timer_driver), peripherals.pins.gpio5)?;

    //
    // wifi
    //

    let mut wifi_driver = EspWifi::new(peripherals.modem, sys_loop, Some(nvs))?;

    wifi_driver.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
        ssid: "esp32-rc-car".into(),
        protocols: Protocol::P802D11BGN.into(),
        auth_method: AuthMethod::WPA2Personal,
        password: "car123!@#".into(),
        ..Default::default()
    }))?;

    wifi_driver.start()?;

    let wifi_config = wifi_driver.get_configuration()?;
    println!("wifi_config={:?}", wifi_config);

    while !wifi_driver.is_up()? {
        println!("waiting for wifi_driver.is_up()...");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    println!("connected.");

    let ip_info = wifi_driver.ap_netif().get_ip_info()?;
    println!("ip_info={:?}", ip_info);

    //
    // vehicle control
    //

    let (incoming_input_message_sender, incoming_input_message_receiver) = std::sync::mpsc::channel();

    // run a thread to handle Server
    std::thread::Builder::new()
        .stack_size(16384).spawn(move || -> anyhow::Result<()> {

        // Server converts UDP datagrams to InputMessages
        let server = rc_messaging::transport::Server::new(
            format!("{}:{}", ip_info.ip.to_string(), 13337).parse()?,
            incoming_input_message_sender,
        )?;

        server.run()?;
        return Ok(());
    })?;

    let throttle_freq_hz: u32 = PWM_FREQ_HZ;
    let throttle_max_duty: u32 = throttle_driver.get_max_duty().into();
    let steering_freq_hz: u32 = PWM_FREQ_HZ;
    let steering_max_duty: u32 = steering_driver.get_max_duty().into();

    // run a thread to handle Esp32Car -> PwmCar -> Vehicle
    std::thread::Builder::new()
        .stack_size(16384).spawn(move || -> anyhow::Result<()> {
        // Esp32Car converts set_throttle and set_steering calls to actual peripherals
        let esp32_car = esp32_car::Esp32Car::new(
            throttle_driver,
            steering_driver,
        );

        // PwmCar converts InputMessages to set_throttle and set_steering calls
        let pwm_car = rc_vehicle::pwm::PwmCar::new(
            throttle_freq_hz,
            throttle_max_duty,
            true,
            steering_freq_hz,
            steering_max_duty,
            false,
            Box::new(esp32_car),
        );

        // Vehicle rate limits InputMessages / sets safe InputMessage as applicable
        let mut vehicle = rc_vehicle::vehicle::Vehicle::new(
            incoming_input_message_receiver,
            Box::new(pwm_car),
            -0.10,
            0.10,
            0.0,
        );

        vehicle.run()?;

        return Ok(());
    })?;

    // main loop just blinks the led
    loop {
        led.set_high()?;
        std::thread::sleep(std::time::Duration::from_millis(200));

        led.set_low()?;
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
