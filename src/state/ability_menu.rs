use bevy::{
    prelude::{
        AssetServer, BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component,
        DespawnRecursiveExt, Entity, Input, KeyCode, NodeBundle, Query, Res, ResMut, TextBundle,
        With, Without,
    },
    text::{Text, TextStyle},
    ui::{
        AlignItems, FlexDirection, Interaction, JustifyContent, Size, Style, UiColor, UiRect, Val,
    },
};
use heron::PhysicsTime;

use crate::{
    abilities::{Element, Equiptment},
    debug::DebugSettings,
    entity::player::Player,
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

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Query<&Player>,
    debug_settings: Res<DebugSettings>,
) {
    let player = player
        .get_single()
        .expect("There should only be one player");

    let button_style = Style {
        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
        // center button
        margin: UiRect {
            left: Val::Auto,
            right: Val::Auto,
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
        },
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..Default::default()
    };

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
                            // Equiptment buttons wrapper
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(50.0), Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::all(Val::Auto),
                                        flex_direction: FlexDirection::ColumnReverse,
                                        ..Default::default()
                                    },
                                    color: Color::rgba(0.0, 0.2, 0.0, 0.8).into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    // Staff button
                                    parent
                                        .spawn_bundle(ButtonBundle {
                                            style: button_style.clone(),
                                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                                            ..Default::default()
                                        })
                                        .insert(Equiptment::Staff)
                                        .with_children(|parent| {
                                            parent.spawn_bundle(TextBundle::from_section(
                                                "Staff",
                                                TextStyle {
                                                    font: asset_server.load("fonts/roboto.ttf"),
                                                    font_size: 30.0,
                                                    color: Color::WHITE,
                                                },
                                            ));
                                        });
                                    // Magic Boots button
                                    if player.unlocked_boots || debug_settings.unlock_all_abilities
                                    {
                                        parent
                                            .spawn_bundle(ButtonBundle {
                                                style: button_style.clone(),
                                                color: Color::rgb(0.15, 0.15, 0.15).into(),
                                                ..Default::default()
                                            })
                                            .insert(Equiptment::MagicBoots)
                                            .with_children(|parent| {
                                                parent.spawn_bundle(TextBundle::from_section(
                                                    "Magic Boots",
                                                    TextStyle {
                                                        font: asset_server.load("fonts/roboto.ttf"),
                                                        font_size: 30.0,
                                                        color: Color::WHITE,
                                                    },
                                                ));
                                            });
                                    }
                                });

                            // Element buttons wrapper
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(50.0), Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::all(Val::Auto),
                                        flex_direction: FlexDirection::ColumnReverse,
                                        ..Default::default()
                                    },
                                    color: Color::rgba(0.0, 0.0, 0.2, 0.8).into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    // Fire button
                                    if player.unlocked_fire || debug_settings.unlock_all_abilities {
                                        parent
                                            .spawn_bundle(ButtonBundle {
                                                style: button_style.clone(),
                                                color: Color::rgb(0.15, 0.15, 0.15).into(),
                                                ..Default::default()
                                            })
                                            .insert(Element::Fire)
                                            .with_children(|parent| {
                                                parent.spawn_bundle(TextBundle::from_section(
                                                    "Fire",
                                                    TextStyle {
                                                        font: asset_server.load("fonts/roboto.ttf"),
                                                        font_size: 30.0,
                                                        color: Color::WHITE,
                                                    },
                                                ));
                                            });
                                    }

                                    // Air button
                                    if player.unlocked_air || debug_settings.unlock_all_abilities {
                                        parent
                                            .spawn_bundle(ButtonBundle {
                                                style: button_style.clone(),
                                                color: Color::rgb(0.15, 0.15, 0.15).into(),
                                                ..Default::default()
                                            })
                                            .insert(Element::Air)
                                            .with_children(|parent| {
                                                parent.spawn_bundle(TextBundle::from_section(
                                                    "Air",
                                                    TextStyle {
                                                        font: asset_server.load("fonts/roboto.ttf"),
                                                        font_size: 30.0,
                                                        color: Color::WHITE,
                                                    },
                                                ));
                                            });
                                    }

                                    // Water button
                                    if player.unlocked_water || debug_settings.unlock_all_abilities
                                    {
                                        parent
                                            .spawn_bundle(ButtonBundle {
                                                style: button_style,
                                                color: Color::rgb(0.15, 0.15, 0.15).into(),
                                                ..Default::default()
                                            })
                                            .insert(Element::Water)
                                            .with_children(|parent| {
                                                parent.spawn_bundle(TextBundle::from_section(
                                                    "Water",
                                                    TextStyle {
                                                        font: asset_server.load("fonts/roboto.ttf"),
                                                        font_size: 30.0,
                                                        color: Color::WHITE,
                                                    },
                                                ));
                                            });
                                    }
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
                            // Equiptment text
                            let text = if let Some(equiptment) = player.combination.0 {
                                equiptment.to_string()
                            } else {
                                "None".to_string()
                            };
                            parent
                                .spawn_bundle(TextBundle::from_section(
                                    "Equipt: ".to_string() + &text,
                                    TextStyle {
                                        font: asset_server.load("fonts/roboto.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                ))
                                .insert(Slot1Text);

                            // Element text
                            let text = if let Some(element) = player.combination.1 {
                                element.to_string()
                            } else {
                                "None".to_string()
                            };
                            parent
                                .spawn_bundle(TextBundle::from_section(
                                    "Element: ".to_string() + &text,
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

pub fn equiptment_button_system(
    mut interaction_query: Query<
        (&Interaction, &Equiptment, &mut UiColor),
        (With<Button>, Changed<Interaction>),
    >,
    mut player_query: Query<&mut Player>,
) {
    for (interaction, equiptment, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.55, 0.55, 0.55).into();
                for mut player in player_query.iter_mut() {
                    player.combination.0 = Some(*equiptment);
                }
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.35, 0.35, 0.35).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub fn element_button_system(
    mut interaction_query: Query<
        (&Interaction, &Element, &mut UiColor),
        (With<Button>, Changed<Interaction>),
    >,
    mut player_query: Query<&mut Player>,
) {
    for (interaction, element, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.55, 0.55, 0.55).into();
                for mut player in player_query.iter_mut() {
                    player.combination.1 = Some(*element);
                }
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.35, 0.35, 0.35).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
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
        if let Some(ability) = player.combination.0 {
            let mut new_text = "Equipt: ".to_string();
            new_text += &ability.to_string();
            for mut text in slot_1.iter_mut() {
                text.sections[0].value = new_text.clone();
            }
        }
        if let Some(ability) = player.combination.1 {
            let mut new_text = "Element: ".to_string();
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
