use bevy::{
    prelude::{
        AssetServer, BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component,
        DespawnRecursiveExt, Entity, ImageBundle, Input, KeyCode, MouseButton, NodeBundle, Query,
        Res, ResMut, TextBundle, With, Without,
    },
    text::{Text, TextStyle},
    ui::{
        AlignItems, FlexDirection, Interaction, JustifyContent, Size, Style, UiColor, UiRect, Val,
    },
};
use heron::PhysicsTime;

use crate::{
    entity::{ability::Ability, player::Player},
    input::Controllable,
};

use super::State;

#[derive(Component)]
pub struct UiRootNode;
#[derive(Component)]
pub struct Slot1Text;
#[derive(Component)]
pub struct Slot2Text;

pub fn trigger_enter(
    mut app_state: ResMut<bevy::prelude::State<State>>,
    query: Query<(&Controllable, &Player), Changed<Controllable>>,
    mut physics_time: ResMut<PhysicsTime>,
) {
    for (controllable, player) in query.iter() {
        if controllable.interacting && player.near_checkpoint {
            physics_time.set_scale(0.0);
            app_state.push(State::AbilityMenu).unwrap();
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Main box
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(80.0), Val::Px(500.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Title text wrapper
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Title text
                            parent.spawn_bundle(TextBundle::from_section(
                                "Select abilities:",
                                TextStyle {
                                    font: asset_server.load("fonts/roboto.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // Buttons wrapper
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Auto),
                                ..Default::default()
                            },
                            color: Color::rgba(0.2, 0.0, 0.0, 0.8).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Fireball button
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                        // center button
                                        margin: UiRect::all(Val::Auto),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: Color::rgb(0.15, 0.15, 0.15).into(),
                                    ..Default::default()
                                })
                                .insert(Ability::Fireball)
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle::from_section(
                                        "Fireball",
                                        TextStyle {
                                            font: asset_server.load("fonts/roboto.ttf"),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });

                            // Jump button
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                                        // center button
                                        margin: UiRect::all(Val::Auto),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: Color::rgb(0.15, 0.15, 0.15).into(),
                                    ..Default::default()
                                })
                                .insert(Ability::Jump)
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle::from_section(
                                        "Improved Jump",
                                        TextStyle {
                                            font: asset_server.load("fonts/roboto.ttf"),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });

                            // Speed button
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                        // center button
                                        margin: UiRect::all(Val::Auto),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: Color::rgb(0.15, 0.15, 0.15).into(),
                                    ..Default::default()
                                })
                                .insert(Ability::Speed)
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle::from_section(
                                        "Speed",
                                        TextStyle {
                                            font: asset_server.load("fonts/roboto.ttf"),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });
                        });

                    // Equipt wrapper
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Slot 1 text
                            parent
                                .spawn_bundle(TextBundle::from_section(
                                    "Slot 1: None",
                                    TextStyle {
                                        font: asset_server.load("fonts/roboto.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                ))
                                .insert(Slot1Text);
                            // Slot 2 text
                            parent
                                .spawn_bundle(TextBundle::from_section(
                                    "Slot 2: None",
                                    TextStyle {
                                        font: asset_server.load("fonts/roboto.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                ))
                                .insert(Slot2Text);
                        });
                });
        })
        .insert(UiRootNode);
}

pub fn button_system(
    mouse_input: Res<Input<MouseButton>>,
    mut interaction_query: Query<(&Interaction, &Ability), With<Button>>,
    mut player_query: Query<&mut Player>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (interaction, ability) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked | Interaction::Hovered => {
                    for mut player in player_query.iter_mut() {
                        player.equipt_abilities.0 = Some(*ability);
                    }
                }
                _ => {}
            }
        }
    } else if mouse_input.just_pressed(MouseButton::Right) {
        for (interaction, ability) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked | Interaction::Hovered => {
                    for mut player in player_query.iter_mut() {
                        player.equipt_abilities.1 = Some(*ability);
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn update_equipt(
    player_query: Query<&Player, Changed<Player>>,
    mut slot_1: Query<&mut Text, (With<Slot1Text>, Without<Slot2Text>)>,
    mut slot_2: Query<&mut Text, (With<Slot2Text>, Without<Slot1Text>)>,
) {
    for player in player_query.iter() {
        if let Some(ability) = player.equipt_abilities.0 {
            let mut new_text = "Slot 1: ".to_string();
            new_text += &ability.to_string();
            for mut text in slot_1.iter_mut() {
                text.sections[0].value = new_text.clone();
            }
        }
        if let Some(ability) = player.equipt_abilities.1 {
            let mut new_text = "Slot 2: ".to_string();
            new_text += &ability.to_string();
            for mut text in slot_2.iter_mut() {
                text.sections[0].value = new_text.clone();
            }
        }
    }
}

pub fn trigger_leave(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<bevy::prelude::State<State>>,
    mut physics_time: ResMut<PhysicsTime>,
    root_node: Query<Entity, With<UiRootNode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        physics_time.set_scale(1.0);
        for entity in root_node.iter() {
            commands.entity(entity).despawn_recursive();
        }
        app_state.pop().unwrap();
    }
}
