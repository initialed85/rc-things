pub trait DroneSetHandler {
    fn set_throttle_left(&mut self, value: f32) -> anyhow::Result<()>;
    fn set_throttle_right(&mut self, value: f32) -> anyhow::Result<()>;
    fn set_steering_left(&mut self, value: f32) -> anyhow::Result<()>;
    fn set_steering_right(&mut self, value: f32) -> anyhow::Result<()>;
    fn set_flying(&mut self, value: bool) -> anyhow::Result<()>;
    fn send_state(&mut self) -> anyhow::Result<()>;
}

pub struct Drone {
    drone_handler: Box<dyn DroneSetHandler>,
}

impl Drone {
    pub fn new(drone_handler: Box<dyn DroneSetHandler>) -> Self {
        Self { drone_handler }
    }
}

impl crate::vehicle::InputMessageHandler for Drone {
    fn handle_input_message(
        &mut self,
        input_message: rc_messaging::serialization::InputMessage,
    ) -> anyhow::Result<()> {
        self.drone_handler
            .set_throttle_left(input_message.throttle_left)?;
        self.drone_handler
            .set_throttle_right(input_message.throttle_right)?;
        self.drone_handler
            .set_steering_left(input_message.steering_left)?;
        self.drone_handler
            .set_steering_right(input_message.steering_right)?;

        if input_message.mode_up {
            self.drone_handler.set_flying(true)?;
        } else if input_message.mode_down {
            self.drone_handler.set_flying(false)?;
        }

        self.drone_handler.send_state()?;

        Ok(())
    }
}
