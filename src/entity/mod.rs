use bevy::prelude::{Color, Component, Vec3};
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};
use bevy_inspector_egui::Inspectable;
use heron::{CollisionLayers, CollisionShape, PhysicMaterial, RigidBody, RotationConstraints};

use crate::{
    animation::Animated,
    physics::{PhysicsLayers, PhysicsObjectBundle},
};

pub mod ability;
pub mod block;
pub mod checkpoint;
pub mod fan;
pub mod goblin;
pub mod lava;
pub mod player;
pub mod signpost;
pub mod torch;
pub mod trophy;
pub mod water;

#[derive(Component, Default)]
pub struct Flamable;

impl From<EntityInstance> for Animated {
    fn from(entity_instance: EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "Player" => Animated::new(0.1, 0, 1, false),
            "Checkpoint" => Animated::new(0.1, 0, 9, false),
            "Goblin" => Animated::new(0.1, 18, 22, false),
            "Ability" => Animated::new(0.1, 0, 6, false),
            "Lava" => Animated::new(0.3, 0, 8, false),
            "Water" => Animated::new(0.15, 0, 8, false),
            "Fan" => Animated::new(0.1, 0, 4, false),
            "AirCurrent" => Animated::new(0.1, 0, 5, false),
            "Torch" => Animated::new(0.1, 0, 12, false),
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
                    density: 100.0,
                    restitution: 0.0,
                },
                rot_constraints: RotationConstraints::lock(),
                layer: CollisionLayers::none()
                    .with_group(PhysicsLayers::PlayerBody)
                    .with_masks(&[
                        PhysicsLayers::Terrain,
                        PhysicsLayers::Enemy,
                        PhysicsLayers::Movable,
                        PhysicsLayers::Interactable,
                        PhysicsLayers::Lava,
                        PhysicsLayers::Water,
                        PhysicsLayers::Spikes,
                    ]),
                ..Default::default()
            },
            "Signpost" | "Checkpoint" | "Ability" | "Trophy" => PhysicsObjectBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::splat(10.0),
                    border_radius: None,
                },
                rb: RigidBody::Sensor,
                layer: CollisionLayers::all_masks::<PhysicsLayers>()
                    .with_group(PhysicsLayers::Interactable),
                ..Default::default()
            },
            "Goblin" => PhysicsObjectBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3 {
                        x: 6.0,
                        y: 7.0,
                        z: 1.0,
                    },
                    border_radius: None,
                },
                rb: RigidBody::KinematicVelocityBased,
                layer: CollisionLayers::all_masks::<PhysicsLayers>()
                    .without_mask(PhysicsLayers::PlayerGroundDetector)
                    .with_group(PhysicsLayers::Enemy),
                ..Default::default()
            },
            "Block" => PhysicsObjectBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::splat(8.0),
                    border_radius: None,
                },
                material: PhysicMaterial {
                    friction: 0.3,
                    density: 1000.0,
                    restitution: 0.0,
                },
                rb: RigidBody::Dynamic,
                layer: CollisionLayers::all_masks::<PhysicsLayers>()
                    .with_group(PhysicsLayers::Movable),
                ..Default::default()
            },
            "WoodBlock" => PhysicsObjectBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::splat(8.0),
                    border_radius: None,
                },
                material: PhysicMaterial {
                    friction: 0.0,
                    ..Default::default()
                },
                rb: RigidBody::Static,
                layer: CollisionLayers::all_masks::<PhysicsLayers>()
                    .with_groups([PhysicsLayers::Terrain, PhysicsLayers::Wood]),
                ..Default::default()
            },
            "Fan" => PhysicsObjectBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3 {
                        x: 8.0,
                        y: 5.0,
                        z: 1.0,
                    },
                    border_radius: None,
                },
                material: PhysicMaterial {
                    friction: 0.0,
                    ..Default::default()
                },
                rb: RigidBody::Static,
                layer: CollisionLayers::all_masks::<PhysicsLayers>()
                    .with_groups([PhysicsLayers::Terrain]),
                ..Default::default()
            },
            "AirCurrent" => PhysicsObjectBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::splat(8.0),
                    border_radius: None,
                },
                rb: RigidBody::Sensor,
                layer: CollisionLayers::all_masks::<PhysicsLayers>()
                    .with_group(PhysicsLayers::Interactable),
                ..Default::default()
            },
            "Lava" => PhysicsObjectBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3 {
                        x: 8.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    border_radius: None,
                },
                rb: RigidBody::Sensor,
                layer: CollisionLayers::all_masks::<PhysicsLayers>()
                    .with_group(PhysicsLayers::Lava),
                ..Default::default()
            },
            "Water" => PhysicsObjectBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3 {
                        x: 8.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    border_radius: None,
                },
                rb: RigidBody::Sensor,
                layer: CollisionLayers::all_masks::<PhysicsLayers>()
                    .with_group(PhysicsLayers::Water),
                ..Default::default()
            },
            _ => PhysicsObjectBundle::default(),
        }
    }
}

#[derive(Component, Default, Inspectable)]
pub struct ProximityText {
    pub text: String,
    pub width: f32,
    pub color: Color,
}

impl From<EntityInstance> for ProximityText {
    fn from(entity_instance: EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "Signpost" => {
                let mut width = 250.0;
                let field_instances = &entity_instance.field_instances;
                if let Some(field_instance) =
                    field_instances.iter().find(|f| f.identifier == *"Text")
                {
                    if let FieldValue::String(Some(text)) = &field_instance.value {
                        if let Some(field_instance) =
                            field_instances.iter().find(|f| f.identifier == *"Width")
                        {
                            if let FieldValue::Float(Some(val)) = &field_instance.value {
                                width = *val;
                            }
                        }
                        ProximityText {
                            text: text.into(),
                            width,
                            color: Color::rgb(0.58, 0.345, 0.282),
                        }
                    } else {
                        ProximityText {
                            text: "Error".into(),
                            width,
                            color: Color::RED,
                        }
                    }
                } else {
                    ProximityText {
                        text: "Error".into(),
                        width,
                        color: Color::RED,
                    }
                }
            }
            "Checkpoint" => ProximityText {
                text: "Checkpoint saved.\nPress <down> to interact".into(),
                width: 165.0,
                color: Color::GRAY,
            },
            "Trophy" => ProximityText {
                text: "You Win!\nThanks for playing.".into(),
                width: 150.0,
                color: Color::rgb(0.839, 0.604, 0.306),
            },
            _ => panic!("Entity should not have ProximityText component"),
        }
    }
}
