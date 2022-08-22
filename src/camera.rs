use bevy::prelude::{Added, Camera, Changed, OrthographicProjection, Query, Transform, Without};

use crate::input::Controllable;

pub fn follow(
    mut query: Query<(&Transform, &Controllable), (Changed<Transform>, Without<Camera>)>,
    mut camera: Query<(&mut Transform, &Camera)>,
) {
    for (transform, c) in query.iter_mut() {
        if c.camera_follow {
            for (mut camera_transform, _camera) in camera.iter_mut() {
                camera_transform.translation.x = transform.translation.x;
                camera_transform.translation.y = transform.translation.y;
            }
        }
    }
}

pub fn set_zoom(mut query: Query<&mut OrthographicProjection, Added<Camera>>) {
    for mut projection in query.iter_mut() {
        projection.scale = 0.3;
    }
}
