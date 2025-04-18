use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins((
            bevy::DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: if cfg!(target_family = "wasm") {
                        AssetMetaCheck::Never
                    } else {
                        AssetMetaCheck::Always
                    },
                    file_path: "asset".to_string(),
                    ..default()
                }),
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(bevy::winit::WinitSettings::desktop_app());
    }
}
