use bevy::{
    prelude::{
        BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component,
        DespawnRecursiveExt, Entity, Input, KeyCode, NodeBundle, Plugin, Query, Res, ResMut,
        SystemSet, TextBundle, With, Without,
    },
    text::{Text, TextStyle},
    ui::{
        AlignItems, FlexDirection, Interaction, JustifyContent, Size, Style, UiColor, UiRect, Val,
    },
};
use bevy_kira_audio::{Audio, AudioChannel, AudioControl};
use heron::PhysicsTime;

use crate::{
    abilities::{Element, Equipment},
    audio::{AudioAssets, SoundChannel, VolumeSettings},
    debug::DebugSettings,
    entity::player::Player,
    input::Controllable,
};

use super::{load_game::GameAssets, State};

pub struct AbilityMenuPlugin;

impl Plugin for AbilityMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_update(State::InGame).with_system(trigger_enter))
            .add_system_set(SystemSet::on_enter(State::AbilityMenu).with_system(setup))
            .add_system_set(
                SystemSet::on_update(State::AbilityMenu)
                    .with_system(trigger_leave)
                    .with_system(button_interaction_system)
                    .with_system(button_mouse_select)
                    .with_system(button_keyboard_select)
                    .with_system(update_text),
            );
    }
}

#[derive(Component)]
struct UiRootNode;
#[derive(Component)]
struct Slot1Text;
#[derive(Component)]
struct Slot2Text;
#[derive(Component)]
struct CombinationText;

#[derive(Component, Default, PartialEq, Eq, Clone, Copy)]
struct BtnGridPos {
    pub row: u8,
    pub col: u8,
}
impl BtnGridPos {
    pub fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }
}

#[derive(Default)]
struct AbilityMenuState {
    pub selected_pos: BtnGridPos,
}

