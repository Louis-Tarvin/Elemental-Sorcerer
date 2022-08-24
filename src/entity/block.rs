use bevy::{
    prelude::{Bundle, Component},
    sprite::SpriteBundle,
};
use bevy_ecs_ldtk::LdtkEntity;

use crate::physics::PhysicsObjectBundle;

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
    pub physics_bundle: PhysicsObjectBundle,
}
