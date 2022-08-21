use bevy::{
    ecs::prelude::*,
    input::{keyboard::KeyCode, mouse::MouseWheel, Input},
    prelude::{EventReader, MouseButton},
    render::camera::{Camera, OrthographicProjection},
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
    pub attacking: bool,
}

impl Controllable {
    pub fn new(
        movement_speed: f32,
        jump_velocity: f32,
        acceleration: f32,
        camera_follow: bool,
    ) -> Self {
        Controllable {
            max_speed: movement_speed,
            jump_velocity,
            acceleration,
            camera_follow,
            up: false,
            down: false,
            left: false,
            right: false,
            jumping: false,
            interacting: false,
            attacking: false,
        }
    }
}

pub fn system(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut debug_settings: ResMut<DebugSettings>,
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

        if keyboard_input.pressed(KeyCode::Space) {
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
            c.attacking = true;
        } else {
            c.attacking = false;
        }

        if keyboard_input.just_pressed(KeyCode::F3) {
            *debug_settings = DebugSettings {
                tile_collisions: true,
                player_collison_box: true,
                hitbox: true,
                triggers: true,
                noclip: true,
            };
        }

        if keyboard_input.just_pressed(KeyCode::F4) {
            *debug_settings = DebugSettings::default();
        }
    }

    for event in mouse_wheel_events.iter() {
        let delta_scale = event.y / 10.0;
        for (mut projection, _camera) in camera_query.iter_mut() {
            projection.scale *= 1.0 + delta_scale / 2.0;
        }
    }
}
