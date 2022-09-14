use bevy::{
    prelude::{
        Added, AssetServer, Assets, Bundle, Changed, Component, Handle, IVec2, Image, Query,
        Transform, Vec2,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};
use bevy_ecs_ldtk::{
    prelude::{FieldValue, LayerInstance, LdtkEntity, TilesetDefinition},
    utils::ldtk_pixel_coords_to_translation_pivoted,
    EntityInstance,
};
use heron::Velocity;

use crate::{animation::Animated, damage::Hurtbox, physics::PhysicsObjectBundle};

use super::Flamable;

const MOVEMENT_SPEED: f32 = 20.0;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component, Default)]
pub struct Patrol {
    pub points: Option<(Vec2, Vec2)>,
    pub movement_speed: f32,
    pub face_left: bool,
}

impl LdtkEntity for Patrol {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Patrol {
        let start = ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        );

        let field_instances = &entity_instance.field_instances;

        let face_left = if let Some(ldtk_patrol) =
            field_instances.iter().find(|f| f.identifier == *"FaceLeft")
        {
            if let FieldValue::Bool(face_left) = &ldtk_patrol.value {
                *face_left
            } else {
                false
            }
        } else {
            false
        };

        if let Some(ldtk_patrol) = field_instances.iter().find(|f| f.identifier == *"Point") {
            if let FieldValue::Point(Some(ldtk_point)) = &ldtk_patrol.value {
                let pixel_coords = (ldtk_point.as_vec2() + Vec2::new(0.5, 1.))
                    * Vec2::splat(layer_instance.grid_size as f32);

                let end = ldtk_pixel_coords_to_translation_pivoted(
                    pixel_coords.as_ivec2(),
                    layer_instance.c_hei * layer_instance.grid_size,
                    IVec2::new(entity_instance.width, entity_instance.height),
                    entity_instance.pivot,
                ) - 8.0;
                Patrol {
                    points: Some((start, end)),
                    movement_speed: MOVEMENT_SPEED,
                    face_left,
                }
            } else {
                Patrol {
                    points: None,
                    movement_speed: MOVEMENT_SPEED,
                    face_left,
                }
            }
        } else {
            Patrol {
                points: None,
                movement_speed: MOVEMENT_SPEED,
                face_left,
            }
        }
    }
}

#[derive(Component, Eq, PartialEq)]
pub enum AnimationState {
    Idle,
    Walking,
    Death,
}
impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Idle
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct GoblinBundle {
    #[bundle]
    #[sprite_sheet_bundle("sprites/goblin_spritesheet.png", 16.0, 16.0, 6, 5, 0.0, 0.0, 0)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub enemy: Enemy,
    pub flamable: Flamable,
    pub hurtbox: Hurtbox,
    #[bundle]
    #[from_entity_instance]
    pub physics_bundle: PhysicsObjectBundle,
    #[from_entity_instance]
    pub animated: Animated,
    pub state: AnimationState,
    #[ldtk_entity]
    pub patrol: Patrol,
}

pub fn init_animation_state(mut query: Query<(&Patrol, &mut AnimationState), Added<Patrol>>) {
    for (patrol, mut state) in query.iter_mut() {
        if patrol.points.is_some() {
            *state = AnimationState::Walking;
        } else {
            *state = AnimationState::Idle;
        }
    }
}

pub fn patrol(mut query: Query<(&Patrol, &mut Velocity, &Transform, &mut TextureAtlasSprite)>) {
    for (patrol, mut velocity, transform, mut sprite) in query.iter_mut() {
        if let Some((start, end)) = patrol.points {
            if transform.translation.x < start.x.min(end.x) || velocity.linear.x == 0.0 {
                velocity.linear.x = patrol.movement_speed;
                sprite.flip_x = false;
            } else if transform.translation.x > start.x.max(end.x) {
                velocity.linear.x = -patrol.movement_speed;
                sprite.flip_x = true;
            }
        }
    }
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
                animation.start = 18;
                animation.end = 22;
            }
            AnimationState::Walking => {
                animation.start = 0;
                animation.end = 6;
            }
            AnimationState::Death => {
                animation.start = 6;
                animation.end = 12;
                animation.play_once = true;
            }
        }
        atlas.index = animation.start;
    }
}

pub fn face_direction(mut query: Query<(&mut TextureAtlasSprite, &Patrol), Added<Patrol>>) {
    for (mut sprite, patrol) in query.iter_mut() {
        sprite.flip_x = patrol.face_left;
    }
}
