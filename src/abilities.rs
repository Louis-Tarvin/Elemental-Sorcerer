use std::fmt::Display;

use bevy::{
    prelude::{
        Assets, Commands, Component, DespawnRecursiveExt, Entity, EventReader, GlobalTransform,
        Query, Res, ResMut, Transform, Vec2, Vec3, With, Without,
    },
    sprite::{Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer},
};
use bevy_inspector_egui::Inspectable;
use bevy_kira_audio::{Audio, AudioChannel, AudioControl};
use heron::{
    CollisionEvent, CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity,
};

use crate::{
    animation::Animated,
    audio::{AudioAssets, SoundChannel},
    damage::Hurtbox,
    destruction::DestructionTimer,
    entity::{
        block::Block,
        goblin::{AnimationState, Patrol},
        lava::Lava,
        player::Player,
        Flamable,
    },
    input::Controllable,
    physics::{PhysicsLayers, PhysicsObjectBundle},
    state::load_game::GameAssets,
};

#[derive(Inspectable, Component, PartialEq, Eq, Clone, Copy)]
pub enum Equipment {
    Staff,
    MagicBoots,
    Cloak,
}
impl Display for Equipment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Staff => write!(f, "Staff"),
            Self::MagicBoots => write!(f, "Magic Boots"),
            Self::Cloak => write!(f, "Cloak of Resistance"),
        }
    }
}

#[derive(Inspectable, Component, PartialEq, Eq, Clone, Copy)]
pub enum Element {
    Fire,
    Air,
    Water,
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fire => write!(f, "Fire"),
            Self::Air => write!(f, "Air"),
            Self::Water => write!(f, "Water"),
        }
    }
}

#[derive(Component)]
pub struct FireProjectile;
#[derive(Component)]
pub struct WindProjectile;
#[derive(Component)]
pub struct WaterProjectile;

