use bevy::{
    prelude::{
        Added, BuildChildren, Bundle, Children, Color, Commands, Component, Entity, EventReader,
        Query, Res, Transform, Vec3, Visibility, With,
    },
    sprite::SpriteBundle,
    text::{Text, Text2dBundle, TextAlignment},
};
use bevy_ecs_ldtk::LdtkEntity;
use heron::CollisionEvent;

use crate::{physics::PhysicsObjectBundle, state::loading::GameAssets};

use super::{player::Player, ProximityText};

#[derive(Component, Default)]
pub struct Signpost;

#[derive(Bundle, LdtkEntity)]
pub struct SignpostBundle {
    signpost: Signpost,
    #[bundle]
    #[sprite_bundle("sprites/signpost.png")]
    sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    text: ProximityText,
    #[bundle]
    #[from_entity_instance]
    pub physics_bundle: PhysicsObjectBundle,
}

pub fn spawn_text(
    game_assets: Res<GameAssets>,
    mut commands: Commands,
    signposts: Query<(Entity, &ProximityText), Added<ProximityText>>,
) {
    for (entity, sign_text) in signposts.iter() {
        commands.entity(entity).with_children(|builder| {
            let style = bevy::text::TextStyle {
                font: game_assets.pixel_font.clone(),
                font_size: 15.0,
                color: Color::WHITE,
            };
            builder.spawn_bundle(Text2dBundle {
                text: Text::from_section(&sign_text.0, style)
                    .with_alignment(TextAlignment::TOP_CENTER),
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: 40.0,
                        z: 10.0,
                    },
                    scale: Vec3::splat(0.4),
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                ..Default::default()
            });
        });
    }
}

pub fn check_near(
    signposts: Query<&Children, (With<ProximityText>, With<Signpost>)>,
    player: Query<Entity, With<Player>>,
    mut text: Query<&mut Visibility, With<Text>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for player_entity in player.iter() {
        for collision in collisions.iter() {
            // for (entity, children) in signposts.iter_mut() {
            match collision {
                CollisionEvent::Started(a, b) => {
                    if a.rigid_body_entity() == player_entity {
                        if let Ok(children) = signposts.get(b.rigid_body_entity()) {
                            // show text
                            for child in children.iter() {
                                if let Ok(mut visibility) = text.get_mut(*child) {
                                    visibility.is_visible = true;
                                }
                            }
                        }
                    } else if b.rigid_body_entity() == player_entity {
                        if let Ok(children) = signposts.get(a.rigid_body_entity()) {
                            // show text
                            for child in children.iter() {
                                if let Ok(mut visibility) = text.get_mut(*child) {
                                    visibility.is_visible = true;
                                }
                            }
                        }
                    }
                }
                CollisionEvent::Stopped(a, b) => {
                    if a.rigid_body_entity() == player_entity {
                        if let Ok(children) = signposts.get(b.rigid_body_entity()) {
                            // hide text
                            for child in children.iter() {
                                if let Ok(mut visibility) = text.get_mut(*child) {
                                    visibility.is_visible = false;
                                }
                            }
                        }
                    } else if b.rigid_body_entity() == player_entity {
                        if let Ok(children) = signposts.get(a.rigid_body_entity()) {
                            // hide text
                            for child in children.iter() {
                                if let Ok(mut visibility) = text.get_mut(*child) {
                                    visibility.is_visible = false;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
