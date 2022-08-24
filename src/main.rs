#![allow(clippy::type_complexity)]
use bevy::{
    prelude::{
        App, AssetServer, Camera2dBundle, ClearColor, Color, Commands,
        ParallelSystemDescriptorCoercion, Res, SystemSet, Vec3,
    },
    render::texture::ImageSettings,
    DefaultPlugins,
};
use bevy_ecs_ldtk::{
    prelude::RegisterLdtkObjects, LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSelection,
    LevelSpawnBehavior,
};
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use debug::DebugSettings;
use entity::{
    ability::AbilityBundle,
    block::BlockBundle,
    checkpoint::CheckpointBundle,
    goblin::GoblinBundle,
    player::{Player, PlayerBundle},
    signpost::SignpostBundle,
};
use heron::{Gravity, PhysicsPlugin};
use input::Controllable;
use state::State;

mod abilities;
mod animation;
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
        .add_state(State::InGame)
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<DebugSettings>::new())
        .register_inspectable::<Controllable>()
        .register_inspectable::<Player>()
        .insert_resource(ClearColor(Color::rgb(0.133, 0.122, 0.192)))
        .insert_resource(DebugSettings::default())
        .insert_resource(LevelSelection::Index(5))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..Default::default()
        })
        .insert_resource(Gravity::from(Vec3::new(0.0, -500.0, 0.0)))
        .add_system_set(SystemSet::on_enter(State::MainMenu).with_system(state::main_menu::setup))
        .add_system_set(
            SystemSet::on_update(State::MainMenu).with_system(state::main_menu::button_system),
        )
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
                .with_system(level::spawn_spike_collision)
                .with_system(level::update_level_selection)
                .with_system(level::restart_level)
                .with_system(animation::system)
                .with_system(camera::follow)
                .with_system(camera::set_zoom)
                .with_system(destruction::destroy)
                .with_system(physics::add_ground_sensor)
                .with_system(physics::check_grounded.label(physics::PhysicsLabel::CheckGrounded))
                .with_system(
                    physics::handle_controllables
                        .label(physics::PhysicsLabel::HandleControllables)
                        .after(physics::PhysicsLabel::CheckGrounded),
                )
                .with_system(state::ability_menu::trigger_enter)
                .with_system(abilities::use_ability)
                .with_system(abilities::projectile_collision)
                .with_system(damage::detect)
                .with_system(damage::kill.after(physics::PhysicsLabel::HandleControllables))
                .with_system(damage::respawn)
                .with_system(entity::player::animation_state_update)
                .with_system(entity::player::set_spawn)
                .with_system(entity::goblin::patrol)
                .with_system(entity::goblin::animation_state_update)
                .with_system(entity::goblin::face_direction)
                .with_system(entity::ability::check_near)
                .with_system(entity::signpost::spawn_text)
                .with_system(entity::signpost::check_near)
                .with_system(entity::checkpoint::check_near),
        )
        .add_system_set(
            SystemSet::on_enter(State::AbilityMenu).with_system(state::ability_menu::setup),
        )
        .add_system_set(
            SystemSet::on_update(State::AbilityMenu)
                .with_system(state::ability_menu::trigger_leave)
                .with_system(state::ability_menu::equiptment_button_system)
                .with_system(state::ability_menu::element_button_system)
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
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/test.ldtk"),
        ..Default::default()
    });
}
