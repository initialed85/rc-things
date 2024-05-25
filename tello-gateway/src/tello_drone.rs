use anyhow::anyhow;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct TelloDroneState {
    pitch: f32,
    nick: f32,
    roll: f32,
    yaw: f32,
    flying_desired: bool,
    flying_actual: bool,
}

pub struct TelloDrone {
    drone: Arc<Mutex<tello::Drone>>,
    state: Arc<Mutex<TelloDroneState>>,
}

impl TelloDrone {
    pub fn new(drone: Arc<Mutex<tello::Drone>>) -> Self {
        let state = Arc::new(Mutex::new(TelloDroneState {
            pitch: 0.0,
            nick: 0.0,
            roll: 0.0,
            yaw: 0.0,
            flying_desired: false,
            flying_actual: false,
        }));

        Self { drone, state }
    }
}

impl rc_vehicle::drone::DroneSetHandler for TelloDrone {
    fn set_throttle_left(&mut self, value: f32) -> anyhow::Result<()> {
        let mut state = self.state.lock().unwrap();
        state.pitch = value;
        Ok(())
    }

    fn set_throttle_right(&mut self, value: f32) -> anyhow::Result<()> {
        let mut state = self.state.lock().unwrap();
        state.nick = value;
        Ok(())
    }

    fn set_steering_left(&mut self, value: f32) -> anyhow::Result<()> {
        let mut state = self.state.lock().unwrap();
        state.yaw = value;
        Ok(())
    }

    fn set_steering_right(&mut self, value: f32) -> anyhow::Result<()> {
        let mut state = self.state.lock().unwrap();
        state.roll = value;
        Ok(())
    }

    fn set_flying(&mut self, value: bool) -> anyhow::Result<()> {
        let mut state = self.state.lock().unwrap();
        state.flying_desired = value;
        Ok(())
    }

    fn send_state(&mut self) -> anyhow::Result<()> {
        let mut state = self.state.lock().unwrap();
        let drone = self.drone.lock().unwrap();

        println!("state: {:?}", state);

        let res = drone.send_stick(state.pitch, state.nick, state.roll, state.yaw, true);
        if res.is_err() {
            return Err(anyhow!("failed to send_stick: {:?}", res.unwrap_err()));
        }

        if state.flying_actual != state.flying_desired {
            if state.flying_desired {
                let res = drone.take_off();
                if res.is_err() {
                    return Err(anyhow!("failed to take_off: {:?}", res.unwrap_err()));
                }
                state.flying_actual = true;
            } else {
                let res = drone.land();
                if res.is_err() {
                    return Err(anyhow!("failed to land: {:?}", res.unwrap_err()));
                }
                state.flying_actual = false;
            }
        }

        Ok(())
    }
}
