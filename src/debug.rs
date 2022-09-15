use bevy::prelude::{Input, KeyCode, Plugin, Res, ResMut, SystemSet};
use bevy_inspector_egui::{
    plugin::InspectorWindows, Inspectable, InspectorPlugin, RegisterInspectable,
    WorldInspectorParams, WorldInspectorPlugin,
};

use crate::{entity::player::Player, input::Controllable, state::State};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(InspectorPlugin::<DebugSettings>::new())
            .register_inspectable::<Controllable>()
            .register_inspectable::<Player>()
            .insert_resource(DebugSettings::default())
            .add_startup_system(hide_at_startup)
            .add_system_set(SystemSet::on_update(State::InGame).with_system(toggle_inspector));
    }
}

#[derive(Debug, Default, Inspectable)]
pub struct DebugSettings {
    pub flying: bool,
    pub imortality: bool,
    pub unlock_camera: bool,
    pub unlock_all_abilities: bool,
}

fn toggle_inspector(
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

fn hide_at_startup(
    mut world_inspector_params: ResMut<WorldInspectorParams>,
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    world_inspector_params.enabled = false;
    let mut inspector_window_data = inspector_windows.window_data_mut::<DebugSettings>();
    inspector_window_data.visible = false;
}
