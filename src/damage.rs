use bevy::{
    prelude::{
        Added, Commands, Component, Entity, EventReader, Handle, Query, Res, ResMut, Transform,
        With,
    },
    time::{Time, Timer},
};
use bevy_ecs_ldtk::{LdtkLevel, LevelSelection, Respawn};
use bevy_kira_audio::{Audio, AudioControl};
use heron::{CollisionEvent, Velocity};

use crate::{
    audio::AudioAssets,
    debug::DebugSettings,
    entity::player::{AnimationState, Player},
    input::Controllable,
    physics::GroundDetector,
};

#[derive(Component, Default)]
pub struct Hurtbox;

#[derive(Component)]
pub struct Killed;

#[derive(Component)]
pub struct RespawnTimer(Timer);

pub fn detect(
    mut commands: Commands,
    hurtboxes: Query<Entity, With<Hurtbox>>,
    player: Query<Entity, With<Player>>,
    mut collision_events: EventReader<CollisionEvent>,
    debug_settings: Res<DebugSettings>,
) {
    collision_events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|e| {
            let (e1, e2) = e.rigid_body_entities();
            if player.get(e1).is_ok() && hurtboxes.get(e2).is_ok() && !debug_settings.imortality {
                return Some(e1);
            } else if player.get(e2).is_ok()
                && hurtboxes.get(e1).is_ok()
                && !debug_settings.imortality
            {
                return Some(e2);
            }
            None
        })
        .for_each(|entity| {
            commands.entity(entity).insert(Killed {});
        });
}

pub fn kill(
    mut commands: Commands,
    mut player: Query<(Entity, &mut AnimationState, &mut Velocity), With<Player>>,
    killed: Query<Entity, Added<Killed>>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for entity in killed.iter() {
        if let Ok((player_entity, mut state, mut velocity)) = player.get_mut(entity) {
            *state = AnimationState::Death;
            velocity.linear.x = 0.0;
            velocity.linear.y = 0.0;
            commands
                .entity(player_entity)
                .remove::<Controllable>()
                .insert(RespawnTimer(Timer::from_seconds(0.6, false)));
            audio.play(audio_assets.death.clone());
        }
    }
}

pub fn respawn(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Transform, &mut RespawnTimer, &Player)>,
    level_query: Query<Entity, With<Handle<LdtkLevel>>>,
    mut level_selection: ResMut<LevelSelection>,
    mut detectors: Query<&mut GroundDetector>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut timer, player) in player.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands
                .entity(entity)
                .remove::<RespawnTimer>()
                .remove::<Killed>()
                .insert(Controllable::new());
            transform.translation = player.checkpoint;
            *level_selection = player.checkpoint_level.clone();
            for level_entity in level_query.iter() {
                commands.entity(level_entity).insert(Respawn);
            }
        }
        for mut detector in detectors.iter_mut() {
            detector.active_collisions = 0;
        }
    }
}
