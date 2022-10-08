use std::f32::consts::PI;

use bevy::{
    prelude::{warn, Added, Bundle, Component, EventReader, Query, Transform, Vec3, Without},
    sprite::SpriteSheetBundle,
};
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance, LdtkEntity};
use heron::{Acceleration, CollisionEvent};

use crate::{
    animation::Animated,
    physics::{Direction, Dynamic, PhysicsObjectBundle},
};

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

pub fn check_collision(
    mut movables: Query<&mut Dynamic>,
    areas: Query<&ForceArea>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.iter() {
        match collision {
            CollisionEvent::Started(a, b) => {
                if let Ok(area) = areas.get(a.rigid_body_entity()) {
                    if let Ok(mut movable) = movables.get_mut(b.rigid_body_entity()) {
                        movable.counter += 1;
                        movable.direction = area.direction;
                    }
                } else if let Ok(area) = areas.get(b.rigid_body_entity()) {
                    if let Ok(mut movable) = movables.get_mut(a.rigid_body_entity()) {
                        movable.counter += 1;
                        movable.direction = area.direction;
                    }
                }
            }
            CollisionEvent::Stopped(a, b) => {
                if areas.contains(a.rigid_body_entity()) {
                    if let Ok(mut movable) = movables.get_mut(b.rigid_body_entity()) {
                        if movable.counter > 0 {
                            movable.counter -= 1;
                        } else {
                            warn!("Dynamic entity attempted to decrement collision counter that was already 0");
                        }
                    }
                } else if areas.contains(b.rigid_body_entity()) {
                    if let Ok(mut movable) = movables.get_mut(a.rigid_body_entity()) {
                        if movable.counter > 0 {
                            movable.counter -= 1;
                        } else {
                            warn!("Dynamic entity attempted to decrement collision counter that was already 0");
                        }
                    }
                }
            }
        }
    }
}

pub fn apply_force(mut movables: Query<(&mut Acceleration, &Dynamic)>) {
    for (mut acceleration, movable) in movables.iter_mut() {
        if movable.counter > 0 {
            acceleration.linear = (Vec3::from(movable.direction)) * 700.0;
        } else {
            acceleration.linear = Vec3::ZERO;
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
