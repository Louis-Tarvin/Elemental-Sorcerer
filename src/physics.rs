use std::time::Duration;

use bevy::{
    prelude::{
        Added, BuildChildren, Bundle, Children, Commands, Component, Entity, EventReader,
        GlobalTransform, Query, Res, SystemLabel, Transform, Vec3,
    },
    sprite::TextureAtlasSprite,
    time::{Time, Timer},
};

use heron::{
    CollisionEvent, CollisionShape, PhysicMaterial, RigidBody, RotationConstraints, Velocity,
};

use crate::{
    debug::DebugSettings,
    entity::{
        ability::Ability,
        player::{AnimationState, Player},
    },
    input::Controllable,
};

#[derive(SystemLabel)]
pub enum PhysicsLabel {
    HandleControllables,
    CheckGrounded,
}

#[derive(Bundle, Default)]
pub struct PhysicsObjectBundle {
    pub collider: CollisionShape,
    pub rb: RigidBody,
    pub velocity: Velocity,
    pub material: PhysicMaterial,
    pub rot_constraints: RotationConstraints,
}

pub fn handle_controllables(
    time: Res<Time>,
    mut query: Query<(
        &mut Velocity,
        &Controllable,
        &Player,
        &mut AnimationState,
        &mut TextureAtlasSprite,
        &Children,
    )>,
    mut ground_detectors: Query<&mut GroundDetector>,
    debug_settings: Res<DebugSettings>,
) {
    for (mut velocity, controllable, player, mut animation, mut texture_atlas, children) in
        query.iter_mut()
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
                    if player.has_equipt(Ability::Jump) {
                        velocity.linear.y = jump_velocity * 1.5;
                    } else {
                        velocity.linear.y = jump_velocity;
                    }
                    // run out the timer
                    detector.coyote_timer.tick(Duration::from_secs(10.0 as u64));
                }

                let acceleration = if detector.is_grounded {
                    acceleration
                } else {
                    acceleration / 2.0
                };

                let max_speed = if player.has_equipt(Ability::Speed) {
                    max_speed * 1.5
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
            }
        }

        // if up && !down {
        // velocity.linear.y = movement_speed;
        // } else if down && !up {
        // velocity.linear.y = -movement_speed;
        // } else if velocity.linear.y != 0.0 {
        // let delta = movement_speed * time.delta_seconds();

        // if velocity.linear.y < 0.0 {
        // velocity.linear.y = (velocity.linear.y + delta).max(0.0)
        // } else {
        // velocity.linear.y = (velocity.linear.y - delta).min(0.0)
        // }
        // }
        if velocity.linear.y > 0.1 {
            if *animation != AnimationState::JumpUp {
                *animation = AnimationState::JumpUp;
            }
        } else if velocity.linear.y < -0.1 {
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

#[derive(Default, Component)]
pub struct GroundDetector {
    pub is_grounded: bool,
    pub coyote_timer: Timer,
}

pub fn add_ground_sensor(mut commands: Commands, query: Query<Entity, Added<Controllable>>) {
    for entity in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn()
                .insert(GroundDetector {
                    is_grounded: false,
                    coyote_timer: Timer::from_seconds(0.1, false),
                })
                .insert(RigidBody::Sensor)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3 {
                        x: 4.0,
                        y: 3.0,
                        z: 1.0,
                    },
                    border_radius: None,
                })
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
                CollisionEvent::Started(a, _b) => {
                    if a.rigid_body_entity() == entity {
                        ground_detector.is_grounded = true;
                        ground_detector.coyote_timer.reset();
                        ground_detector.coyote_timer.pause();
                    }
                }
                CollisionEvent::Stopped(a, _b) => {
                    if a.rigid_body_entity() == entity {
                        ground_detector.is_grounded = false;
                        ground_detector.coyote_timer.unpause();
                    }
                }
            }
        }
    }
}
