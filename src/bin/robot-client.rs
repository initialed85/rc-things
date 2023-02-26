use std::f32::consts::PI;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use bevy::app::{App, PluginGroup, PluginGroupBuilder};
use bevy::core::CorePlugin;
use bevy::diagnostic;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::gilrs::GilrsPlugin;
use bevy::hierarchy;
use bevy::hierarchy::HierarchyPlugin;
use bevy::input;
use bevy::input::{Axis, Input, InputPlugin};
use bevy::log;
use bevy::log::LogPlugin;
use bevy::math::Vec2;
use bevy::prelude::WindowPosition::At;
use bevy::prelude::{
    default, Commands, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType, Gamepads,
    IntoSystemDescriptor, NonSend, Res, ResMut, Resource, SystemSet, TransformPlugin,
    WindowDescriptor, WindowPlugin, Windows,
};
use bevy::time::TimePlugin;
use bevy::window::CreateWindow;
use bevy::winit::WinitPlugin;
use bevy::{core, MinimalPlugins};
use iyes_loopless::prelude::AppLooplessFixedTimestepExt;

use rc_things::{get_socket_addr_from_env, serialize, InputMessage};

pub const TITLE: &str = "car-client";
pub const BOUNDS: Vec2 = Vec2::new(640.0, 400.0);
pub const LOCAL_TIME_STEP: f64 = 1.0 / 60.0;
pub const LOCAL_TIME_STEP_NAME: &str = "local_time_step";
pub const NETWORK_TIME_STEP: f64 = 1.0 / 15.0;
pub const NETWORK_TIME_STEP_NAME: &str = "network_time_step";

#[derive(Resource, Debug)]
struct InputState {
    pub last_input_message: Option<InputMessage>,
    pub is_handled: bool,
}

fn handle_input(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut input_state: ResMut<InputState>,
) {
    for gamepad in gamepads.iter() {
        let right_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
            .unwrap();

        let right_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
            .unwrap();

        let left_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();

        let left_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();

        let mut up: bool = false;
        let mut down: bool = false;

        for button_input in button_inputs.get_pressed() {
            if button_input.button_type == GamepadButtonType::DPadUp {
                up = true;
            } else if button_input.button_type == GamepadButtonType::DPadDown {
                down = true;
            }
        }

        // TODO this is still too twitchy at low speeds (but manageable at high speeds)
        // let x_scale = 1.0 - right_y.abs();

        // TODO this is fine while moving, but slow for pivoting
        let x_scale = 0.33;

        // TODO no changes
        // let x_scale = 1.0;

        let mut left_drive = 0.0;
        let mut right_drive = 0.0;

        left_drive = -(right_x * x_scale);
        right_drive = (right_x * x_scale);

        left_drive += right_y;
        right_drive += right_y;

        left_drive = left_drive.clamp(-1.0, 1.0);
        right_drive = right_drive.clamp(-1.0, 1.0);

        let input_message = InputMessage {
            throttle: 0.0,
            brake: 0.0,
            steering: 0.0,
            handbrake: false,
            up,
            down,
            left_drive,
            right_drive,
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

fn main() {
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

    let remote_addr = get_socket_addr_from_env();
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
