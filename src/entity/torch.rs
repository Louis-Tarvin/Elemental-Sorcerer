use bevy::{
    prelude::{Added, Bundle, Component, Query, Transform},
    sprite::SpriteSheetBundle,
};
use bevy_ecs_ldtk::LdtkEntity;

use crate::animation::Animated;

#[derive(Default, Component)]
pub struct Torch;

#[derive(Bundle, LdtkEntity)]
pub struct TorchBundle {
    torch: Torch,
    #[bundle]
    #[sprite_sheet_bundle("sprites/torch.png", 8.0, 24.0, 12, 1, 0.0, 0.0, 0)]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    animated: Animated,
}

pub fn offset(mut query: Query<&mut Transform, Added<Torch>>) {
    for mut transform in query.iter_mut() {
        transform.translation.y += 4.0;
    }
}
