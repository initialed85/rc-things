use std::env;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use bevy::app::App;
use bevy::core::CorePlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::gilrs::GilrsPlugin;
use bevy::hierarchy::HierarchyPlugin;
use bevy::input::InputPlugin;
use bevy::log;
use bevy::log::LogPlugin;
use bevy::math::Vec2;
use bevy::prelude::WindowPosition::At;
use bevy::prelude::{
    default, Axis, Commands, Component, GamepadAxis, GamepadAxisType, GamepadButton,
    GamepadButtonType, Gamepads, Input, IntoSystemDescriptor, NonSend, PluginGroup, Res, ResMut,
    Resource, SystemSet, TransformPlugin, WindowDescriptor, WindowPlugin,
};
use bevy::time::TimePlugin;
use bevy::window::PresentMode;
use bevy::winit::WinitPlugin;
use iyes_loopless::prelude::AppLooplessFixedTimestepExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use android_activity::AndroidApp;

pub fn get_socket_addr_from_env() -> SocketAddr {
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();
    let socket_addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    return socket_addr;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMessage {
    pub throttle: f32,
    pub brake: f32,
    pub steering: f32,
    pub handbrake: bool,
    pub up: bool,
    pub down: bool,
    pub left_drive: f32,
    pub right_drive: f32,
}

pub fn serialize<T>(t: T) -> Vec<u8>
where
    T: Serialize,
{
    return rmp_serde::to_vec(&t).unwrap();
}

pub fn deserialize<T>(message: Vec<u8>) -> T
where
    T: DeserializeOwned,
{
    return rmp_serde::from_slice::<T>(message.as_slice()).unwrap();
}

pub const TITLE: &str = "car-client";
pub const BOUNDS: Vec2 = Vec2::new(640.0, 400.0);
pub const LOCAL_TIME_STEP: f64 = 1.0 / 60.0;
pub const LOCAL_TIME_STEP_NAME: &str = "local_time_step";
pub const NETWORK_TIME_STEP: f64 = 1.0 / 30.0;
pub const NETWORK_TIME_STEP_NAME: &str = "network_time_step";

#[derive(Resource, Debug)]
struct InputState {
    pub last_input_message: Option<InputMessage>,
    pub is_handled: bool,
}

fn handle_input(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut input_state: ResMut<InputState>,
) {
    for gamepad in gamepads.iter() {
        let throttle = button_axes
            .get(GamepadButton::new(
                gamepad,
                GamepadButtonType::RightTrigger2,
            ))
            .unwrap();

        let brake = button_axes
            .get(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2))
            .unwrap();

        let steering = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();

        let mut handbrake: bool = false;
        let mut up: bool = false;
        let mut down: bool = false;

        for button_input in button_inputs.get_pressed() {
            if button_input.button_type == GamepadButtonType::South {
                handbrake = true;
            } else if button_input.button_type == GamepadButtonType::DPadUp {
                up = true;
            } else if button_input.button_type == GamepadButtonType::DPadDown {
                down = true;
            }
        }

        let input_message = InputMessage {
            throttle,
            brake,
            steering,
            handbrake,
            up,
            down,
            left_drive: 0.0,
            right_drive: 0.0,
        };

        // TODO: what about multiple gamepads?
        input_state.last_input_message = Some(input_message);
        input_state.is_handled = false;
    }
}

fn handle_network(input_state: Res<InputState>, socket: NonSend<UdpSocket>) {
    if input_state.last_input_message.is_none() {
        return;
    }

    if input_state.is_handled {
        return;
    }

    let input_message = input_state.last_input_message.clone().unwrap();
    println!("input_message={:?}", input_message);

    let input_message_data = serialize(input_message.clone());
    // println!("input_message_data={:?}", input_message_data);

    let _ = socket.send(input_message_data.to_vec().as_slice());
}

pub fn car_client_main() {
    let mut app = App::new();

    app.add_plugin(LogPlugin {
        filter: "robot-client=trace,wgpu_core=warn,bevy_render=warn".into(),
        level: log::Level::INFO,
    });
    app.add_plugin(CorePlugin::default());
    app.add_plugin(TimePlugin::default());
    app.add_plugin(TransformPlugin::default());
    app.add_plugin(HierarchyPlugin::default());
    app.add_plugin(DiagnosticsPlugin::default());
    app.add_plugin(InputPlugin::default());
    app.add_plugin(WindowPlugin::default());
    app.add_plugin(WinitPlugin::default());
    app.add_plugin(GilrsPlugin::default());

    app.add_fixed_timestep(
        Duration::from_secs_f64(LOCAL_TIME_STEP as f64),
        LOCAL_TIME_STEP_NAME,
    );

    app.add_fixed_timestep(
        Duration::from_secs_f64(NETWORK_TIME_STEP as f64),
        NETWORK_TIME_STEP_NAME,
    );

    app.add_fixed_timestep_system_set(LOCAL_TIME_STEP_NAME, 0, SystemSet::default());

    app.insert_resource(InputState {
        last_input_message: None,
        is_handled: false,
    });

    app.add_fixed_timestep_system(LOCAL_TIME_STEP_NAME, 0, handle_input);

    app.add_fixed_timestep_system(
        NETWORK_TIME_STEP_NAME,
        0,
        handle_network.after(handle_input),
    );

    let remote_addr: SocketAddr = format!("{}:{}", "0.0.0.0", "13337").parse().unwrap();
    println!("remote_addr={:?}", remote_addr);

    let local_addr: SocketAddr = if remote_addr.is_ipv4() {
        "0.0.0.0:0"
    } else {
        "[::]:0"
    }
    .parse()
    .unwrap();

    let socket = UdpSocket::bind(local_addr).unwrap();
    socket.connect(remote_addr).unwrap();

    app.insert_non_send_resource(socket);

    app.run();
}

#[no_mangle]
fn android_main(_app: AndroidApp) {
    car_client_main();
}
