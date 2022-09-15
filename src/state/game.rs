use bevy::prelude::{Commands, ParallelSystemDescriptorCoercion, Plugin, Res, SystemSet};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_kira_audio::{Audio, AudioControl};

use crate::{
    abilities, animation, audio::AudioAssets, camera, damage, destruction, entity, input, physics,
};

use super::{load_game::GameAssets, State};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loading_state(
            LoadingState::new(State::LoadGame)
                .continue_to_state(State::InGame)
                .with_collection::<GameAssets>()
                .with_collection::<AudioAssets>(),
        )
        .add_system_set(SystemSet::on_enter(State::InGame).with_system(setup))
        .add_system_set(
            SystemSet::on_update(State::InGame)
                .with_system(input::system.label(input::InputLabel::ControllableUpdate))
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
        );
    }
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
