use std::sync::{Arc, Mutex};
use tello::{Drone, Message, Package, PackageData, ResponseMsg};

mod tello_drone;

fn main() -> anyhow::Result<()> {
    //
    // io
    //

    let mut actual_drone = Drone::new("192.168.10.1:8889");
    actual_drone.connect(11111);

    let our_drone = Arc::new(Mutex::new(actual_drone));
    let their_drone = Arc::clone(&our_drone);

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

    // run a thread to handle Tello -> StringCar -> Vehicle
    std::thread::spawn(move || -> anyhow::Result<()> {
        // TelloDrone converts set_throttle_* / set_steering_* calls to Tello API calls
        let tello_drone = tello_drone::TelloDrone::new(their_drone);

        // Drone converts InputMessages to set_throttle_* / set_steering_* calls
        let drone = rc_vehicle::drone::Drone::new(Box::new(tello_drone));

        // Vehicle rate limits InputMessages / sets safe InputMessage as applicable
        let mut vehicle = rc_vehicle::vehicle::Vehicle::new(
            incoming_input_message_receiver,
            Box::new(drone),
            -1.0,
            1.0,
            0.0,
        );

        vehicle.run()?;

        Ok(())
    });

    // main loop does nothing
    loop {
        {
            let mut this_drone = our_drone.lock().unwrap();
            if let Some(msg) = this_drone.poll() {
                match msg {
                    Message::Data(Package {
                        data: PackageData::FlightData(d),
                        ..
                    }) => {
                        println!("d: {:?}", d);
                    }
                    Message::Response(ResponseMsg::Connected(_)) => {
                        println!("connected.");
                    }
                    Message::Frame(frame_id, data) => {
                        println!("frame_id: {:?}, data: {:?}", frame_id, data.len())
                    }
                    _ => (),
                }
            }
        }

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 20));
    }
}
