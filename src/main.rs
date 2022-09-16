#![allow(clippy::type_complexity, clippy::too_many_arguments)]
use bevy::{
    prelude::{App, ClearColor, Color},
    render::texture::ImageSettings,
    window::WindowDescriptor,
    DefaultPlugins,
};

use bevy_ecs_ldtk::LdtkPlugin;
use bevy_kira_audio::AudioPlugin;
use debug::DebugPlugin;

use heron::PhysicsPlugin;
use level::LevelPlugin;
use state::{ability_menu::AbilityMenuPlugin, game::GamePlugin, main_menu::MainMenuPlugin, State};

mod abilities;
mod animation;
mod audio;
mod camera;
mod damage;
mod debug;
mod destruction;
mod entity;
mod input;
mod level;
mod physics;
mod state;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_state(State::LoadMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(AbilityMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(LdtkPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(AudioPlugin)
        .insert_resource(ClearColor(Color::rgb(0.133, 0.122, 0.192)))
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            title: "Elemental Sorcerer".to_string(),
            fit_canvas_to_parent: true,
            ..Default::default()
        })
        .run();
}
