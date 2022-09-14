#![allow(clippy::type_complexity, clippy::too_many_arguments)]
use audio::AudioAssets;
use bevy::{
    prelude::{
        App, ClearColor, Color, Commands, ParallelSystemDescriptorCoercion, Res, SystemSet, Vec3,
    },
    render::texture::ImageSettings,
    window::WindowDescriptor,
    DefaultPlugins,
};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_ecs_ldtk::{
    prelude::RegisterLdtkObjects, LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSelection,
    LevelSpawnBehavior,
};
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin};
use debug::DebugSettings;
use entity::{
    ability::AbilityBundle,
    block::{BlockBundle, WoodBlockBundle},
    checkpoint::CheckpointBundle,
    goblin::GoblinBundle,
    lava::LavaBundle,
    player::{Player, PlayerBundle},
    signpost::SignpostBundle,
    torch::TorchBundle,
    trophy::TrophyBundle,
    water::WaterBundle,
};
use heron::{Gravity, PhysicsPlugin};
use input::Controllable;
use state::{load_game::GameAssets, load_menu::MenuAssets, State};

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
        .add_plugin(LdtkPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<DebugSettings>::new())
        .register_inspectable::<Controllable>()
        .register_inspectable::<Player>()
        .insert_resource(ClearColor(Color::rgb(0.133, 0.122, 0.192)))
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            title: "Elemental Sorcerer".to_string(),
            fit_canvas_to_parent: true,
            ..Default::default()
        })
        .insert_resource(DebugSettings::default())
        .insert_resource(LevelSelection::Index(1))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..Default::default()
        })
        .insert_resource(Gravity::from(Vec3::new(0.0, -500.0, 0.0)))
        .add_startup_system(level::prevent_asset_unloading)
        .add_startup_system(debug::hide_at_startup)
        .add_loading_state(
            LoadingState::new(State::LoadGame)
                .continue_to_state(State::InGame)
                .with_collection::<GameAssets>()
                .with_collection::<AudioAssets>(),
        )
        .add_loading_state(
            LoadingState::new(State::LoadMenu)
                .continue_to_state(State::MainMenu)
                .with_collection::<MenuAssets>(),
        )
        .add_system_set(SystemSet::on_enter(State::MainMenu).with_system(state::main_menu::setup))
        .add_system_set(
            SystemSet::on_update(State::MainMenu).with_system(state::main_menu::button_system),
        )
        .add_system_set(SystemSet::on_exit(State::MainMenu).with_system(state::main_menu::cleanup))
        .add_system_set(SystemSet::on_enter(State::InGame).with_system(setup))
        .add_system_set(
            SystemSet::on_update(State::InGame)
                .with_system(debug::toggle_inspector)
                .with_system(input::system.label(input::InputLabel::ControllableUpdate))
                .with_system(level::spawn_wall_collision)
                .with_system(level::spawn_spike_collision)
                .with_system(level::update_level_selection)
                .with_system(level::pause_physics_during_load)
                .with_system(level::restart_level)
                .with_system(animation::system)
                .with_system(camera::set_zoom)
                .with_system(destruction::destroy)
                .with_system(physics::add_ground_sensor)
                .with_system(
                    physics::check_grounded
                        .label(physics::PhysicsLabel::CheckGrounded)
                        .after(input::InputLabel::ControllableUpdate),
                )
                .with_system(
                    physics::handle_controllables
                        .label(physics::PhysicsLabel::HandleControllables)
                        .after(physics::PhysicsLabel::CheckGrounded),
                )
                .with_system(state::ability_menu::trigger_enter)
                .with_system(abilities::use_ability)
                .with_system(abilities::fire_projectile_collision)
                .with_system(abilities::wind_projectile_collision)
                .with_system(abilities::water_projectile_collision)
                .with_system(damage::detect)
                .with_system(damage::kill.after(physics::PhysicsLabel::HandleControllables))
                .with_system(damage::respawn)
                .with_system(
                    entity::player::animation_state_update
                        .after(physics::PhysicsLabel::HandleControllables),
                )
                .with_system(entity::player::set_spawn)
                .with_system(entity::goblin::patrol)
                .with_system(entity::goblin::init_animation_state)
                .with_system(entity::goblin::animation_state_update)
                .with_system(entity::goblin::face_direction)
                .with_system(entity::ability::check_near)
                .with_system(entity::ability::dont_spawn_if_collected)
                .with_system(entity::signpost::spawn_text)
                .with_system(entity::signpost::check_near)
                .with_system(entity::lava::check_collision)
                .with_system(entity::water::check_collision)
                .with_system(entity::checkpoint::check_near)
                .with_system(entity::checkpoint::offset)
                .with_system(entity::torch::offset)
                .with_system(entity::trophy::check_near),
        )
        .add_system_set(
            SystemSet::on_enter(State::AbilityMenu).with_system(state::ability_menu::setup),
        )
        .add_system_set(
            SystemSet::on_update(State::AbilityMenu)
                .with_system(state::ability_menu::trigger_leave)
                .with_system(state::ability_menu::button_interaction_system)
                .with_system(state::ability_menu::button_mouse_select)
                .with_system(state::ability_menu::button_keyboard_select)
                .with_system(state::ability_menu::update_text),
        )
        .register_ldtk_int_cell::<level::WallBundle>(1)
        .register_ldtk_int_cell::<level::SpikeBundle>(2)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<CheckpointBundle>("Checkpoint")
        .register_ldtk_entity::<SignpostBundle>("Signpost")
        .register_ldtk_entity::<AbilityBundle>("Ability")
        .register_ldtk_entity::<GoblinBundle>("Goblin")
        .register_ldtk_entity::<BlockBundle>("Block")
        .register_ldtk_entity::<WoodBlockBundle>("WoodBlock")
        .register_ldtk_entity::<LavaBundle>("Lava")
        .register_ldtk_entity::<WaterBundle>("Water")
        .register_ldtk_entity::<TorchBundle>("Torch")
        .register_ldtk_entity::<TrophyBundle>("Trophy")
        .run();
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: game_assets.level.clone(),
        ..Default::default()
    });
    audio
        .play(audio_assets.bgm.clone())
        .with_volume(0.5)
        .looped();
}
