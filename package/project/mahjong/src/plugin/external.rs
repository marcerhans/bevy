use bevy::prelude::*;
// use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        // app.add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()));
    }
}
