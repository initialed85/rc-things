use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use bevy::app::App;
use bevy::log::LogPlugin;
use bevy::math::Vec2;
use bevy::prelude::WindowPosition::At;
use bevy::prelude::{
    default, Axis, Camera2dBundle, Commands, Component, GamepadAxis, GamepadAxisType,
    GamepadButton, GamepadButtonType, Gamepads, Input, IntoSystemDescriptor, NonSend, PluginGroup,
    Res, ResMut, Resource, SystemSet, WindowDescriptor, WindowPlugin,
};
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use iyes_loopless::prelude::AppLooplessFixedTimestepExt;

use rc_things::{get_socket_addr_from_env, serialize, InputMessage};

pub const TITLE: &str = "car-client";
pub const BOUNDS: Vec2 = Vec2::new(640.0, 400.0);
pub const LOCAL_TIME_STEP: f64 = 1.0 / 60.0;
pub const LOCAL_TIME_STEP_NAME: &str = "local_time_step";
pub const NETWORK_TIME_STEP: f64 = 1.0 / 15.0;
pub const NETWORK_TIME_STEP_NAME: &str = "network_time_step";

#[derive(Component, Debug)]
struct MainCamera;

#[derive(Resource, Debug)]
struct InputState {
    pub last_input_message: Option<InputMessage>,
    pub is_handled: bool,
}

fn handle_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn handle_input(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut input_state: ResMut<InputState>,
) {
    for gamepad in gamepads.iter() {
        let left_drive = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();

        let right_drive = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
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

        let input_message = InputMessage {
            throttle: 0.0,
            brake: 0.0,
            steering: 0.0,
            handbrake: false,
            up,
            down,
            left_drive: -left_drive,
            right_drive: -right_drive,
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

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: TITLE.to_string(),
                    width: BOUNDS.x,
                    height: BOUNDS.y,
                    present_mode: PresentMode::Fifo,
                    position: At(Vec2::new(0.0, 0.0)),
                    ..default()
                },
                ..default()
            })
            .set(LogPlugin {
                filter: "car_client=trace,wgpu_core=warn,bevy_render=warn".into(),
                level: bevy::log::Level::INFO,
            }),
    );

    app.add_fixed_timestep(
        Duration::from_secs_f64(LOCAL_TIME_STEP as f64),
        LOCAL_TIME_STEP_NAME,
    );

    app.add_fixed_timestep(
        Duration::from_secs_f64(NETWORK_TIME_STEP as f64),
        NETWORK_TIME_STEP_NAME,
    );

    app.add_fixed_timestep_system_set(LOCAL_TIME_STEP_NAME, 0, SystemSet::default());

    app.add_startup_system(handle_setup);

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