fn trigger_enter(
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

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    player: Query<&Player>,
    debug_settings: Res<DebugSettings>,
    mut input: ResMut<Input<KeyCode>>,
) {
    let player = player
        .get_single()
        .expect("There should only be one player");

    input.clear(); // clear any `just_pressed` events that may be left over from previous state

    commands.insert_resource(AbilityMenuState::default());

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
                                        .insert(BtnGridPos::new(0, 0))
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
                                            .insert(BtnGridPos::new(1, 0))
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
                                            .insert(BtnGridPos::new(2, 0))
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
                                            .insert(BtnGridPos::new(0, 1))
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
                                            .insert(BtnGridPos::new(1, 1))
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
                                            .insert(BtnGridPos::new(2, 1))
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
                                "Use arrow keys & <z> to select. Press <x> when done",
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

fn button_interaction_system(
    element_button_query: Query<
        (&Interaction, &Element, &BtnGridPos),
        (With<Button>, Changed<Interaction>),
    >,
    equipment_button_query: Query<
        (&Interaction, &Equipment, &BtnGridPos),
        (With<Button>, Changed<Interaction>),
    >,
    mut player_query: Query<&mut Player>,
    mut state: ResMut<AbilityMenuState>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    for (interaction, element, grid_pos) in &element_button_query {
        match *interaction {
            Interaction::Clicked => {
                sound_channel.play(audio_assets.blip2.clone());
                for mut player in player_query.iter_mut() {
                    player.combination.1 = Some(*element);
                }
            }
            Interaction::Hovered => {
                state.selected_pos = *grid_pos;
                sound_channel.play(audio_assets.blip1.clone());
            }
            _ => {}
        }
    }
    for (interaction, equipment, grid_pos) in &equipment_button_query {
        match *interaction {
            Interaction::Clicked => {
                sound_channel.play(audio_assets.blip2.clone());
                for mut player in player_query.iter_mut() {
                    player.combination.0 = Some(*equipment);
                }
            }
            Interaction::Hovered => {
                state.selected_pos = *grid_pos;
                sound_channel.play(audio_assets.blip1.clone());
            }
            _ => {}
        }
    }
}
fn button_mouse_select(
    mut element_button_query: Query<
        (&Element, &BtnGridPos, &mut UiColor),
        (With<Button>, Without<Equipment>),
    >,
    mut equipment_button_query: Query<
        (&Equipment, &BtnGridPos, &mut UiColor),
        (With<Button>, Without<Element>),
    >,
    player_query: Query<&mut Player>,
    state: ResMut<AbilityMenuState>,
) {
    let player = player_query
        .get_single()
        .expect("There should only be one player");
    for (element, grid_pos, mut color) in &mut element_button_query {
        if state.selected_pos == *grid_pos {
            if player.has_infused(*element) {
                *color = Color::rgb(0.25, 0.55, 0.25).into();
            } else {
                *color = Color::rgb(0.35, 0.35, 0.35).into();
            }
        } else if player.has_infused(*element) {
            *color = Color::rgb(0.15, 0.45, 0.15).into();
        } else {
            *color = Color::rgb(0.15, 0.15, 0.15).into();
        }
    }
    for (equipment, grid_pos, mut color) in &mut equipment_button_query {
        if state.selected_pos == *grid_pos {
            if player.has_equipt(*equipment) {
                *color = Color::rgb(0.25, 0.55, 0.25).into();
            } else {
                *color = Color::rgb(0.35, 0.35, 0.35).into();
            }
        } else if player.has_equipt(*equipment) {
            *color = Color::rgb(0.15, 0.45, 0.15).into();
        } else {
            *color = Color::rgb(0.15, 0.15, 0.15).into();
        }
    }
}

fn button_keyboard_select(
    element_button_query: Query<(&Element, &BtnGridPos)>,
    equipment_button_query: Query<(&Equipment, &BtnGridPos)>,
    mut state: ResMut<AbilityMenuState>,
    mut player_query: Query<&mut Player>,
    keyboard_input: Res<Input<KeyCode>>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    let mut player = player_query
        .get_single_mut()
        .expect("There should only be one player");
    if keyboard_input.just_pressed(KeyCode::Down) {
        sound_channel.play(audio_assets.blip1.clone());
        state.selected_pos.row += 1;
        if state.selected_pos.col == 0 {
            if state.selected_pos.row >= player.num_equipment() {
                state.selected_pos.row = 0;
            }
        } else if state.selected_pos.col == 1 && state.selected_pos.row >= player.num_elements() {
            state.selected_pos.row = 0;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Up) {
        sound_channel.play(audio_assets.blip1.clone());
        if state.selected_pos.col == 0 {
            if state.selected_pos.row == 0 {
                state.selected_pos.row = player.num_equipment() - 1;
            } else {
                state.selected_pos.row -= 1;
            }
        } else if state.selected_pos.col == 1 && state.selected_pos.row == 0 {
            state.selected_pos.row = player.num_elements() - 1;
        } else {
            state.selected_pos.row -= 1;
        }
    }
    if (keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::Right))
        && player.num_elements() != 0
    {
        sound_channel.play(audio_assets.blip1.clone());
        if state.selected_pos.col == 0 {
            state.selected_pos.col = 1;
        } else {
            state.selected_pos.col = 0;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Z) {
        sound_channel.play(audio_assets.blip2.clone());
        for (element, grid_pos) in element_button_query.iter() {
            if *grid_pos == state.selected_pos {
                player.combination.1 = Some(*element);
                return;
            }
        }
        for (equipment, grid_pos) in equipment_button_query.iter() {
            if *grid_pos == state.selected_pos {
                player.combination.0 = Some(*equipment);
                return;
            }
        }
    }
}

fn update_text(
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

fn trigger_leave(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<bevy::prelude::State<State>>,
    mut physics_time: ResMut<PhysicsTime>,
    root_node: Query<Entity, With<UiRootNode>>,
) {
    if keyboard_input.just_pressed(KeyCode::X) {
        physics_time.set_scale(1.0);
        commands.remove_resource::<AbilityMenuState>();
        for entity in root_node.iter() {
            commands.entity(entity).despawn_recursive();
        }
        keyboard_input.clear();
        app_state.pop().unwrap();
    }
}
