use bevy::{
    ecs::prelude::*,
    input::{keyboard::KeyCode, mouse::MouseWheel, Input},
    prelude::EventReader,
    render::camera::{Camera, OrthographicProjection},
    time::Timer,
};
use bevy_inspector_egui::Inspectable;

use crate::debug::DebugSettings;

#[derive(PartialEq, Eq, Debug, Hash, Clone, SystemLabel)]
pub enum InputLabel {
    ControllableUpdate,
}

#[derive(Component, Debug, Inspectable)]
pub struct Controllable {
    pub max_speed: f32,
    pub jump_velocity: f32,
    pub acceleration: f32,
    pub camera_follow: bool,
    pub left: bool,
    pub right: bool,
    pub jumping: bool,
    pub interacting: bool,
    pub ability: bool,
    #[inspectable(ignore)]
    pub ability_timer: Timer,
    #[inspectable(ignore)]
    pub jump_buffer: Timer,
}

impl Controllable {
    pub fn new() -> Self {
        Controllable {
            max_speed: 100.0,
            jump_velocity: 200.0,
            acceleration: 400.0,
            camera_follow: true,
            left: false,
            right: false,
            jumping: false,
            interacting: false,
            ability: false,
            ability_timer: Timer::from_seconds(0.3, false),
            jump_buffer: Timer::from_seconds(0.2, false),
        }
    }
}

pub fn system(
    keyboard_input: Res<Input<KeyCode>>,
    debug_settings: Res<DebugSettings>,
    mut query: Query<&mut Controllable>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut OrthographicProjection, &Camera)>,
) {
    for mut c in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            c.left = true;
        } else {
            c.left = false;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            c.right = true;
        } else {
            c.right = false;
        }

        if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::Z) {
            c.jumping = true;
        } else {
            c.jumping = false;
        }

        if keyboard_input.pressed(KeyCode::X) {
            c.ability = true;
        } else {
            c.ability = false;
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            c.interacting = true;
        } else {
            c.interacting = false;
        }
    }

    for event in mouse_wheel_events.iter() {
        if debug_settings.unlock_camera {
            let delta_scale = event.y / 10.0;
            for (mut projection, _camera) in camera_query.iter_mut() {
                projection.scale *= 1.0 + delta_scale / 2.0;
            }
        }
    }
}
