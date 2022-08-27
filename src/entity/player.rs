use bevy::{
    prelude::{Added, Bundle, Changed, Component, Query, Res, Transform, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LdtkEntity, LevelSelection, Worldly};
use bevy_inspector_egui::Inspectable;

use crate::{
    abilities::{Element, Equiptment},
    animation::Animated,
    input::Controllable,
    physics::PhysicsObjectBundle,
};

impl From<EntityInstance> for Controllable {
    fn from(_: EntityInstance) -> Self {
        Controllable::new()
    }
}

#[derive(Component, Default, Inspectable)]
pub struct Player {
    pub checkpoint: Vec3,
    #[inspectable(ignore)]
    pub checkpoint_level: LevelSelection,
    pub unlocked_fire: bool,
    pub unlocked_air: bool,
    pub unlocked_water: bool,
    pub unlocked_boots: bool,
    pub combination: (Option<Equiptment>, Option<Element>),
    pub near_checkpoint: bool,
}
impl Player {
    pub fn has_equipt(&self, equiptment: Equiptment) -> bool {
        if let Some(slot) = &self.combination.0 {
            if slot == &equiptment {
                return true;
            }
        }
        false
    }

    pub fn has_infused(&self, element: Element) -> bool {
        if let Some(slot) = &self.combination.1 {
            if slot == &element {
                return true;
            }
        }
        false
    }

    pub fn get_combination_description(&self) -> &str {
        match self.combination {
            (Some(Equiptment::Staff), Some(Element::Fire)) => "Left click to cast Fireball",
            (Some(Equiptment::Staff), Some(Element::Air)) => "Left click to cast a gust of wind",
            (Some(Equiptment::Staff), Some(Element::Water)) => "Left click to summon water",
            (Some(Equiptment::MagicBoots), Some(Element::Fire)) => {
                "Jump higher with an explosive kick"
            }
            (Some(Equiptment::MagicBoots), Some(Element::Air)) => "Double jump",
            (Some(Equiptment::MagicBoots), Some(Element::Water)) => {
                "Flow like water (movement speed up)"
            }
            _ => "No effect",
        }
    }
}

#[derive(Component, Eq, PartialEq)]
pub enum AnimationState {
    Idle,
    Walking,
    JumpUp,
    JumpDown,
    Death,
}
impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Idle
    }
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
    #[grid_coords]
    grid_coords: GridCoords,
}

pub fn animation_state_update(
    mut query: Query<
        (&mut Animated, &AnimationState, &mut TextureAtlasSprite),
        Changed<AnimationState>,
    >,
) {
    for (mut animation, state, mut atlas) in query.iter_mut() {
        match state {
            AnimationState::Idle => {
                animation.start = 40;
                animation.end = 44;
            }
            AnimationState::Walking => {
                animation.start = 8;
                animation.end = 14;
            }
            AnimationState::JumpUp => {
                animation.start = 56;
                animation.end = 59;
            }
            AnimationState::JumpDown => {
                animation.start = 48;
                animation.end = 51;
            }
            AnimationState::Death => {
                animation.start = 0;
                animation.end = 8;
            }
        }
        atlas.index = animation.start;
    }
}

pub fn set_spawn(
    level_selection: Res<LevelSelection>,
    mut query: Query<(&mut Player, &mut Transform), Added<Player>>,
) {
    for (mut player, mut transform) in query.iter_mut() {
        // Note: for some reason player transform is wrong when this system runs so I've hard coded
        // it for now
        player.checkpoint.x = -536.0;
        player.checkpoint.y = 664.0;
        player.checkpoint_level = level_selection.clone();
        transform.translation.z = 5.0;
    }
}
