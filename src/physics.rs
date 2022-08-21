use std::collections::{HashMap, HashSet};

use bevy::{
    prelude::{
        Added, Assets, BuildChildren, Bundle, Children, Commands, Component, Entity, EventReader,
        GlobalTransform, Handle, Parent, Query, Res, Transform, Vec3, Without,
    },
    sprite::TextureAtlasSprite,
    time::Time,
};
use bevy_ecs_ldtk::{prelude::LayerInstance, GridCoords, LdtkIntCell, LdtkLevel};
use heron::{
    CollisionEvent, CollisionShape, PhysicMaterial, RigidBody, RotationConstraints, Velocity,
};

use crate::{
    animation::{AnimaionState, Animated},
    input::Controllable,
};

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
        &mut Animated,
        &mut TextureAtlasSprite,
        &Children,
    )>,
    ground_detectors: Query<&GroundDetector>,
) {
    for (mut velocity, controllable, mut animation, mut texture_atlas, children) in query.iter_mut()
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
            if let Ok(detector) = ground_detectors.get(child) {
                if jumping && detector.is_grounded {
                    velocity.linear.y = jump_velocity;
                }

                let acceleration = if detector.is_grounded {
                    acceleration
                } else {
                    acceleration / 2.0
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
            animation.state = AnimaionState::JumpUp;
        } else if velocity.linear.y < -0.1 {
            animation.state = AnimaionState::JumpDown;
        } else if velocity.linear.x.abs() > 0.05 {
            animation.state = AnimaionState::Walking;
        } else {
            animation.state = AnimaionState::Idle;
        }
    }
}

#[derive(Default, Component)]
pub struct GroundDetector {
    pub is_grounded: bool,
}

pub fn add_ground_sensor(mut commands: Commands, query: Query<Entity, Added<Controllable>>) {
    for entity in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn()
                .insert(GroundDetector::default())
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
    mut detectors: Query<(Entity, &mut GroundDetector)>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for (entity, mut ground_detector) in detectors.iter_mut() {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(a, _b) => {
                    if a.rigid_body_entity() == entity {
                        ground_detector.is_grounded = true;
                    }
                }
                CollisionEvent::Stopped(a, _b) => {
                    if a.rigid_body_entity() == entity {
                        ground_detector.is_grounded = false;
                    }
                }
            }
        }
    }
}

// pub fn position_update(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
// for (velocity, mut pos) in query.iter_mut() {
// pos.translation.x += velocity.0.x * time.delta_seconds();
// pos.translation.y += velocity.0.y * time.delta_seconds();
// }
// }

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

/// Spawns heron collisions for the walls of a level
///
/// You could just insert a ColliderBundle in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The algorithm used here is a nice compromise between simplicity, speed,
/// and a small number of rectangle colliders.
/// In basic terms, it will:
/// 1. consider where the walls are
/// 2. combine wall tiles into flat "plates" in each individual row
/// 3. combine the plates into rectangles across multiple rows wherever possible
/// 4. spawn colliders for each rectangle
pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_insert_with(HashSet::new)
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_handle)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let level = levels
                    .get(level_handle)
                    .expect("Level should be loaded by this point");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level
                    .level
                    .layer_instances
                    .clone()
                    .expect("Level asset should have layers")[3];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right
                    // edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut wall_rects: Vec<Rect> = Vec::new();
                let mut previous_rects: HashMap<Plate, Rect> = HashMap::new();

                // an extra empty row so the algorithm "terminates" the rects that touch the top
                // edge
                plate_stack.push(Vec::new());

                for (y, row) in plate_stack.iter().enumerate() {
                    let mut current_rects: HashMap<Plate, Rect> = HashMap::new();
                    for plate in row {
                        if let Some(previous_rect) = previous_rects.remove(plate) {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    top: previous_rect.top + 1,
                                    ..previous_rect
                                },
                            );
                        } else {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    bottom: y as i32,
                                    top: y as i32,
                                    left: plate.left,
                                    right: plate.right,
                                },
                            );
                        }
                    }

                    // Any plates that weren't removed above have terminated
                    wall_rects.append(&mut previous_rects.values().copied().collect());
                    previous_rects = current_rects;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn()
                            .insert(CollisionShape::Cuboid {
                                half_extends: Vec3::new(
                                    (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                        * grid_size as f32
                                        / 2.,
                                    (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                        * grid_size as f32
                                        / 2.,
                                    0.,
                                ),
                                border_radius: None,
                            })
                            .insert(RigidBody::Static)
                            .insert(PhysicMaterial {
                                friction: 0.0,
                                restitution: 0.0,
                                ..Default::default()
                            })
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}
