use std::fmt::Display;

use bevy::{
    prelude::{
        Assets, Commands, Component, DespawnRecursiveExt, Entity, EventReader, GlobalTransform,
        Mut, Query, Res, ResMut, Transform, Vec2, Vec3, With, Without,
    },
    sprite::{Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer},
};
use bevy_inspector_egui::Inspectable;
use bevy_kira_audio::{Audio, AudioControl};
use heron::{
    CollisionEvent, CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity,
};

use crate::{
    animation::Animated,
    audio::AudioAssets,
    destruction::DestructionTimer,
    entity::{block::Block, lava::Lava, player::Player, Flamable},
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
pub enum Projectile {
    Fireball,
    Wind,
    Water,
}

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
    audio: Res<Audio>,
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
                    .insert(Projectile::Fireball)
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
                audio.play(audio_assets.fireball.clone());
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
                    .insert(Projectile::Wind)
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
                audio.play(audio_assets.air.clone());
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
                    .insert(Projectile::Water)
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
                audio.play(audio_assets.pew.clone());
            }
        }
    }
}

pub fn projectile_collision(
    mut commands: Commands,
    mut projectiles: Query<
        (&Projectile, &Velocity, &mut CollisionLayers),
        (Without<Block>, Without<Lava>),
    >,
    flamables: Query<(Entity, &Flamable)>,
    mut blocks: Query<&mut Velocity, (With<Block>, Without<Projectile>)>,
    mut lava: Query<
        (Entity, &mut Animated, &mut RigidBody, &mut CollisionLayers),
        (With<Lava>, Without<Projectile>),
    >,
    mut collisions: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    audio_manager: Res<AudioAssets>,
) {
    // Should probably split this into multiple systems

    for event in collisions.iter().filter(|e| e.is_started()) {
        let (e1, e2) = event.rigid_body_entities();
        if let Ok((projectile, projectile_velocity, layers)) = projectiles.get_mut(e1) {
            // entity 1 is projectile
            resolve_projectile_collision(
                &mut commands,
                projectile,
                projectile_velocity,
                layers,
                e1,
                e2,
                &flamables,
                &mut blocks,
                &mut lava,
                &audio,
                &audio_manager,
            );
        } else if let Ok((projectile, projectile_velocity, layers)) = projectiles.get_mut(e2) {
            // entity 2 is projectile
            resolve_projectile_collision(
                &mut commands,
                projectile,
                projectile_velocity,
                layers,
                e2,
                e1,
                &flamables,
                &mut blocks,
                &mut lava,
                &audio,
                &audio_manager,
            );
        }
    }
}

fn resolve_projectile_collision(
    commands: &mut Commands,
    projectile: &Projectile,
    projectile_velocity: &Velocity,
    mut layers: Mut<CollisionLayers>,
    projectile_entity: Entity,
    other: Entity,
    flamables: &Query<(Entity, &Flamable)>,
    blocks: &mut Query<&mut Velocity, (With<Block>, Without<Projectile>)>,
    lava: &mut Query<
        (Entity, &mut Animated, &mut RigidBody, &mut CollisionLayers),
        (With<Lava>, Without<Projectile>),
    >,
    audio: &Res<Audio>,
    audio_manager: &Res<AudioAssets>,
) {
    match projectile {
        Projectile::Fireball => {
            if flamables.get(other).is_ok() {
                // despawn
                commands.entity(other).despawn_recursive();
                commands.entity(projectile_entity).despawn_recursive();
                audio.play(audio_manager.hurt.clone());
            }
        }
        Projectile::Wind => {
            if let Ok(mut velocity) = blocks.get_mut(other) {
                // push
                velocity.linear = projectile_velocity.linear;
                *layers = layers.without_mask(PhysicsLayers::Movable);
            }
        }
        Projectile::Water => {
            commands.entity(projectile_entity).despawn();
            if let Ok((entity, mut animation, mut rb, mut layers)) = lava.get_mut(other) {
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
                audio.play(audio_manager.steam.clone());
            }
        }
    }
}
