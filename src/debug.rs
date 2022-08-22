use bevy_inspector_egui::Inspectable;

#[derive(Debug, Default, Inspectable)]
pub struct DebugSettings {
    pub flying: bool,
    pub imortality: bool,
    pub unlock_camera: bool,
}
