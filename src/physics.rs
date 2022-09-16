use std::time::Duration;

use bevy::{
    prelude::{
        Added, Assets, BuildChildren, Bundle, Children, Commands, Component, Entity, EventReader,
        GlobalTransform, Query, Res, ResMut, SystemLabel, Transform, Vec2, Vec3,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer},
};

use bevy_kira_audio::{Audio, AudioControl};
use heron::{
    CollisionEvent, CollisionLayers, CollisionShape, PhysicMaterial, PhysicsLayer, RigidBody,
    RotationConstraints, Velocity,
};

use crate::{
    abilities::{Element, Equipment},
    animation::Animated,
    audio::{AudioAssets, VolumeSettings},
    debug::DebugSettings,
    destruction::DestructionTimer,
    entity::player::{AnimationState, Player},
    input::Controllable,
    state::load_game::GameAssets,
};

#[derive(SystemLabel)]
pub enum PhysicsLabel {
    HandleControllables,
    CheckGrounded,
}

#[derive(PhysicsLayer)]
pub enum PhysicsLayers {
    Terrain,
    PlayerBody,
    PlayerGroundDetector,
    Interactable,
    Enemy,
    Fireball,
    Wind,
    Movable,
    Wood,
    Lava,
    WaterDrop,
    Water,
    Spikes,
}

#[derive(Bundle, Default)]
pub struct PhysicsObjectBundle {
    pub collider: CollisionShape,
    pub rb: RigidBody,
    pub velocity: Velocity,
    pub material: PhysicMaterial,
    pub rot_constraints: RotationConstraints,
    pub layer: CollisionLayers,
}

pub fn handle_controllables(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        &mut Velocity,
        &Controllable,
        &Player,
        &mut AnimationState,
        &mut TextureAtlasSprite,
        &Transform,
        &Children,
    )>,
    mut ground_detectors: Query<&mut GroundDetector>,
    game_assets: Res<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    debug_settings: Res<DebugSettings>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    volume_settings: Res<VolumeSettings>,
) {
    for (
        mut velocity,
        controllable,
        player,
        mut animation,
        mut texture_atlas,
        transform,
        children,
    ) in query.iter_mut()
    {
        let Controllable {
            left,
            right,
            jumping,
            max_speed,
            jump_velocity,
            acceleration,
            ..
        } = *controllable;

        // get the ground detector to see if the player is grounded
        for &child in children.iter() {
            if let Ok(mut detector) = ground_detectors.get_mut(child) {
                if jumping && (!detector.coyote_timer.finished() || debug_settings.flying) {
                    if player.has_equipt(Equipment::MagicBoots) && player.has_infused(Element::Fire)
                    {
                        velocity.linear.y = jump_velocity * 1.3;
                        let texture_handle = game_assets.explosion.clone();
                        let texture_atlas =
                            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 10, 1);
                        let texture_atlas_handle = texture_atlases.add(texture_atlas);
                        let mut transform = Transform::from_translation(transform.translation);
                        transform.translation.y += 8.0;
                        commands
                            .spawn()
                            .insert_bundle(SpriteSheetBundle {
                                texture_atlas: texture_atlas_handle,
                                transform,
                                ..Default::default()
                            })
                            .insert(Animated::new(0.05, 0, 10, true))
                            .insert(DestructionTimer(Timer::from_seconds(0.5, false)));
                        audio
                            .play(audio_assets.explosion.clone())
                            .with_volume(volume_settings.sfx_vol);
                    } else {
                        velocity.linear.y = jump_velocity;
                        audio
                            .play(audio_assets.jump.clone())
                            .with_volume(volume_settings.sfx_vol);
                    }
                    // run out the timer
                    detector.coyote_timer.tick(Duration::from_secs(10.0 as u64));
                } else if jumping
                    && detector.has_double_jump
                    && player.has_equipt(Equipment::MagicBoots)
                    && player.has_infused(Element::Air)
                {
                    // Double jump
                    velocity.linear.y = jump_velocity;
                    detector.has_double_jump = false;
                    let texture_handle = game_assets.poof.clone();
                    let texture_atlas =
                        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 4.0), 3, 1);
                    let texture_atlas_handle = texture_atlases.add(texture_atlas);
                    let mut transform = Transform::from_translation(transform.translation);
                    transform.translation.y -= 8.0;
                    commands
                        .spawn()
                        .insert_bundle(SpriteSheetBundle {
                            texture_atlas: texture_atlas_handle,
                            transform,
                            ..Default::default()
                        })
                        .insert(Animated::new(0.1, 0, 3, true))
                        .insert(DestructionTimer(Timer::from_seconds(0.3, false)));
                    audio
                        .play(audio_assets.air.clone())
                        .with_volume(volume_settings.sfx_vol);
                }

                let acceleration = if detector.is_grounded {
                    acceleration
                } else {
                    acceleration / 2.0
                };

                let max_speed = if player.has_equipt(Equipment::MagicBoots)
                    && player.has_infused(Element::Water)
                {
                    max_speed * 2.0
                } else {
                    max_speed
                };

                if right && !left {
                    if velocity.linear.x < max_speed {
                        let delta = acceleration * time.delta_seconds();
                        velocity.linear.x += delta;
                    }
                    texture_atlas.flip_x = false;
                } else if left && !right {
                    if velocity.linear.x > -max_speed {
                        let delta = acceleration * time.delta_seconds();
                        velocity.linear.x -= delta;
                    }
                    texture_atlas.flip_x = true;
                } else if velocity.linear.x != 0.0 {
                    let delta = acceleration * time.delta_seconds();

                    if velocity.linear.x < 0.0 {
                        velocity.linear.x = (velocity.linear.x + delta).min(0.0)
                    } else {
                        velocity.linear.x = (velocity.linear.x - delta).max(0.0)
                    }
                }
                if velocity.linear.y > 0.1 && !detector.is_grounded {
                    if *animation != AnimationState::JumpUp {
                        *animation = AnimationState::JumpUp;
                    }
                } else if velocity.linear.y < -0.1 && !detector.is_grounded {
                    if *animation != AnimationState::JumpDown {
                        *animation = AnimationState::JumpDown;
                    }
                } else if velocity.linear.x.abs() > 0.05 {
                    if *animation != AnimationState::Walking {
                        *animation = AnimationState::Walking;
                    }
                } else if *animation != AnimationState::Idle {
                    *animation = AnimationState::Idle;
                }
            }
        }
    }
}

