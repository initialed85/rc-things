mod pi_tank;

fn main() -> anyhow::Result<()> {
    //
    // io
    //

    let uart = std::sync::Arc::new(std::sync::Mutex::new(
        rppal::uart::Uart::with_path("/dev/ttyAMA0", 115200, rppal::uart::Parity::None, 8, 1)
            .unwrap(),
    ));

    //
    // vehicle control
    //

    let (incoming_input_message_sender, incoming_input_message_receiver) =
        std::sync::mpsc::channel();

    // run a thread to handle Server
    std::thread::spawn(move || -> anyhow::Result<()> {
        // Server converts UDP datagrams to InputMessages
        let server = rc_messaging::transport::Server::new(
            format!("{}:{}", "0.0.0.0", 13337).parse()?,
            incoming_input_message_sender,
        )?;

        server.run()?;

        Ok(())
    });

    // run a thread to handle PiTank -> StringCar -> Vehicle
    std::thread::spawn(move || -> anyhow::Result<()> {
        // PiTank converts set_throttles calls to serial writes
        let pi_tank = pi_tank::PiTank::new(uart);

        // StringFormatTank converts InputMessages to set_throttles calls
        let string_format_tank = rc_vehicle::string::StringFormatTank::new(true, Box::new(pi_tank));

        // Vehicle rate limits InputMessages / sets safe InputMessage as applicable
        let mut vehicle = rc_vehicle::vehicle::Vehicle::new(
            incoming_input_message_receiver,
            Box::new(string_format_tank),
            -0.20,
            0.20,
            0.0,
        );

        vehicle.run()?;

        Ok(())
    });

    // main loop does nothing
    loop {
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
