use anyhow::Context;
use embedded_svc::wifi::{AccessPointConfiguration, AuthMethod, Configuration, Protocol};
use esp_idf_hal::gpio::*;
use esp_idf_hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};
use esp_idf_sys::*;

mod esp32_truck;

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
    let throttle_forward_enable = PinDriver::output(peripherals.pins.gpio17)?;
    let throttle_reverse_enable = PinDriver::output(peripherals.pins.gpio16)?;
    let tray_forward_enable = PinDriver::output(peripherals.pins.gpio26)?;
    let tray_reverse_enable = PinDriver::output(peripherals.pins.gpio27)?;

    let throttle_and_steering_and_tray_timer_config = TimerConfig {
        frequency: PWM_FREQ_HZ.Hz(),
        resolution: Resolution::Bits20,
        ..Default::default()
    };

    let throttle_and_steering_and_tray_timer_driver = std::sync::Arc::new(LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &throttle_and_steering_and_tray_timer_config,
    )?);

    let throttle_driver = LedcDriver::new(
        peripherals.ledc.channel0,
        std::sync::Arc::clone(&throttle_and_steering_and_tray_timer_driver),
        peripherals.pins.gpio4,
    )?;
    let steering_driver = LedcDriver::new(
        peripherals.ledc.channel1,
        std::sync::Arc::clone(&throttle_and_steering_and_tray_timer_driver),
        peripherals.pins.gpio5,
    )?;
    let tray_driver = LedcDriver::new(
        peripherals.ledc.channel2,
        std::sync::Arc::clone(&throttle_and_steering_and_tray_timer_driver),
        peripherals.pins.gpio14,
    )?;

    //
    // wifi
    //

    let mut wifi_driver = EspWifi::new(peripherals.modem, sys_loop, Some(nvs))?;

    wifi_driver.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
        ssid: "esp32-rc-truck".into(),
        protocols: Protocol::P802D11BGN.into(),
        auth_method: AuthMethod::WPA2Personal,
        password: "truck123!@#".into(),
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

    let (incoming_input_message_sender, incoming_input_message_receiver) =
        std::sync::mpsc::channel();

    // run a thread to handle Server
    std::thread::Builder::new()
        .stack_size(16384)
        .spawn(move || -> anyhow::Result<()> {
            // Server converts UDP datagrams to InputMessages
            let server = rc_messaging::transport::Server::new(
                format!("{}:{}", ip_info.ip.to_string(), 13337).parse()?,
                incoming_input_message_sender,
            )?;

            server.run()?;

            Ok(())
        })?;

    let throttle_max_duty: u32 = throttle_driver.get_max_duty();
    let steering_max_duty: u32 = steering_driver.get_max_duty();
    let tray_max_duty: u32 = tray_driver.get_max_duty();

    // run a thread to handle Esp32Car -> PwmCar -> Vehicle
    std::thread::Builder::new()
        .stack_size(16384)
        .spawn(move || -> anyhow::Result<()> {
            // Esp32Truck converts InputMessages to L298N PWM / enable signals and to servo PWM for
            // steering
            let esp32_truck = esp32_truck::Esp32Truck::new(
                throttle_driver,
                throttle_forward_enable,
                throttle_reverse_enable,
                throttle_max_duty,
                steering_driver,
                PWM_FREQ_HZ,
                steering_max_duty,
                true,
                tray_driver,
                tray_forward_enable,
                tray_reverse_enable,
                tray_max_duty,
            );

            // Vehicle rate limits InputMessages / sets safe InputMessage as applicable
            let mut vehicle = rc_vehicle::vehicle::Vehicle::new(
                incoming_input_message_receiver,
                Box::new(esp32_truck),
                -0.20,
                0.20,
                0.0,
            );

            vehicle.run()?;

            Ok(())
        })?;

    // main loop just blinks the led
    loop {
        led.set_high()?;
        std::thread::sleep(std::time::Duration::from_millis(200));

        led.set_low()?;
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
