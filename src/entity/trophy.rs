use bevy::{
    prelude::{Bundle, Children, Component, Entity, EventReader, Query, Visibility, With},
    sprite::SpriteBundle,
};
use bevy_ecs_ldtk::LdtkEntity;
use heron::CollisionEvent;

use crate::physics::PhysicsObjectBundle;

use super::{player::Player, signpost::TextBox, ProximityText};

#[derive(Component, Default)]
pub struct Trophy;

#[derive(Bundle, LdtkEntity)]
pub struct TrophyBundle {
    trophy: Trophy,
    #[bundle]
    #[sprite_bundle("sprites/trophy.png")]
    sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    text: ProximityText,
    #[bundle]
    #[from_entity_instance]
    pub physics_bundle: PhysicsObjectBundle,
}

pub fn check_near(
    trophies: Query<&Children, (With<ProximityText>, With<Trophy>)>,
    player: Query<Entity, With<Player>>,
    mut text: Query<&mut Visibility, With<TextBox>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for player_entity in player.iter() {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(a, b) => {
                    if a.rigid_body_entity() == player_entity {
                        if let Ok(children) = trophies.get(b.rigid_body_entity()) {
                            // show text
                            for child in children.iter() {
                                if let Ok(mut visibility) = text.get_mut(*child) {
                                    visibility.is_visible = true;
                                }
                            }
                        }
                    } else if b.rigid_body_entity() == player_entity {
                        if let Ok(children) = trophies.get(a.rigid_body_entity()) {
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
                        if let Ok(children) = trophies.get(b.rigid_body_entity()) {
                            // hide text
                            for child in children.iter() {
                                if let Ok(mut visibility) = text.get_mut(*child) {
                                    visibility.is_visible = false;
                                }
                            }
                        }
                    } else if b.rigid_body_entity() == player_entity {
                        if let Ok(children) = trophies.get(a.rigid_body_entity()) {
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
