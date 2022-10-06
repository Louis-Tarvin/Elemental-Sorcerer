use bevy::{
    prelude::{Bundle, Component},
    sprite::SpriteBundle,
};
use bevy_ecs_ldtk::LdtkEntity;
use heron::Acceleration;

use crate::physics::{Dynamic, PhysicsObjectBundle};

use super::Flamable;

#[derive(Component, Default)]
pub struct Block;

#[derive(Bundle, LdtkEntity)]
pub struct BlockBundle {
    block: Block,
    #[bundle]
    #[sprite_bundle("sprites/block.png")]
    sprite_bundle: SpriteBundle,
    #[bundle]
    #[from_entity_instance]
    physics_bundle: PhysicsObjectBundle,
    acceleration: Acceleration,
    dynamic: Dynamic,
}

#[derive(Bundle, LdtkEntity)]
pub struct WoodBlockBundle {
    block: Block,
    flamable: Flamable,
    #[bundle]
    #[sprite_bundle("sprites/wood_block.png")]
    sprite_bundle: SpriteBundle,
    #[bundle]
    #[from_entity_instance]
    physics_bundle: PhysicsObjectBundle,
}
