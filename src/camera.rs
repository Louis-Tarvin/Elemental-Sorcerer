use bevy::prelude::{Added, Camera, OrthographicProjection, Query};

pub fn set_zoom(mut query: Query<&mut OrthographicProjection, Added<Camera>>) {
    for mut projection in query.iter_mut() {
        projection.scale = 0.3;
    }
}
