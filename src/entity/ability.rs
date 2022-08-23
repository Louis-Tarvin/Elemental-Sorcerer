use std::fmt::Display;

use bevy::{
    prelude::{
        AssetServer, Assets, Bundle, Commands, Component, Entity, EventReader, Handle, Image, Query,
    },
    sprite::{SpriteSheetBundle, TextureAtlas},
};
use bevy_ecs_ldtk::{
    prelude::{FieldValue, LayerInstance, LdtkEntity, TilesetDefinition},
    EntityInstance,
};
use bevy_inspector_egui::Inspectable;
use heron::CollisionEvent;

use crate::{animation::Animated, physics::PhysicsObjectBundle};

use super::player::Player;

#[derive(Inspectable, Component, PartialEq, Eq, Clone, Copy)]
pub enum Ability {
    Fireball,
    Jump,
    Airblast,
    Speed,
}

impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ability::Fireball => write!(f, "Fireball"),
            Ability::Jump => write!(f, "Improved Jump"),
            Ability::Airblast => write!(f, "Air Blast"),
            Ability::Speed => write!(f, "Increased Speed"),
        }
    }
}

impl LdtkEntity for Ability {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Ability {
        if let Some(ldtk_ability) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"Ability")
        {
            if let FieldValue::Enum(Some(ability)) = &ldtk_ability.value {
                match ability.as_str() {
                    "Fireball" => Ability::Fireball,
                    "Jump" => Ability::Jump,
                    "Airblast" => Ability::Airblast,
                    "Speed" => Ability::Speed,
                    _ => panic!("Unknown ability enum variant: {}", ability),
                }
            } else {
                panic!("Ability entity has no assigned ability");
            }
        } else {
            panic!("Ability entity has no assigned ability");
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct AbilityBundle {
    #[ldtk_entity]
    ability: Ability,
    #[bundle]
    #[sprite_sheet_bundle("sprites/orb.png", 8.0, 8.0, 6, 1, 0.0, 0.0, 0)]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    animated: Animated,
    #[bundle]
    #[from_entity_instance]
    pub physics_bundle: PhysicsObjectBundle,
}

pub fn check_near(
    mut commands: Commands,
    ability_orbs: Query<&Ability>,
    mut player: Query<(Entity, &mut Player)>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for (player_entity, mut player) in player.iter_mut() {
        for collision in collisions.iter() {
            // for (entity, children) in signposts.iter_mut() {
            if let CollisionEvent::Started(a, b) = collision {
                if a.rigid_body_entity() == player_entity {
                    if let Ok(ability) = ability_orbs.get(b.rigid_body_entity()) {
                        match ability {
                            Ability::Fireball => player.unlocked_fireball = true,
                            Ability::Jump => player.unlocked_jump = true,
                            Ability::Speed => player.unlocked_speed = true,
                            Ability::Airblast => player.unlocked_airblast = true,
                        }
                        commands.entity(b.rigid_body_entity()).despawn();
                    }
                } else if b.rigid_body_entity() == player_entity {
                    if let Ok(ability) = ability_orbs.get(a.rigid_body_entity()) {
                        match ability {
                            Ability::Fireball => player.unlocked_fireball = true,
                            Ability::Jump => player.unlocked_jump = true,
                            Ability::Speed => player.unlocked_speed = true,
                            Ability::Airblast => player.unlocked_airblast = true,
                        }
                        commands.entity(a.rigid_body_entity()).despawn();
                    }
                }
            }
        }
    }
}
