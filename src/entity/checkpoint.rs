use bevy::{
    prelude::{
        Bundle, Children, Component, Entity, EventReader, GlobalTransform, Query, Res, Visibility,
        With,
    },
    sprite::SpriteSheetBundle,
    text::Text,
};
use bevy_ecs_ldtk::{LdtkEntity, LevelSelection};
use bevy_kira_audio::{Audio, AudioControl};
use heron::CollisionEvent;

use crate::{animation::Animated, audio::AudioAssets, physics::PhysicsObjectBundle};

use super::{player::Player, ProximityText};

#[derive(Component, Default)]
pub struct Checkpoint;

#[derive(Bundle, LdtkEntity)]
pub struct CheckpointBundle {
    checkpoint: Checkpoint,
    #[bundle]
    #[sprite_sheet_bundle("sprites/checkpoint.png", 12.0, 20.0, 9, 1, 0.0, 0.0, 0)]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    animated: Animated,
    #[from_entity_instance]
    text: ProximityText,
    #[bundle]
    #[from_entity_instance]
    pub physics_bundle: PhysicsObjectBundle,
}

pub fn check_near(
    checkpoints: Query<(&GlobalTransform, &Children), (With<ProximityText>, With<Checkpoint>)>,
    mut player: Query<(Entity, &mut Player)>,
    mut text: Query<&mut Visibility, With<Text>>,
    mut collisions: EventReader<CollisionEvent>,
    level_selection: Res<LevelSelection>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for (player_entity, mut player) in player.iter_mut() {
        for collision in collisions.iter() {
            // for (entity, children) in signposts.iter_mut() {
            match collision {
                CollisionEvent::Started(a, b) => {
                    if a.rigid_body_entity() == player_entity {
                        if let Ok((transform, children)) = checkpoints.get(b.rigid_body_entity()) {
                            // show text
                            for child in children.iter() {
                                if let Ok(mut visibility) = text.get_mut(*child) {
                                    visibility.is_visible = true;
                                }
                            }
                            // set checkpoint
                            player.checkpoint = transform.translation();
                            player.checkpoint_level = level_selection.clone();
                            player.near_checkpoint = true;
                            audio.play(audio_assets.ping.clone());
                        }
                    } else if b.rigid_body_entity() == player_entity {
                        if let Ok((transform, children)) = checkpoints.get(a.rigid_body_entity()) {
                            // show text
                            for child in children.iter() {
                                if let Ok(mut visibility) = text.get_mut(*child) {
                                    visibility.is_visible = true;
                                }
                            }
                            // set checkpoint
                            player.checkpoint = transform.translation();
                            player.checkpoint_level = level_selection.clone();
                            player.near_checkpoint = true;
                            audio.play(audio_assets.ping.clone());
                        }
                    }
                }
                CollisionEvent::Stopped(a, b) => {
                    if a.rigid_body_entity() == player_entity {
                        if let Ok((_, children)) = checkpoints.get(b.rigid_body_entity()) {
                            // hide text
                            for child in children.iter() {
                                if let Ok(mut visibility) = text.get_mut(*child) {
                                    visibility.is_visible = false;
                                }
                            }
                            player.near_checkpoint = false;
                        }
                    } else if b.rigid_body_entity() == player_entity {
                        if let Ok((_, children)) = checkpoints.get(a.rigid_body_entity()) {
                            // hide text
                            for child in children.iter() {
                                if let Ok(mut visibility) = text.get_mut(*child) {
                                    visibility.is_visible = false;
                                }
                            }
                            player.near_checkpoint = false;
                        }
                    }
                }
            }
        }
    }
}