// TODO: split into multiple systems
pub fn use_ability(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        &mut Controllable,
        &GlobalTransform,
        &Player,
        &TextureAtlasSprite,
    )>,
    game_assets: Res<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sound_channel: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for (mut controllable, transform, player, sprite) in query.iter_mut() {
        controllable.ability_timer.tick(time.delta());
        if controllable.ability
            && controllable.ability_timer.finished()
            && player.has_equipt(Equipment::Staff)
        {
            if player.has_infused(Element::Fire) {
                controllable.ability_timer.reset();
                let vel_x = if sprite.flip_x { -150.0 } else { 150.0 };
                let texture_handle = game_assets.fireball.clone();
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                let mut projectile_sprite = TextureAtlasSprite::default();
                if sprite.flip_x {
                    projectile_sprite.flip_x = true;
                }
                commands
                    .spawn()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: Transform::from_translation(transform.translation()),
                        sprite: projectile_sprite,
                        ..Default::default()
                    })
                    .insert(Animated::new(0.05, 0, 4, false))
                    .insert(FireProjectile)
                    .insert(DestructionTimer(Timer::from_seconds(0.6, false)))
                    .insert_bundle(PhysicsObjectBundle {
                        collider: CollisionShape::Cuboid {
                            half_extends: Vec3 {
                                x: 2.5,
                                y: 0.5,
                                z: 0.0,
                            },
                            border_radius: Some(4.0),
                        },
                        rb: RigidBody::KinematicVelocityBased,
                        rot_constraints: RotationConstraints::lock(),
                        velocity: Velocity::from_linear(Vec3 {
                            x: vel_x,
                            y: 0.0,
                            z: 0.0,
                        }),
                        ..Default::default()
                    })
                    .insert(
                        CollisionLayers::none()
                            .with_group(PhysicsLayers::Fireball)
                            .with_masks([PhysicsLayers::Enemy, PhysicsLayers::Wood]),
                    );
                sound_channel.play(audio_assets.fireball.clone());
            } else if player.has_infused(Element::Air) {
                controllable.ability_timer.reset();
                let vel_x = if sprite.flip_x { -50.0 } else { 50.0 };
                let texture_handle = game_assets.wind.clone();
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 5, 1);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                let mut projectile_sprite = TextureAtlasSprite::default();
                if sprite.flip_x {
                    projectile_sprite.flip_x = true;
                }
                commands
                    .spawn()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: Transform::from_translation(transform.translation()),
                        sprite: projectile_sprite,
                        ..Default::default()
                    })
                    .insert(Animated::new(0.1, 0, 5, false))
                    .insert(WindProjectile)
                    .insert(DestructionTimer(Timer::from_seconds(0.6, false)))
                    .insert_bundle(PhysicsObjectBundle {
                        collider: CollisionShape::Cuboid {
                            half_extends: Vec3 {
                                x: 2.5,
                                y: 0.5,
                                z: 0.0,
                            },
                            border_radius: Some(4.0),
                        },
                        rb: RigidBody::KinematicVelocityBased,
                        rot_constraints: RotationConstraints::lock(),
                        velocity: Velocity::from_linear(Vec3 {
                            x: vel_x,
                            y: 0.0,
                            z: 0.0,
                        }),
                        ..Default::default()
                    })
                    .insert(
                        CollisionLayers::none()
                            .with_group(PhysicsLayers::Wind)
                            .with_mask(PhysicsLayers::Movable),
                    );
                sound_channel.play(audio_assets.air.clone());
            } else if player.has_infused(Element::Water) {
                controllable.ability_timer.reset();
                let vel_x = if sprite.flip_x { -100.0 } else { 100.0 };
                let texture_handle = game_assets.droplet.clone();
                let mut projectile_sprite = Sprite::default();
                if sprite.flip_x {
                    projectile_sprite.flip_x = true;
                }
                commands
                    .spawn()
                    .insert_bundle(SpriteBundle {
                        transform: Transform::from_translation(transform.translation()),
                        texture: texture_handle,
                        sprite: projectile_sprite,
                        ..Default::default()
                    })
                    .insert(WaterProjectile)
                    .insert_bundle(PhysicsObjectBundle {
                        collider: CollisionShape::Sphere { radius: 4.0 },
                        rb: RigidBody::Dynamic,
                        rot_constraints: RotationConstraints::lock(),
                        velocity: Velocity::from_linear(Vec3 {
                            x: vel_x,
                            y: 0.0,
                            z: 0.0,
                        }),
                        ..Default::default()
                    })
                    .insert(
                        CollisionLayers::none()
                            .with_group(PhysicsLayers::WaterDrop)
                            .with_masks([PhysicsLayers::Terrain, PhysicsLayers::Lava]),
                    );
                sound_channel.play(audio_assets.pew.clone());
            }
        }
    }
}

pub fn fire_projectile_collision(
    mut commands: Commands,
    fireballs: Query<Entity, (With<FireProjectile>, Without<Block>, Without<Lava>)>,
    flamables: Query<Entity, With<Flamable>>,
    mut goblins: Query<(&mut AnimationState, &mut Velocity, &mut Patrol)>,
    mut collisions: EventReader<CollisionEvent>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    for event in collisions.iter().filter(|e| e.is_started()) {
        let (e1, e2) = event.rigid_body_entities();
        if fireballs.contains(e1) {
            // entity 1 is projectile
            if let Ok((mut state, mut velocity, mut patrol)) = goblins.get_mut(e2) {
                commands.entity(e1).despawn_recursive();
                sound_channel.play(audio_assets.hurt.clone());
                // play goblin death animation
                *state = AnimationState::Death;
                patrol.movement_speed = 0.0;
                velocity.linear.x = 0.0;
                commands
                    .entity(e2)
                    .remove::<Hurtbox>()
                    .remove::<RigidBody>()
                    .insert(DestructionTimer(Timer::from_seconds(0.6, false)));
            } else if flamables.contains(e2) {
                // despawn
                commands.entity(e2).despawn_recursive();
                commands.entity(e1).despawn_recursive();
                sound_channel.play(audio_assets.hurt.clone());
            }
        } else if fireballs.contains(e2) {
            // entity 2 is projectile
            if let Ok((mut state, mut velocity, mut patrol)) = goblins.get_mut(e1) {
                commands.entity(e2).despawn_recursive();
                sound_channel.play(audio_assets.hurt.clone());
                // play goblin death animation
                *state = AnimationState::Death;
                patrol.movement_speed = 0.0;
                velocity.linear.x = 0.0;
                commands
                    .entity(e1)
                    .remove::<Hurtbox>()
                    .remove::<RigidBody>()
                    .insert(DestructionTimer(Timer::from_seconds(0.6, false)));
            } else if flamables.contains(e1) {
                // despawn
                commands.entity(e2).despawn_recursive();
                commands.entity(e1).despawn_recursive();
                sound_channel.play(audio_assets.hurt.clone());
            }
        }
    }
}

