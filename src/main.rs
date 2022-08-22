#![allow(clippy::type_complexity)]
use bevy::{
    prelude::{
        App, AssetServer, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, ClearColor,
        Color, Commands, Query, Res, SystemSet, TextBundle, Vec3, With,
    },
    render::texture::ImageSettings,
    text::TextStyle,
    ui::{AlignItems, Interaction, JustifyContent, Size, Style, UiColor, UiRect, Val},
    DefaultPlugins,
};
use bevy_ecs_ldtk::{
    prelude::RegisterLdtkObjects, LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSelection,
    LevelSpawnBehavior,
};
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use debug::DebugSettings;
use entity::{checkpoint::CheckpointBundle, player::PlayerBundle, signpost::SignpostBundle};
use heron::{Gravity, PhysicsPlugin};
use input::Controllable;

mod animation;
mod camera;
mod debug;
mod entity;
mod input;
mod level;
mod physics;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum State {
    MainMenu,
    Loading,
    InGame,
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_state(State::InGame)
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<DebugSettings>::new())
        .register_inspectable::<Controllable>()
        .insert_resource(ClearColor(Color::rgb(0.133, 0.122, 0.192)))
        .insert_resource(DebugSettings::default())
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..Default::default()
        })
        .insert_resource(Gravity::from(Vec3::new(0.0, -500.0, 0.0)))
        .add_system_set(SystemSet::on_enter(State::MainMenu).with_system(main_menu_setup))
        .add_system_set(SystemSet::on_update(State::MainMenu).with_system(button_system))
        .add_system_set(
            SystemSet::on_enter(State::InGame)
                // .with_system(spawn_player)
                .with_system(setup),
        )
        .add_system_set(
            SystemSet::on_update(State::InGame)
                .with_system(input::system)
                .label(input::InputLabel::ControllableUpdate)
                .with_system(level::spawn_wall_collision)
                .with_system(level::update_level_selection)
                .with_system(level::restart_level)
                .with_system(animation::system)
                .with_system(animation::state_update_system)
                .with_system(camera::follow)
                .with_system(camera::set_zoom)
                .with_system(physics::add_ground_sensor)
                .with_system(physics::check_grounded)
                .with_system(physics::handle_controllables)
                .with_system(entity::signpost::spawn_text)
                .with_system(entity::signpost::check_near)
                .with_system(entity::checkpoint::check_near),
        )
        .register_ldtk_int_cell::<level::WallBundle>(1)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<CheckpointBundle>("Checkpoint")
        .register_ldtk_entity::<SignpostBundle>("Signpost")
        .run();
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(TextBundle::from_section(
        "Sample Text",
        TextStyle {
            font: asset_server.load("fonts/roboto.ttf"),
            font_size: 30.0,
            color: Color::WHITE,
        },
    ));
    commands
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
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Start",
                TextStyle {
                    font: asset_server.load("fonts/roboto.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/test.ldtk"),
        ..Default::default()
    });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.55, 0.55, 0.55).into();
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
