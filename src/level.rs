use std::collections::{HashMap, HashSet};

use bevy::prelude::{
    Added, AssetServer, Assets, BuildChildren, Bundle, Commands, Component, Entity, EventReader,
    GlobalTransform, Handle, Image, Input, KeyCode, Parent, Query, Res, ResMut, Transform, Vec2,
    Vec3, With, Without,
};
use bevy_ecs_ldtk::{
    prelude::LayerInstance, GridCoords, LdtkIntCell, LdtkLevel, LevelEvent, LevelSelection, Respawn,
};
use heron::{CollisionLayers, CollisionShape, PhysicMaterial, PhysicsTime, RigidBody};

use crate::{
    damage::Hurtbox,
    entity::player::Player,
    physics::{GroundDetector, PhysicsLayers},
};

/// This function was copied from the example in bevy_ecs_ldtk. All credit goes to the author
pub fn update_level_selection(
    level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (level_handle, level_transform) in level_query.iter() {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level_bounds = bevy::sprite::Rect {
                min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
                max: Vec2::new(
                    level_transform.translation.x + ldtk_level.level.px_wid as f32,
                    level_transform.translation.y + ldtk_level.level.px_hei as f32,
                ),
            };

            for player_transform in player_query.iter() {
                if player_transform.translation.x < level_bounds.max.x
                    && player_transform.translation.x > level_bounds.min.x
                    && player_transform.translation.y < level_bounds.max.y
                    && player_transform.translation.y > level_bounds.min.y
                {
                    *level_selection = LevelSelection::Iid(ldtk_level.level.iid.clone());
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

/// This function was copied from the example in bevy_ecs_ldtk. All credit goes to the author
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
                            .insert(
                                CollisionLayers::all_masks::<PhysicsLayers>()
                                    .with_group(PhysicsLayers::Terrain),
                            )
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}

pub fn restart_level(
    mut commands: Commands,
    level_query: Query<Entity, With<Handle<LdtkLevel>>>,
    mut player_query: Query<(&mut Transform, &Player)>,
    mut level_selection: ResMut<LevelSelection>,
    mut detectors: Query<&mut GroundDetector>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::R) {
        for (mut transform, player) in player_query.iter_mut() {
            transform.translation = player.checkpoint;
            transform.translation.z = 7.0;
            *level_selection = player.checkpoint_level.clone();
        }
        for level_entity in level_query.iter() {
            commands.entity(level_entity).insert(Respawn);
        }
        for mut detector in detectors.iter_mut() {
            detector.active_collisions = 0;
        }
    }
}

/// Prevents entity sprites from dissapearing upon reload
/// See: https://github.com/Trouv/bevy_ecs_ldtk/issues/111
pub fn prevent_asset_unloading(mut commands: Commands, asset_server: Res<AssetServer>) {
    #[derive(Component)]
    struct LdtkImageHolder(Handle<Image>);
    for asset in ["block", "signpost", "wood_block", "trophy"].iter() {
        commands.spawn().insert(LdtkImageHolder(
            asset_server.load(&format!("sprites/{}.png", asset)),
        ));
    }
}

#[derive(Component, Default)]
pub struct Spike;

#[derive(Bundle, LdtkIntCell)]
pub struct SpikeBundle {
    pub hurtbox: Hurtbox,
    pub spike: Spike,
}

pub fn spawn_spike_collision(
    mut commands: Commands,
    spike_query: Query<(&GridCoords, &Parent), Added<Spike>>,
    parent_query: Query<&Parent, Without<Spike>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn spike collisions
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    spike_query.for_each(|(&grid_coords, parent)| {
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

    if !spike_query.is_empty() {
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

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for (y, row) in plate_stack.iter().enumerate() {
                        for plate in row {
                            level
                                .spawn()
                                .insert(CollisionShape::Cuboid {
                                    half_extends: Vec3::new(
                                        (plate.right as f32 - plate.left as f32 + 1.)
                                            * grid_size as f32
                                            / 2.,
                                        grid_size as f32 / 2. - 5.0,
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
                                    (plate.left + plate.right + 1) as f32 * grid_size as f32 / 2.,
                                    (y + y + 1) as f32 * grid_size as f32 / 2.,
                                    0.,
                                ))
                                .insert(
                                    CollisionLayers::none()
                                        .with_group(PhysicsLayers::Spikes)
                                        .with_mask(PhysicsLayers::PlayerBody),
                                )
                                .insert(GlobalTransform::default())
                                .insert(Hurtbox);
                        }
                    }
                });
            }
        });
    }
}

/// This function was copied from the example in bevy_ecs_ldtk. All credit goes to the author
pub fn pause_physics_during_load(
    mut level_events: EventReader<LevelEvent>,
    mut physics_time: ResMut<PhysicsTime>,
) {
    for event in level_events.iter() {
        match event {
            LevelEvent::SpawnTriggered(_) => physics_time.set_scale(0.),
            LevelEvent::Transformed(_) => physics_time.set_scale(1.),
            _ => (),
        }
    }
}
