use bevy::prelude::{Component, Vec3};
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};
use bevy_inspector_egui::Inspectable;
use heron::{CollisionLayers, CollisionShape, PhysicMaterial, RigidBody, RotationConstraints};

use crate::{
    animation::Animated,
    physics::{PhysicsLayers, PhysicsObjectBundle},
};

pub mod ability;
pub mod checkpoint;
pub mod goblin;
pub mod player;
pub mod signpost;

impl From<EntityInstance> for Animated {
    fn from(entity_instance: EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "Player" => Animated::new(0.1, 0, 1, false),
            "Checkpoint" => Animated::new(0.1, 0, 9, false),
            "Goblin" => Animated::new(0.1, 18, 22, false),
            "Ability" => Animated::new(0.1, 0, 6, false),
            _ => Animated::new(0.1, 0, 1, false),
        }
    }
}

impl From<EntityInstance> for PhysicsObjectBundle {
    fn from(entity_instance: EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "Player" => PhysicsObjectBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3 {
                        x: 4.0,
                        y: 7.0,
                        z: 0.0,
                    },
                    border_radius: Some(1.0),
                },
                rb: RigidBody::Dynamic,
                material: PhysicMaterial {
                    friction: 0.0,
                    density: 1000.0,
                    restitution: 0.0,
                },
                rot_constraints: RotationConstraints::lock(),
                layer: CollisionLayers::none()
                    .with_group(PhysicsLayers::Player)
                    .with_masks(&[PhysicsLayers::Terrain, PhysicsLayers::Enemy]),
                ..Default::default()
            },
            "Signpost" | "Checkpoint" | "Ability" => PhysicsObjectBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::splat(10.0),
                    border_radius: None,
                },
                rb: RigidBody::Sensor,
                ..Default::default()
            },
            "Goblin" => PhysicsObjectBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::splat(8.0),
                    border_radius: None,
                },
                rb: RigidBody::KinematicVelocityBased,
                layer: CollisionLayers::all_masks::<PhysicsLayers>()
                    .with_group(PhysicsLayers::Enemy),
                ..Default::default()
            },
            _ => PhysicsObjectBundle::default(),
        }
    }
}

#[derive(Component, Default, Inspectable)]
pub struct ProximityText(String);

impl From<EntityInstance> for ProximityText {
    fn from(entity_instance: EntityInstance) -> Self {
        let text = match entity_instance.identifier.as_ref() {
            "Signpost" => {
                if let Some(field_instance) = entity_instance
                    .field_instances
                    .iter()
                    .find(|f| f.identifier == *"Text")
                {
                    if let FieldValue::String(Some(text)) = &field_instance.value {
                        text
                    } else {
                        "Error"
                    }
                } else {
                    "Error"
                }
            }
            "Checkpoint" => "Checkpoint saved",
            _ => "Entity should not have ProximityText component",
        };
        ProximityText(text.into())
    }
}
