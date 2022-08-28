use bevy::{
    prelude::{
        BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component,
        DespawnRecursiveExt, Entity, Input, KeyCode, NodeBundle, Query, Res, ResMut, TextBundle,
        With, Without,
    },
    text::{Text, TextStyle},
    ui::{
        AlignItems, FlexDirection, Interaction, JustifyContent, Size, Style, UiColor, UiRect, Val,
    },
};
use bevy_kira_audio::{Audio, AudioControl};
use heron::PhysicsTime;

use crate::{
    abilities::{Element, Equipment},
    audio::AudioAssets,
    debug::DebugSettings,
    entity::player::Player,
    input::Controllable,
};

use super::{loading::GameAssets, State};

#[derive(Component)]
pub struct UiRootNode;
#[derive(Component)]
pub struct Slot1Text;
#[derive(Component)]
pub struct Slot2Text;
#[derive(Component)]
pub struct CombinationText;

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
    game_assets: Res<GameAssets>,
    player: Query<&Player>,
    debug_settings: Res<DebugSettings>,
) {
    let player = player
        .get_single()
        .expect("There should only be one player");

    let button_style = Style {
        size: Size::new(Val::Px(195.0), Val::Px(65.0)),
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
                        size: Size::new(Val::Px(1000.0), Val::Px(500.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    image: game_assets.menu_background.clone().into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Title text wrapper
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Px(110.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Title text
                            parent.spawn_bundle(TextBundle::from_section(
                                "Combine equipment with an element to create an ability",
                                TextStyle {
                                    font: game_assets.pixel_font.clone(),
                                    font_size: 16.0,
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
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Equipment buttons wrapper
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(50.0), Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect {
                                            left: Val::Auto,
                                            right: Val::Auto,
                                            top: Val::Px(0.0),
                                            bottom: Val::Auto,
                                        },
                                        flex_direction: FlexDirection::ColumnReverse,
                                        ..Default::default()
                                    },
                                    color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle::from_section(
                                        "Equipment:",
                                        TextStyle {
                                            font: game_assets.pixel_font.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                    // Staff button
                                    parent
                                        .spawn_bundle(ButtonBundle {
                                            style: button_style.clone(),
                                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                                            image: game_assets.button.clone().into(),
                                            ..Default::default()
                                        })
                                        .insert(Equipment::Staff)
                                        .with_children(|parent| {
                                            parent.spawn_bundle(TextBundle::from_section(
                                                "Staff",
                                                TextStyle {
                                                    font: game_assets.pixel_font.clone(),
                                                    font_size: 20.0,
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
                                                image: game_assets.button.clone().into(),
                                                ..Default::default()
                                            })
                                            .insert(Equipment::MagicBoots)
                                            .with_children(|parent| {
                                                parent.spawn_bundle(TextBundle::from_section(
                                                    "Magic Boots",
                                                    TextStyle {
                                                        font: game_assets.pixel_font.clone(),
                                                        font_size: 17.0,
                                                        color: Color::WHITE,
                                                    },
                                                ));
                                            });
                                    }
                                    // Cloak of resistance button
                                    if player.unlocked_cloak || debug_settings.unlock_all_abilities
                                    {
                                        parent
                                            .spawn_bundle(ButtonBundle {
                                                style: button_style.clone(),
                                                color: Color::rgb(0.15, 0.15, 0.15).into(),
                                                image: game_assets.button.clone().into(),
                                                ..Default::default()
                                            })
                                            .insert(Equipment::Cloak)
                                            .with_children(|parent| {
                                                parent.spawn_bundle(TextBundle::from_section(
                                                    "Cloak of\nResistance",
                                                    TextStyle {
                                                        font: game_assets.pixel_font.clone(),
                                                        font_size: 17.0,
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
                                        margin: UiRect {
                                            left: Val::Auto,
                                            right: Val::Auto,
                                            top: Val::Px(0.0),
                                            bottom: Val::Auto,
                                        },
                                        flex_direction: FlexDirection::ColumnReverse,
                                        ..Default::default()
                                    },
                                    color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle::from_section(
                                        "Elements:",
                                        TextStyle {
                                            font: game_assets.pixel_font.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                    // Fire button
                                    if player.unlocked_fire || debug_settings.unlock_all_abilities {
                                        parent
                                            .spawn_bundle(ButtonBundle {
                                                style: button_style.clone(),
                                                color: Color::rgb(0.15, 0.15, 0.15).into(),
                                                image: game_assets.button.clone().into(),
                                                ..Default::default()
                                            })
                                            .insert(Element::Fire)
                                            .with_children(|parent| {
                                                parent.spawn_bundle(TextBundle::from_section(
                                                    "Fire",
                                                    TextStyle {
                                                        font: game_assets.pixel_font.clone(),
                                                        font_size: 20.0,
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
                                                image: game_assets.button.clone().into(),
                                                ..Default::default()
                                            })
                                            .insert(Element::Air)
                                            .with_children(|parent| {
                                                parent.spawn_bundle(TextBundle::from_section(
                                                    "Air",
                                                    TextStyle {
                                                        font: game_assets.pixel_font.clone(),
                                                        font_size: 20.0,
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
                                                image: game_assets.button.clone().into(),
                                                ..Default::default()
                                            })
                                            .insert(Element::Water)
                                            .with_children(|parent| {
                                                parent.spawn_bundle(TextBundle::from_section(
                                                    "Water",
                                                    TextStyle {
                                                        font: game_assets.pixel_font.clone(),
                                                        font_size: 20.0,
                                                        color: Color::WHITE,
                                                    },
                                                ));
                                            });
                                    }
                                });
                        });

                    // header wrapper
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Px(10.0),
                                    bottom: Val::Px(10.0),
                                },
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                "Combined effect:",
                                TextStyle {
                                    font: game_assets.pixel_font.clone(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    // Combination text wrapper
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(TextBundle::from_section(
                                    player.get_combination_description(),
                                    TextStyle {
                                        font: game_assets.pixel_font.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                ))
                                .insert(CombinationText);
                        });
                    // Buttons help text wrapper
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Px(10.0),
                                    bottom: Val::Px(10.0),
                                },
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                "Press <ESC> when finished",
                                TextStyle {
                                    font: game_assets.pixel_font.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        })
        .insert(UiRootNode);
}

pub fn equipment_button_system(
    mut interaction_query: Query<
        (&Interaction, &Equipment, &mut UiColor),
        (With<Button>, Changed<Interaction>),
    >,
    mut player_query: Query<&mut Player>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for (interaction, equipment, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.55, 0.55, 0.55).into();
                audio.play(audio_assets.blip2.clone());
                for mut player in player_query.iter_mut() {
                    player.combination.0 = Some(*equipment);
                }
            }
            Interaction::Hovered => {
                audio.play(audio_assets.blip1.clone());
                *color = Color::rgb(0.35, 0.35, 0.35).into();
            }
            Interaction::None => {
                for player in player_query.iter() {
                    if player.has_equipt(*equipment) {
                        *color = Color::rgb(0.15, 0.45, 0.15).into();
                    } else {
                        *color = Color::rgb(0.15, 0.15, 0.15).into();
                    }
                }
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
    audio: Res<Audio>,
    audio_manager: Res<AudioAssets>,
) {
    for (interaction, element, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.55, 0.55, 0.55).into();
                audio.play(audio_manager.blip2.clone());
                for mut player in player_query.iter_mut() {
                    player.combination.1 = Some(*element);
                }
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.35, 0.35, 0.35).into();
                audio.play(audio_manager.blip1.clone());
            }
            Interaction::None => {
                for player in player_query.iter() {
                    if player.has_infused(*element) {
                        *color = Color::rgb(0.15, 0.45, 0.15).into();
                    } else {
                        *color = Color::rgb(0.15, 0.15, 0.15).into();
                    }
                }
            }
        }
    }
}

pub fn update_button_colours(
    mut equipment_query: Query<(&Equipment, &mut UiColor), (With<Button>, Without<Element>)>,
    mut element_query: Query<(&Element, &mut UiColor), (With<Button>, Without<Equipment>)>,
    player_query: Query<&Player, Changed<Player>>,
) {
    for player in player_query.iter() {
        for (equipment, mut color) in equipment_query.iter_mut() {
            if player.has_equipt(*equipment) {
                *color = Color::rgb(0.15, 0.45, 0.15).into();
            } else {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
        for (element, mut color) in element_query.iter_mut() {
            if player.has_infused(*element) {
                *color = Color::rgb(0.15, 0.45, 0.15).into();
            } else {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub fn update_text(
    player_query: Query<&Player, Changed<Player>>,
    mut slot_1: Query<
        &mut Text,
        (
            With<Slot1Text>,
            Without<Slot2Text>,
            Without<CombinationText>,
        ),
    >,
    mut slot_2: Query<
        &mut Text,
        (
            With<Slot2Text>,
            Without<Slot1Text>,
            Without<CombinationText>,
        ),
    >,
    mut combined: Query<
        &mut Text,
        (
            With<CombinationText>,
            Without<Slot1Text>,
            Without<Slot2Text>,
        ),
    >,
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
        for mut text in combined.iter_mut() {
            text.sections[0].value = player.get_combination_description().to_string();
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
