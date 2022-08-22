use bevy::{
    prelude::{Bundle, Changed, Component, Query, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity, LevelSelection, Worldly};

use crate::{animation::Animated, input::Controllable, physics::PhysicsObjectBundle};

impl From<EntityInstance> for Controllable {
    fn from(_: EntityInstance) -> Self {
        Controllable::new()
    }
}

#[derive(Component, Default)]
pub struct Player {
    pub checkpoint: Vec3,
    pub checkpoint_level: LevelSelection,
}

#[derive(Component, Eq, PartialEq)]
pub enum AnimationState {
    Idle,
    Walking,
    JumpUp,
    JumpDown,
    Death,
}
impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Idle
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[bundle]
    #[sprite_sheet_bundle("sprites/herochar_spritesheet.png", 16.0, 16.0, 8, 15, 0.0, 0.0, 0)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[worldly]
    pub worldly: Worldly,
    pub player: Player,
    #[bundle]
    #[from_entity_instance]
    pub physics_bundle: PhysicsObjectBundle,
    #[from_entity_instance]
    pub controllable: Controllable,
    #[from_entity_instance]
    pub animated: Animated,
    pub animation_state: AnimationState,
}

pub fn animation_state_update(
    mut query: Query<
        (&mut Animated, &AnimationState, &mut TextureAtlasSprite),
        Changed<AnimationState>,
    >,
) {
    for (mut animation, state, mut atlas) in query.iter_mut() {
        match state {
            AnimationState::Idle => {
                animation.start = 40;
                animation.end = 44;
            }
            AnimationState::Walking => {
                animation.start = 8;
                animation.end = 14;
            }
            AnimationState::JumpUp => {
                animation.start = 56;
                animation.end = 59;
            }
            AnimationState::JumpDown => {
                animation.start = 48;
                animation.end = 51;
            }
            AnimationState::Death => {
                animation.start = 0;
                animation.end = 8;
            }
        }
        atlas.index = animation.start;
    }
}