#[derive(Default, Component)]
pub struct GroundDetector {
    pub is_grounded: bool,
    pub has_double_jump: bool,
    pub coyote_timer: Timer,
    pub active_collisions: u8,
}

pub fn add_ground_sensor(mut commands: Commands, query: Query<Entity, Added<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn()
                .insert(GroundDetector {
                    is_grounded: false,
                    has_double_jump: false,
                    coyote_timer: Timer::from_seconds(0.1, false),
                    active_collisions: 0,
                })
                .insert(RigidBody::Sensor)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3 {
                        x: 3.0,
                        y: 3.0,
                        z: 1.0,
                    },
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(PhysicsLayers::PlayerGroundDetector)
                        .with_masks([PhysicsLayers::Terrain, PhysicsLayers::Movable]),
                )
                .insert(Transform::from_translation(Vec3 {
                    x: 0.0,
                    y: -6.0,
                    z: 0.0,
                }))
                .insert(GlobalTransform::default());
        });
    }
}

pub fn check_grounded(
    time: Res<Time>,
    mut detectors: Query<(Entity, &mut GroundDetector)>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for (entity, mut ground_detector) in detectors.iter_mut() {
        ground_detector.coyote_timer.tick(time.delta());
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(a, b) => {
                    if a.rigid_body_entity() == entity || b.rigid_body_entity() == entity {
                        ground_detector.active_collisions += 1;
                    }
                }
                CollisionEvent::Stopped(a, b) => {
                    if a.rigid_body_entity() == entity || b.rigid_body_entity() == entity {
                        ground_detector.active_collisions -= 1;
                    }
                }
            }
            if ground_detector.active_collisions > 0 {
                ground_detector.is_grounded = true;
                ground_detector.has_double_jump = true;
                ground_detector.coyote_timer.reset();
                ground_detector.coyote_timer.pause();
            } else {
                ground_detector.is_grounded = false;
                ground_detector.coyote_timer.unpause();
            }
        }
    }
}
