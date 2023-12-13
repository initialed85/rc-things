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
use bevy::prelude::{
    Axis, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType, Gamepads, Input,
    IntoSystemDescriptor, NonSend, Res, ResMut, Resource, SystemSet, TransformPlugin, WindowPlugin,
};
use bevy::time::TimePlugin;
use bevy::winit::WinitPlugin;
use iyes_loopless::prelude::AppLooplessFixedTimestepExt;

use rc_messaging::serialization::{serialize, InputMessage};

pub const TITLE: &str = "car-client";
pub const BOUNDS: Vec2 = Vec2::new(640.0, 400.0);
pub const LOCAL_TIME_STEP: f64 = 1.0 / 60.0;
pub const LOCAL_TIME_STEP_NAME: &str = "local_time_step";
pub const NETWORK_TIME_STEP: f64 = 1.0 / 20.0;
pub const NETWORK_TIME_STEP_NAME: &str = "network_time_step";

pub fn get_socket_addr_from_env() -> SocketAddr {
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();
    let socket_addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    socket_addr
}

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

        let throttle_left = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();

        let throttle_right = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
            .unwrap();

        let mut handbrake: bool = false;
        let mut mode_up: bool = false;
        let mut mode_down: bool = false;
        let mut mode_left: bool = false;
        let mut mode_right: bool = false;

        for button_input in button_inputs.get_pressed() {
            if button_input.button_type == GamepadButtonType::South {
                handbrake = true;
            } else if button_input.button_type == GamepadButtonType::DPadUp {
                mode_up = true;
            } else if button_input.button_type == GamepadButtonType::DPadDown {
                mode_down = true;
            } else if button_input.button_type == GamepadButtonType::DPadLeft {
                mode_left = true;
            } else if button_input.button_type == GamepadButtonType::DPadRight {
                mode_right = true;
            }
        }

        let input_message = InputMessage {
            throttle: throttle - brake,
            throttle_left,
            throttle_right,
            steering,
            handbrake,
            mode_up,
            mode_down,
            mode_left,
            mode_right,
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

    let input_message_data = serialize(input_message.clone()).unwrap();

    let _ = socket.send(input_message_data.to_vec().as_slice());
}

fn main() {
    let mut app = App::new();

    app.add_plugin(LogPlugin {
        filter: "robot_client=trace,wgpu_core=warn,bevy_render=warn".into(),
        level: log::Level::INFO,
    });
    app.add_plugin(CorePlugin::default());
    app.add_plugin(TimePlugin);
    app.add_plugin(TransformPlugin);
    app.add_plugin(HierarchyPlugin);
    app.add_plugin(DiagnosticsPlugin);
    app.add_plugin(InputPlugin);
    app.add_plugin(WindowPlugin::default());
    app.add_plugin(WinitPlugin);
    app.add_plugin(GilrsPlugin);

    app.add_fixed_timestep(
        Duration::from_secs_f64(LOCAL_TIME_STEP),
        LOCAL_TIME_STEP_NAME,
    );

    app.add_fixed_timestep(
        Duration::from_secs_f64(NETWORK_TIME_STEP),
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
