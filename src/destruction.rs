use bevy::{
    prelude::{Commands, Component, DespawnRecursiveExt, Entity, Query, Res},
    time::{Time, Timer},
};

#[derive(Component)]
pub struct DestructionTimer(pub Timer);

pub fn destroy(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut DestructionTimer)>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
