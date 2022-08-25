use bevy::{
    prelude::{Bundle, Component},
    sprite::SpriteSheetBundle,
};
use bevy_ecs_ldtk::LdtkEntity;

use crate::{animation::Animated, damage::Hurtbox, physics::PhysicsObjectBundle};

#[derive(Component, Default)]
pub struct Lava;

#[derive(Bundle, LdtkEntity)]
pub struct LavaBundle {
    lava: Lava,
    #[bundle]
    #[sprite_sheet_bundle("sprites/lava.png", 16.0, 16.0, 9, 1, 0.0, 0.0, 0)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub hurtbox: Hurtbox,
    #[bundle]
    #[from_entity_instance]
    pub physics_bundle: PhysicsObjectBundle,
    #[from_entity_instance]
    pub animated: Animated,
}
