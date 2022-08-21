use bevy_inspector_egui::Inspectable;

#[derive(Debug, Default, Inspectable)]
pub struct DebugSettings {
    pub tile_collisions: bool,
    pub player_collison_box: bool,
    pub hitbox: bool,
    pub triggers: bool,
    pub noclip: bool,
}
