use std::fmt::Display;

use bevy::{
    prelude::{
        AssetServer, Assets, Commands, Component, DespawnRecursiveExt, Entity, EventReader,
        GlobalTransform, Query, Res, ResMut, Transform, Vec2, Vec3, With, Without,
    },
    sprite::{Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer},
};
use bevy_inspector_egui::Inspectable;
use heron::{
    CollisionEvent, CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity,
};

use crate::{
    animation::Animated,
    damage::Hurtbox,
    destruction::DestructionTimer,
    entity::{block::Block, goblin::Enemy, lava::Lava, player::Player, Flamable},
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
            } else if player.has_infused(Element::Air) {
                controllable.ability_timer.reset();
                let vel_x = if sprite.flip_x { -50.0 } else { 50.0 };
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
            } else if player.has_infused(Element::Water) {
                controllable.ability_timer.reset();
                let vel_x = if sprite.flip_x { -100.0 } else { 100.0 };
                let texture_handle = asset_server.load("sprites/droplet.png");
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
                            .with_group(PhysicsLayers::Water)
                            .with_masks([PhysicsLayers::Terrain, PhysicsLayers::Lava]),
                    );
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
) {
    for event in collisions.iter().filter(|e| e.is_started()) {
        let (e1, e2) = event.rigid_body_entities();
        if let Ok((projectile, projectile_velocity, mut layers)) = projectiles.get_mut(e1) {
            // entity 1 is projectile
            match projectile {
                Projectile::Fireball => {
                    if flamables.get(e2).is_ok() {
                        // despawn
                        commands.entity(e1).despawn_recursive();
                        commands.entity(e2).despawn_recursive();
                    }
                }
                Projectile::Wind => {
                    if let Ok(mut velocity) = blocks.get_mut(e2) {
                        // push
                        velocity.linear = projectile_velocity.linear * 4.0;
                        *layers = layers.without_mask(PhysicsLayers::Movable);
                    }
                }
                Projectile::Water => {
                    commands.entity(e1).despawn();
                    if let Ok((entity, mut animation, mut rb, mut layers)) = lava.get_mut(e2) {
                        // turn lava to stone
                        animation.start = 8;
                        animation.end = 9;
                        *rb = RigidBody::Static;
                        commands.entity(entity).remove::<Hurtbox>();
                        *layers = layers.with_group(PhysicsLayers::Terrain);
                    }
                }
            }
        } else if let Ok((projectile, projectile_velocity, mut layers)) = projectiles.get_mut(e2) {
            // entity 2 is projectile
            match projectile {
                Projectile::Fireball => {
                    if flamables.get(e1).is_ok() {
                        // despawn
                        commands.entity(e1).despawn_recursive();
                        commands.entity(e2).despawn_recursive();
                    }
                }
                Projectile::Wind => {
                    if let Ok(mut velocity) = blocks.get_mut(e1) {
                        // push
                        velocity.linear = projectile_velocity.linear;
                        *layers = layers.without_mask(PhysicsLayers::Movable);
                    }
                }
                Projectile::Water => {
                    commands.entity(e2).despawn();
                    if let Ok((entity, mut animation, mut rb, mut layers)) = lava.get_mut(e1) {
                        // turn lava to stone
                        animation.start = 8;
                        animation.end = 9;
                        *rb = RigidBody::Static;
                        commands.entity(entity).remove::<Hurtbox>();
                        *layers = layers.with_group(PhysicsLayers::Terrain);
                    }
                }
            }
        }
    }
}