pub fn wind_projectile_collision(
    mut projectiles: Query<
        (&Velocity, &mut CollisionLayers),
        (With<WindProjectile>, Without<Block>),
    >,
    mut blocks: Query<&mut Velocity, (With<Block>, Without<FireProjectile>)>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for event in collisions.iter().filter(|e| e.is_started()) {
        let (e1, e2) = event.rigid_body_entities();
        if let Ok((projectile_velocity, mut layers)) = projectiles.get_mut(e1) {
            // entity 1 is projectile
            if let Ok(mut velocity) = blocks.get_mut(e2) {
                // push
                velocity.linear = projectile_velocity.linear;
                *layers = layers.without_mask(PhysicsLayers::Movable);
            }
        } else if let Ok((projectile_velocity, mut layers)) = projectiles.get_mut(e2) {
            // entity 2 is projectile
            if let Ok(mut velocity) = blocks.get_mut(e1) {
                // push
                velocity.linear = projectile_velocity.linear;
                *layers = layers.without_mask(PhysicsLayers::Movable);
            }
        }
    }
}

pub fn water_projectile_collision(
    mut commands: Commands,
    projectiles: Query<Entity, With<WaterProjectile>>,
    mut lava: Query<
        (Entity, &mut Animated, &mut RigidBody, &mut CollisionLayers),
        (With<Lava>, Without<WaterProjectile>),
    >,
    mut collisions: EventReader<CollisionEvent>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    for event in collisions.iter().filter(|e| e.is_started()) {
        let (e1, e2) = event.rigid_body_entities();
        if projectiles.contains(e1) {
            // entity 1 is projectile
            commands.entity(e1).despawn();
            if let Ok((entity, mut animation, mut rb, mut layers)) = lava.get_mut(e2) {
                // turn lava to stone
                animation.start = 8;
                animation.end = 9;
                if *rb != RigidBody::Static {
                    *rb = RigidBody::Static;
                }
                commands.entity(entity).remove::<Lava>();
                if !layers.contains_group(PhysicsLayers::Terrain) {
                    *layers = layers.with_group(PhysicsLayers::Terrain);
                }
                sound_channel.play(audio_assets.steam.clone());
            }
        } else if projectiles.contains(e2) {
            // entity 2 is projectile
            commands.entity(e2).despawn();
            if let Ok((entity, mut animation, mut rb, mut layers)) = lava.get_mut(e1) {
                // turn lava to stone
                animation.start = 8;
                animation.end = 9;
                if *rb != RigidBody::Static {
                    *rb = RigidBody::Static;
                }
                commands.entity(entity).remove::<Lava>();
                if !layers.contains_group(PhysicsLayers::Terrain) {
                    *layers = layers.with_group(PhysicsLayers::Terrain);
                }
                sound_channel.play(audio_assets.steam.clone());
            }
        }
    }
}
