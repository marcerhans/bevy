use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins((
            #[cfg(debug_assertions)]
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            #[cfg(debug_assertions)]
            WorldInspectorPlugin::new(),
            PhysicsPlugins::default(),
        ));
    }
}
