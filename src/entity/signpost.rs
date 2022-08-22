use bevy::{
    prelude::{
        Added, AssetServer, BuildChildren, Bundle, Children, Color, Commands, Component, Entity,
        EventReader, Query, Res, Transform, Vec3, Visibility, With,
    },
    sprite::SpriteBundle,
    text::{Text, Text2dBundle, TextAlignment},
};
use bevy_ecs_ldtk::LdtkEntity;
use heron::CollisionEvent;

use crate::physics::PhysicsObjectBundle;

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
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    signposts: Query<(Entity, &ProximityText), Added<ProximityText>>,
) {
    for (entity, sign_text) in signposts.iter() {
        commands.entity(entity).with_children(|builder| {
            let style = bevy::text::TextStyle {
                font: asset_server.load("fonts/roboto.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            };
            builder.spawn_bundle(Text2dBundle {
                text: Text::from_section(&sign_text.0, style).with_alignment(TextAlignment::CENTER),
                transform: Transform::from_translation(Vec3::new(0.0, 20.0, 1.0)),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            });
        });
    }
}

pub fn check_near(
    mut signposts: Query<(Entity, &Children), (With<ProximityText>, With<Signpost>)>,
    player: Query<Entity, With<Player>>,
    mut text: Query<&mut Visibility, With<Text>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for player_entity in player.iter() {
        for collision in collisions.iter() {
            for (entity, children) in signposts.iter_mut() {
                match collision {
                    CollisionEvent::Started(a, b) => {
                        if b.rigid_body_entity() == entity && a.rigid_body_entity() == player_entity
                        {
                            // show text
                            for child in children.iter() {
                                if let Ok(mut visibility) = text.get_mut(*child) {
                                    visibility.is_visible = true;
                                }
                            }
                        }
                    }
                    CollisionEvent::Stopped(a, b) => {
                        if b.rigid_body_entity() == entity && a.rigid_body_entity() == player_entity
                        {
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
