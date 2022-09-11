use bevy::{
    prelude::{
        shape::Quad, Added, Assets, BuildChildren, Bundle, Children, Color, Commands, Component,
        Entity, EventReader, Mesh, Query, Res, ResMut, Transform, Vec2, Vec3, Visibility, With,
    },
    sprite::{ColorMaterial, ColorMesh2dBundle, SpriteBundle},
    text::{Text, Text2dBundle, TextAlignment},
};
use bevy_ecs_ldtk::LdtkEntity;
use heron::CollisionEvent;

use crate::{physics::PhysicsObjectBundle, state::load_game::GameAssets};

use super::{player::Player, ProximityText};

#[derive(Component, Default)]
pub struct TextBox;
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
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    signposts: Query<(Entity, &ProximityText), Added<ProximityText>>,
) {
    for (entity, sign_text) in signposts.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn_bundle(ColorMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(Quad::new(Vec2::new(sign_text.width, 30.0))))
                        .into(),
                    material: materials.add(ColorMaterial::from(sign_text.color)),
                    transform: Transform::from_translation(Vec3 {
                        x: 0.0,
                        y: 40.0,
                        z: 10.0,
                    }),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(TextBox)
                .with_children(|parent| {
                    let style = bevy::text::TextStyle {
                        font: game_assets.pixel_font.clone(),
                        font_size: 15.0,
                        color: Color::WHITE,
                    };
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::from_section(&sign_text.text, style)
                            .with_alignment(TextAlignment::TOP_CENTER),
                        transform: Transform {
                            translation: Vec3 {
                                x: 0.0,
                                y: 10.0,
                                z: 10.0,
                            },
                            scale: Vec3::splat(0.4),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
    }
}

pub fn check_near(
    signposts: Query<&Children, (With<ProximityText>, With<Signpost>)>,
    player: Query<Entity, With<Player>>,
    mut text: Query<&mut Visibility, With<TextBox>>,
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
