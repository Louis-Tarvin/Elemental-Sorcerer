use bevy::{
    ecs::prelude::*,
    input::{keyboard::KeyCode, mouse::MouseWheel, Input},
    prelude::{EventReader, MouseButton},
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
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jumping: bool,
    pub interacting: bool,
    pub ability: bool,
    #[inspectable(ignore)]
    pub ability_timer: Timer,
}

impl Controllable {
    pub fn new() -> Self {
        Controllable {
            max_speed: 100.0,
            jump_velocity: 200.0,
            acceleration: 400.0,
            camera_follow: true,
            up: false,
            down: false,
            left: false,
            right: false,
            jumping: false,
            interacting: false,
            ability: false,
            ability_timer: Timer::from_seconds(1.0, false),
        }
    }
}

pub fn system(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    debug_settings: Res<DebugSettings>,
    mut query: Query<&mut Controllable>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut OrthographicProjection, &Camera)>,
) {
    for mut c in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            c.left = true;
        } else {
            c.left = false;
        }

        if keyboard_input.pressed(KeyCode::D) {
            c.right = true;
        } else {
            c.right = false;
        }

        if keyboard_input.pressed(KeyCode::W) {
            c.up = true;
        } else {
            c.up = false;
        }

        if keyboard_input.pressed(KeyCode::S) {
            c.down = true;
        } else {
            c.down = false;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            c.jumping = true;
        } else {
            c.jumping = false;
        }

        if keyboard_input.just_pressed(KeyCode::E) {
            c.interacting = true;
        } else {
            c.interacting = false;
        }

        if mouse_input.pressed(MouseButton::Left) {
            c.ability = true;
        } else {
            c.ability = false;
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
