use bevy::{
    prelude::{Bundle, Component, Vec3},
    sprite::SpriteSheetBundle,
};
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity, LevelSelection, Worldly};

use crate::{
    animation::{Animated, AnimationState},
    input::Controllable,
    physics::PhysicsObjectBundle,
};

impl From<EntityInstance> for Controllable {
    fn from(_: EntityInstance) -> Self {
        Controllable::new(100.0, 180.0, 400.0, true)
    }
}

#[derive(Component, Default)]
pub struct Player {
    pub checkpoint: Vec3,
    pub checkpoint_level: LevelSelection,
}

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[bundle]
    #[sprite_sheet_bundle("sprites/herochar_spritesheet.png", 16.0, 16.0, 8, 15, 0.0, 0.0, 0)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[worldly]
    pub worldly: Worldly,
    pub player: Player,
    #[bundle]
    #[from_entity_instance]
    pub physics_bundle: PhysicsObjectBundle,
    #[from_entity_instance]
    pub controllable: Controllable,
    #[from_entity_instance]
    pub animated: Animated,
    pub animation_state: AnimationState,
}

// pub fn spawn_player(
// mut commands: Commands,
// asset_server: Res<AssetServer>,
// mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
// let texture_handle = asset_server.load("chars/herochar_spritesheet.png");
// let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 8, 15);
// let texture_atlas_handle = texture_atlases.add(texture_atlas);
// let mut player = commands.spawn_bundle(SpriteSheetBundle {
// texture_atlas: texture_atlas_handle,
// transform: Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
// ..Default::default()
// });
// player.insert(Player);
// player.insert(Controllable::new(100.0, true));
// player.insert(RigidBody::Dynamic);
// player.insert(CollisionShape::Capsule {
// half_segment: 5.0,
// radius: 5.0,
// });
// player.insert(RotationConstraints::lock());
// player.insert(Velocity::default());
// player.insert(Animated::new(0.1, false));
// }
