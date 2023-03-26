use std::default::Default;
use std::time::Duration;

use bevy::app::App;
use bevy::hierarchy::BuildChildren;
use bevy::log::LogPlugin;
use bevy::math::Vec2;
use bevy::prelude::WindowPosition::At;
use bevy::prelude::{
    info, AlignItems, Assets, Axis, Camera2dBundle, Commands, Component, Entity, GamepadAxis,
    GamepadAxisType, GamepadButton, GamepadButtonType, Gamepads, Handle, Image, ImageBundle, Input,
    IntoSystemDescriptor, JustifyContent, NodeBundle, NonSend, NonSendMut, PluginGroup,
    PositionType, Query, Res, ResMut, Resource, Size, Style, SystemSet, Val, WindowDescriptor,
    WindowPlugin,
};
use bevy::utils::default;
use bevy::window::PresentMode;
use bevy::{log, DefaultPlugins};
use bevy_video::components::VideoDecoder;
use iyes_loopless::prelude::AppLooplessFixedTimestepExt;
use openh264::decoder::Decoder;
use openh264::nal_units;
use tello::{Drone, Message, Package, PackageData};

pub const TITLE: &str = "tello-client";
pub const BOUNDS: Vec2 = Vec2::new(1280.0, 720.0);
pub const LOCAL_TIME_STEP: f64 = 1.0 / 60.0;
pub const LOCAL_TIME_STEP_NAME: &str = "local_time_step";
pub const NETWORK_TIME_STEP: f64 = 1.0 / 20.0;
pub const NETWORK_TIME_STEP_NAME: &str = "network_time_step";

#[derive(Resource, Debug)]
struct InputState {
    pub yaw: f32,
    pub throttle: f32,
    pub roll: f32,
    pub pitch: f32,
    pub motors_on: bool,
    pub motors_off: bool,
    pub is_handled: bool,
}

fn handle_setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());

    let (image_handle, video_decoder) = VideoDecoder::create(&mut images);

    commands.spawn(video_decoder);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(1280.0), Val::Px(720.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let _ = parent
                .spawn(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        ..default()
                    },
                    image: image_handle.into(),
                    ..default()
                })
                .id();
        });
}

fn handle_input(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut input_state: ResMut<InputState>,
) {
    for gamepad in gamepads.iter() {
        let yaw = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();

        let throttle = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
            .unwrap();

        let roll = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
            .unwrap();

        let pitch = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();

        let mut motors_on = false;
        let mut motors_off = false;

        for button_input in button_inputs.get_pressed() {
            if button_input.button_type == GamepadButtonType::DPadUp {
                motors_on = true;
            } else if button_input.button_type == GamepadButtonType::DPadDown {
                motors_off = true;
            }
        }
        input_state.yaw = yaw;
        input_state.throttle = throttle;
        input_state.roll = roll;
        input_state.pitch = pitch;
        input_state.motors_on = motors_on;
        input_state.motors_off = motors_off;
        input_state.is_handled = false;
    }
}

fn handle_network(input_state: Res<InputState>, drone: NonSend<Drone>) {
    if input_state.is_handled {
        return;
    }

    // info!("input_state={:?}", input_state);

    if input_state.motors_off {
        let _ = drone.land();
    } else if input_state.motors_on {
        let _ = drone.take_off();
    }

    drone
        .send_stick(
            input_state.pitch,
            input_state.throttle,
            input_state.roll,
            input_state.yaw,
            true,
        )
        .unwrap();
}

fn handle_poll(
    mut _frame_bucket: NonSendMut<Vec<u8>>,
    _decoders: Query<&VideoDecoder>,
    mut drone: NonSendMut<Drone>,
) {
    let message = drone.poll();
    if message.is_none() {
        return;
    }

    let _message: Message = message.unwrap();

    // match message {
    //     Message::Data(Package {
    //         data: PackageData::FlightData(_flight_data),
    //         ..
    //     }) => {}
    //     Message::Frame(frame_id, data) => {
    //         info!("frame_id={:?}; data={:?}", frame_id, data.len());
    //
    //         for decoder in decoders.iter() {
    //             decoder.add_video_packet(data.clone());
    //         }
    //     }
    //     _ => (),
    // }
}

fn main() {
    let frame_bucket: Vec<u8> = vec![];
    let decoder = Decoder::new().unwrap();

    let mut drone = Drone::new("192.168.10.1:8889");
    drone.connect(11111);
    // drone.start_video().unwrap();

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
                filter: "".into(),
                level: log::Level::INFO,
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

    app.insert_resource(InputState {
        yaw: 0.0,
        throttle: 0.0,
        roll: 0.0,
        pitch: 0.0,
        motors_on: false,
        motors_off: false,
        is_handled: false,
    });

    app.insert_non_send_resource(frame_bucket);
    app.insert_non_send_resource(decoder);
    app.insert_non_send_resource(drone);

    app.add_startup_system(handle_setup);

    app.add_fixed_timestep_system(LOCAL_TIME_STEP_NAME, 0, handle_input);

    app.add_fixed_timestep_system(
        NETWORK_TIME_STEP_NAME,
        0,
        handle_network.after(handle_input),
    );

    app.add_fixed_timestep_system(LOCAL_TIME_STEP_NAME, 0, handle_poll);

    app.run();
}
