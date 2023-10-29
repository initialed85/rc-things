use anyhow::Context;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::adc::*;
use esp_idf_hal::adc::config::Config;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::prelude::*;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};
use esp_idf_svc::log::EspLogger;
use esp_idf_sys::*;
use esp_idf_sys::*;

// TODO: can't remove import of esp_idf_sys + link_patches call as of 4.4

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

    let mut adc = AdcDriver::new(peripherals.adc1, &Config::new().calibration(true))?;
    let mut steering_adc_pin: AdcChannelDriver<{ attenuation::DB_11 }, _> = AdcChannelDriver::new(peripherals.pins.gpio36)?;
    let mut throttle_adc_pin: AdcChannelDriver<{ attenuation::DB_11 }, _> = AdcChannelDriver::new(peripherals.pins.gpio39)?;

    //
    // wifi
    //

    let mut wifi_driver = EspWifi::new(peripherals.modem, sys_loop, Some(nvs))?;

    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "esp32-rc-car".into(),
        auth_method: AuthMethod::WPA2Personal,
        password: "car123!@#".into(),
        // ssid: "Get schwifty".into(),
        // auth_method: AuthMethod::WPA2Personal,
        // password: "P@$$w0rd1".into(),
        ..Default::default()
    }))?;

    wifi_driver.start()?;
    wifi_driver.connect()?;

    let wifi_config = wifi_driver.get_configuration()?;
    println!("wifi_config={:?}", wifi_config);

    while !wifi_driver.is_connected()? {
        println!("waiting for wifi_driver.is_connected()...");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    println!("connected.");

    while !wifi_driver.sta_netif().is_up()? {
        println!("waiting for wifi_driver.sta_netif.is_up()...");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    let ip_info = wifi_driver.sta_netif().get_ip_info()?;
    println!("ip_info={:?}", ip_info);

    //
    // vehicle control
    //

    // Client converts InputMessages to UDP datagrams
    let client = rc_messaging::transport::Client::new(
        format!("{}:{}", "192.168.71.1", 13337).parse()?,
    )?;

    let outgoing_input_message_sender: std::sync::mpsc::Sender<rc_messaging::serialization::InputMessage> = client.get_outgoing_input_message_sender();

    // run a thread to handle Client
    std::thread::Builder::new()
        .stack_size(32768).spawn(move || -> anyhow::Result<()> {
        client.run()?;

        return Ok(());
    })?;

    // TODO: move this loop into an abstraction like the other stuff
    // run a thread to handle input
    std::thread::Builder::new()
        .stack_size(32768).spawn(move || -> anyhow::Result<()> {
        loop {
            // 142 (forward) -> 1650 (neutral) -> 2580 (reverse)
            let raw_throttle = adc.read(&mut throttle_adc_pin)?;

            // 197 (left) -> 1830 (center) -> 3134 (right)
            let raw_steering = adc.read(&mut steering_adc_pin)?;

            let mut throttle: f32 = raw_throttle.into();
            throttle = -(((throttle - 142.0) / (3134.0 - 142.0)) * 2.0 - 1.0); // rough translate and scale
            throttle += 0.0075; // adjust out remaining error
            if throttle >= -0.01 && throttle <= 0.01 { // apply deadzone
                throttle = 0.0;
            }

            let mut steering: f32 = raw_steering.into();
            steering = ((steering - 142.0) / (3134.0 - 142.0)) * 2.0 - 1.0; // rough translate and scale
            steering -= 0.129; // adjust out remaining error
            if steering >= -0.01 && steering <= 0.01 { // apply deadzone
                steering = 0.0;
            }

            let input_message = rc_messaging::serialization::InputMessage {
                throttle,
                steering,
                throttle_left: 0.0,
                throttle_right: 0.0,
                mode_up: false,
                mode_down: false,
                mode_left: false,
                mode_right: false,
                handbrake: false,
            };

            println!("throttle: {:?}, steering: {:?}, {:?}", raw_throttle, raw_steering, input_message);

            outgoing_input_message_sender.send(input_message)?;

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    })?;

    // main loop just blinks the led
    loop {
        led.set_high()?;
        std::thread::sleep(std::time::Duration::from_millis(200));

        led.set_low()?;
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
