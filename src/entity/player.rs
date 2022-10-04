use bevy::{
    prelude::{
        Added, BuildChildren, Bundle, Camera, Changed, Commands, Component, Entity, Query, Res,
        Transform, Vec3, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LdtkEntity, LevelSelection, Worldly};
use bevy_inspector_egui::Inspectable;
use heron::Acceleration;

use crate::{
    abilities::{Element, Equipment},
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
    pub unlocked_cloak: bool,
    pub combination: (Option<Equipment>, Option<Element>),
    pub near_checkpoint: bool,
}
impl Player {
    pub fn has_equipt(&self, equipment: Equipment) -> bool {
        if let Some(slot) = &self.combination.0 {
            if slot == &equipment {
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
            (Some(Equipment::Staff), Some(Element::Fire)) => "<x> to cast Fireball",
            (Some(Equipment::Staff), Some(Element::Air)) => "<x> to cast a gust of wind",
            (Some(Equipment::Staff), Some(Element::Water)) => "<x> to summon water",
            (Some(Equipment::MagicBoots), Some(Element::Fire)) => {
                "Jump higher with an explosive kick"
            }
            (Some(Equipment::MagicBoots), Some(Element::Air)) => "Double jump",
            (Some(Equipment::MagicBoots), Some(Element::Water)) => {
                "Flow like water (movement speed up)"
            }
            (Some(Equipment::Cloak), Some(Element::Fire)) => "Lava resistance",
            (Some(Equipment::Cloak), Some(Element::Water)) => "Water resistance",
            _ => "No effect",
        }
    }

    /// Get the number of equipment unlocked (including staff)
    pub fn num_equipment(&self) -> u8 {
        let mut count = 1;
        if self.unlocked_boots {
            count += 1;
        }
        if self.unlocked_cloak {
            count += 1;
        }
        count
    }

    /// Get the number of elements unlocked
    pub fn num_elements(&self) -> u8 {
        let mut count = 0;
        if self.unlocked_fire {
            count += 1;
        }
        if self.unlocked_air {
            count += 1;
        }
        if self.unlocked_water {
            count += 1;
        }
        count
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
    pub acceleration: Acceleration,
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
    mut commands: Commands,
    level_selection: Res<LevelSelection>,
    mut query: Query<(Entity, &mut Player, &mut Transform), Added<Player>>,
    camera: Query<Entity, With<Camera>>,
) {
    for (entity, mut player, mut transform) in query.iter_mut() {
        // Note: for some reason player transform is wrong when this system runs so I've hard coded
        // it for now
        player.checkpoint.x = -1064.0;
        player.checkpoint.y = 776.0;
        player.checkpoint_level = level_selection.clone();
        transform.translation.z = 7.0;
        for camera in camera.iter() {
            commands.entity(entity).add_child(camera);
        }
    }
}
