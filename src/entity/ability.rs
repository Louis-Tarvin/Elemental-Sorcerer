use std::fmt::Display;

use bevy::{
    prelude::{
        Added, AssetServer, Assets, Bundle, Commands, Component, Entity, EventReader, Handle,
        Image, Query, Res,
    },
    sprite::{SpriteSheetBundle, TextureAtlas},
};
use bevy_ecs_ldtk::{
    prelude::{FieldValue, LayerInstance, LdtkEntity, TilesetDefinition},
    EntityInstance,
};
use bevy_inspector_egui::Inspectable;
use bevy_kira_audio::{Audio, AudioControl};
use heron::CollisionEvent;

use crate::{animation::Animated, audio::AudioAssets, physics::PhysicsObjectBundle};

use super::player::Player;

#[derive(Inspectable, Component, PartialEq, Eq, Clone, Copy)]
pub enum Ability {
    Fire,
    Air,
    Water,
    MagicBoots,
    Cloak,
}

impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ability::Fire => write!(f, "Fire"),
            Ability::Air => write!(f, "Air"),
            Ability::Water => write!(f, "Water"),
            Ability::MagicBoots => write!(f, "Magic Boots"),
            Ability::Cloak => write!(f, "Cloak of Resistance"),
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
                    "Fire" => Ability::Fire,
                    "Air" => Ability::Air,
                    "Water" => Ability::Water,
                    "Boots" => Ability::MagicBoots,
                    "Cloak" => Ability::Cloak,
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
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for (player_entity, mut player) in player.iter_mut() {
        for collision in collisions.iter() {
            if let CollisionEvent::Started(a, b) = collision {
                if a.rigid_body_entity() == player_entity {
                    if let Ok(ability) = ability_orbs.get(b.rigid_body_entity()) {
                        match ability {
                            Ability::Fire => player.unlocked_fire = true,
                            Ability::Air => player.unlocked_air = true,
                            Ability::MagicBoots => player.unlocked_boots = true,
                            Ability::Water => player.unlocked_water = true,
                            Ability::Cloak => player.unlocked_cloak = true,
                        }
                        commands.entity(b.rigid_body_entity()).despawn();
                        audio.play(audio_assets.collect.clone());
                    }
                } else if b.rigid_body_entity() == player_entity {
                    if let Ok(ability) = ability_orbs.get(a.rigid_body_entity()) {
                        match ability {
                            Ability::Fire => player.unlocked_fire = true,
                            Ability::Air => player.unlocked_air = true,
                            Ability::MagicBoots => player.unlocked_boots = true,
                            Ability::Water => player.unlocked_water = true,
                            Ability::Cloak => player.unlocked_cloak = true,
                        }
                        commands.entity(a.rigid_body_entity()).despawn();
                        audio.play(audio_assets.collect.clone());
                    }
                }
            }
        }
    }
}

pub fn dont_spawn_if_collected(
    mut commands: Commands,
    query: Query<(Entity, &Ability), Added<Ability>>,
    player: Query<&Player>,
) {
    for player in player.iter() {
        for (entity, ability) in query.iter() {
            match ability {
                Ability::Fire => {
                    if player.unlocked_fire {
                        commands.entity(entity).despawn();
                    }
                }
                Ability::Air => {
                    if player.unlocked_air {
                        commands.entity(entity).despawn();
                    }
                }
                Ability::Water => {
                    if player.unlocked_water {
                        commands.entity(entity).despawn();
                    }
                }
                Ability::MagicBoots => {
                    if player.unlocked_boots {
                        commands.entity(entity).despawn();
                    }
                }
                Ability::Cloak => {
                    if player.unlocked_cloak {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}
