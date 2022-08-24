use std::fmt::Display;

use bevy::{
    prelude::{
        AssetServer, Assets, Commands, Component, DespawnRecursiveExt, Entity, EventReader,
        GlobalTransform, Query, Res, ResMut, Transform, Vec2, Vec3,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer},
};
use bevy_inspector_egui::Inspectable;
use heron::{
    CollisionEvent, CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity,
};

use crate::{
    animation::Animated,
    destruction::DestructionTimer,
    entity::{goblin::Enemy, player::Player},
    input::Controllable,
    physics::{PhysicsLayers, PhysicsObjectBundle},
};

#[derive(Inspectable, Component, PartialEq, Eq, Clone, Copy)]
pub enum Equiptment {
    Staff,
    MagicBoots,
}
impl Display for Equiptment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Staff => write!(f, "Staff"),
            Self::MagicBoots => write!(f, "Magic Boots"),
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

#[derive(Component, Default)]
pub struct Projectile;

pub fn use_ability(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        &mut Controllable,
        &GlobalTransform,
        &Player,
        &TextureAtlasSprite,
    )>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (mut controllable, transform, player, sprite) in query.iter_mut() {
        controllable.ability_timer.tick(time.delta());
        if controllable.ability
            && controllable.ability_timer.finished()
            && player.has_equipt(Equiptment::Staff)
        {
            if player.has_infused(Element::Fire) {
                controllable.ability_timer.reset();
                let vel_x = if sprite.flip_x { -150.0 } else { 150.0 };
                let texture_handle = asset_server.load("sprites/fireball.png");
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
                    .insert(Animated::new(0.1, 0, 4, false))
                    .insert(Projectile::default())
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
                            .with_mask(PhysicsLayers::Enemy),
                    );
            } else if player.has_infused(Element::Air) {
                controllable.ability_timer.reset();
                let vel_x = if sprite.flip_x { -150.0 } else { 150.0 };
                let texture_handle = asset_server.load("sprites/wind.png");
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
                    .insert(Projectile::default())
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
            }
        }
    }
}

pub fn projectile_collision(
    mut commands: Commands,
    projectiles: Query<&Projectile>,
    enemies: Query<(Entity, &Enemy)>,
    mut collisions: EventReader<CollisionEvent>,
) {
    collisions
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|e| {
            let (e1, e2) = e.rigid_body_entities();
            if (enemies.get(e1).is_ok() && projectiles.get(e2).is_ok())
                || (enemies.get(e2).is_ok() && projectiles.get(e1).is_ok())
            {
                Some((e1, e2))
            } else {
                None
            }
        })
        .for_each(|(e1, e2)| {
            commands.entity(e1).despawn_recursive();
            commands.entity(e2).despawn_recursive();
        });
}
