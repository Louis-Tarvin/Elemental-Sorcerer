use bevy::prelude::{Camera, Changed, Query, Transform, Without};

use crate::input::Controllable;

pub fn follow(
    mut query: Query<(&Transform, &Controllable), (Changed<Transform>, Without<Camera>)>,
    mut camera: Query<(&mut Transform, &Camera)>,
) {
    for (transform, c) in query.iter_mut() {
        if c.camera_follow {
            for (mut camera_transform, _camera) in camera.iter_mut() {
                // if Some(true) == camera.name.as_ref().map(|name| name == CAMERA_2D) {
                camera_transform.translation.x = transform.translation.x;
                camera_transform.translation.y = transform.translation.y;
                // }
            }
        }
    }
}
