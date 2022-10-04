use std::f32::consts::PI;

use bevy::{
    prelude::{
        Added, Bundle, Component, Entity, EventReader, Query, Transform, Vec3, With, Without,
    },
    sprite::SpriteSheetBundle,
};
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance, LdtkEntity};
use heron::{Acceleration, CollisionEvent};

use crate::{animation::Animated, physics::PhysicsObjectBundle};

use super::player::Player;

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
impl From<Direction> for Vec3 {
    fn from(val: Direction) -> Self {
        match val {
            Direction::North => Vec3::from_slice(&[0.0, 1.0, 0.0]),
            Direction::South => Vec3::from_slice(&[0.0, -1.0, 0.0]),
            Direction::East => Vec3::from_slice(&[1.0, 0.0, 0.0]),
            Direction::West => Vec3::from_slice(&[-1.0, 0.0, 0.0]),
        }
    }
}

#[derive(Component)]
pub struct Fan {
    pub direction: Direction,
}
impl From<EntityInstance> for Fan {
    fn from(entity_instance: EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "Fan" => {
                let field_instances = &entity_instance.field_instances;
                if let Some(field_instance) = field_instances
                    .iter()
                    .find(|f| f.identifier == *"Direction")
                {
                    if let FieldValue::Enum(Some(direction)) = &field_instance.value {
                        let d = match direction.as_str() {
                            "North" => Direction::North,
                            "South" => Direction::South,
                            "East" => Direction::East,
                            "West" => Direction::West,
                            _ => panic!("Fan entity had invalid direction"),
                        };
                        return Fan { direction: d };
                    }
                }
                panic!("Fan entity is missing direction parameter")
            }
            _ => panic!("Entity should not have Fan component"),
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct FanBundle {
    #[from_entity_instance]
    fan: Fan,
    #[bundle]
    #[sprite_sheet_bundle("sprites/fan.png", 16.0, 16.0, 4, 1, 0.0, 0.0, 0)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[bundle]
    #[from_entity_instance]
    pub physics_bundle: PhysicsObjectBundle,
    #[from_entity_instance]
    pub animated: Animated,
}

#[derive(Component)]
pub struct ForceArea {
    pub direction: Direction,
    pub strength: f32,
}
impl From<EntityInstance> for ForceArea {
    fn from(entity_instance: EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "AirCurrent" => {
                let field_instances = &entity_instance.field_instances;
                if let Some(field_instance) = field_instances
                    .iter()
                    .find(|f| f.identifier == *"Direction")
                {
                    if let FieldValue::Enum(Some(direction)) = &field_instance.value {
                        let d = match direction.as_str() {
                            "North" => Direction::North,
                            "South" => Direction::South,
                            "East" => Direction::East,
                            "West" => Direction::West,
                            _ => panic!("Fan entity had invalid direction"),
                        };
                        return ForceArea {
                            direction: d,
                            strength: 1000.0,
                        };
                    }
                }
                panic!("AirCurrent entity is missing direction parameter")
            }
            _ => panic!("Entity should not have ForceArea component"),
        }
    }
}

#[derive(Default, Component)]
pub struct AirCurrent;

#[derive(Bundle, LdtkEntity)]
pub struct AirCurrentBundle {
    air_current: AirCurrent,
    #[from_entity_instance]
    force_area: ForceArea,
    #[bundle]
    #[sprite_sheet_bundle("sprites/wind.png", 16.0, 16.0, 5, 1, 0.0, 0.0, 0)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[bundle]
    #[from_entity_instance]
    pub physics_bundle: PhysicsObjectBundle,
    #[from_entity_instance]
    pub animated: Animated,
}

pub fn apply_force(
    mut player: Query<(Entity, &mut Acceleration), With<Player>>,
    areas: Query<&ForceArea>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for (player_entity, mut player_accelearation) in player.iter_mut() {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(a, b) => {
                    if let Ok(area) = areas.get(a.rigid_body_entity()) {
                        if player_entity == b.rigid_body_entity() {
                            player_accelearation.linear =
                                Vec3::from(area.direction) * area.strength;
                        }
                    } else if let Ok(area) = areas.get(b.rigid_body_entity()) {
                        if player_entity == a.rigid_body_entity() {
                            player_accelearation.linear =
                                Vec3::from(area.direction) * area.strength;
                        }
                    }
                }
                CollisionEvent::Stopped(a, b) => {
                    if areas.contains(a.rigid_body_entity()) {
                        if player_entity == b.rigid_body_entity() {
                            player_accelearation.linear = Vec3::ZERO;
                        }
                    } else if areas.contains(b.rigid_body_entity())
                        && player_entity == a.rigid_body_entity()
                    {
                        player_accelearation.linear = Vec3::ZERO;
                    }
                }
            }
        }
    }
}

pub fn rotate(
    mut fans: Query<(&mut Transform, &Fan), (Added<Fan>, Without<ForceArea>)>,
    mut currents: Query<(&mut Transform, &ForceArea), (Added<AirCurrent>, Without<Fan>)>,
) {
    // Make fan sprites face direction
    for (mut transform, fan) in fans.iter_mut() {
        match fan.direction {
            Direction::North => {}
            Direction::West => transform.rotate_z(PI / 2.0),
            Direction::South => transform.rotate_z(PI),
            Direction::East => transform.rotate_z((3.0 * PI) / 2.0),
        }
    }
    // Make wind currents face direction
    for (mut transform, current) in currents.iter_mut() {
        match current.direction {
            Direction::East => {}
            Direction::North => transform.rotate_z(PI / 2.0),
            Direction::West => transform.rotate_z(PI),
            Direction::South => transform.rotate_z((3.0 * PI) / 2.0),
        }
    }
}
