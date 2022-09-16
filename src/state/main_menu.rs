use bevy::{
    prelude::{
        BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, Color, Commands, Component,
        DespawnRecursiveExt, Entity, NodeBundle, Plugin, Query, Res, ResMut, SystemSet, TextBundle,
        Transform, With,
    },
    text::{TextSection, TextStyle},
    ui::{
        AlignItems, FlexDirection, Interaction, JustifyContent, Size, Style, UiColor, UiRect, Val,
    },
};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_kira_audio::{Audio, AudioControl};

use crate::audio::{AudioAssets, VolumeSettings};

use super::{load_menu::MenuAssets, State};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loading_state(
            LoadingState::new(State::LoadMenu)
                .continue_to_state(State::MainMenu)
                .with_collection::<MenuAssets>()
                .with_collection::<AudioAssets>(),
        )
        .add_system_set(SystemSet::on_enter(State::MainMenu).with_system(setup))
        .add_system_set(SystemSet::on_update(State::MainMenu).with_system(button_system))
        .add_system_set(SystemSet::on_exit(State::MainMenu).with_system(cleanup));
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
enum MenuButton {
    Start,
    Sound,
    Music,
}

fn setup(mut commands: Commands, menu_assets: Res<MenuAssets>) {
    commands.insert_resource(VolumeSettings::default());

    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 900.0),
        ..Default::default()
    });
    let button_style = Style {
        size: Size::new(Val::Px(195.0), Val::Px(65.0)),
        // center button
        margin: UiRect::all(Val::Px(20.0)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..Default::default()
    };
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                padding: UiRect::all(Val::Px(20.)),
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            image: menu_assets.background.clone().into(),
            ..Default::default()
        })
        .insert(MainMenu)
        .with_children(|parent| {
            // header wrapper
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(600.0), Val::Px(110.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_sections([
                        TextSection::new(
                            "Elemental\n",
                            TextStyle {
                                font: menu_assets.pixel_font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.435, 0.62, 0.145),
                            },
                        ),
                        TextSection::new(
                            " Sorcerer",
                            TextStyle {
                                font: menu_assets.pixel_font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.404, 0.561, 0.796),
                            },
                        ),
                    ]));
                });
            // Buttons wrapper
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(600.0), Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            image: menu_assets.button.clone().into(),
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .insert(MenuButton::Start)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                "Start",
                                TextStyle {
                                    font: menu_assets.pixel_font.clone(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            image: menu_assets.button.clone().into(),
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .insert(MenuButton::Sound)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                "Toggle\nSFX Volume",
                                TextStyle {
                                    font: menu_assets.pixel_font.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            image: menu_assets.button.clone().into(),
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .insert(MenuButton::Music)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                "Toggle\nMusic Volume",
                                TextStyle {
                                    font: menu_assets.pixel_font.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        });
}

fn button_system(
    mut interaction_query: Query<
        (&MenuButton, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<bevy::prelude::State<State>>,
    mut volume_settings: ResMut<VolumeSettings>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for (button, interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.55, 0.55, 0.55).into();
                match button {
                    MenuButton::Start => {
                        state.set(State::LoadGame).unwrap();
                    }
                    MenuButton::Sound => {
                        volume_settings.toggle_sfx_vol();
                        audio
                            .play(audio_assets.blip1.clone())
                            .with_volume(volume_settings.sfx_vol);
                    }
                    MenuButton::Music => {
                        volume_settings.toggle_music_vol();
                        audio
                            .play(audio_assets.blip1.clone())
                            .with_volume(volume_settings.music_vol);
                    }
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

fn cleanup(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
