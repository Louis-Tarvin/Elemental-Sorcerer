use bevy::prelude::{Input, KeyCode, Res, ResMut};
use bevy_inspector_egui::{plugin::InspectorWindows, Inspectable, WorldInspectorParams};

#[derive(Debug, Default, Inspectable)]
pub struct DebugSettings {
    pub flying: bool,
    pub imortality: bool,
    pub unlock_camera: bool,
    pub unlock_all_abilities: bool,
}

pub fn toggle_inspector(
    input: Res<Input<KeyCode>>,
    mut world_inspector_params: ResMut<WorldInspectorParams>,
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    if input.just_pressed(KeyCode::F1) {
        world_inspector_params.enabled = !world_inspector_params.enabled;
        let mut inspector_window_data = inspector_windows.window_data_mut::<DebugSettings>();
        inspector_window_data.visible = !inspector_window_data.visible;
    }
}

pub fn hide_at_startup(
    mut world_inspector_params: ResMut<WorldInspectorParams>,
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    world_inspector_params.enabled = false;
    let mut inspector_window_data = inspector_windows.window_data_mut::<DebugSettings>();
    inspector_window_data.visible = false;
}
