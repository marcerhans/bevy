use bevy::{asset::AssetMetaCheck, log::LogPlugin, prelude::*, window::PresentMode};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins(
            bevy::DefaultPlugins
                .set(LogPlugin {
                    filter: "error,mahjong=debug".into(),
                    level: bevy::log::Level::DEBUG,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Mah Jong".into(),
                        name: Some("Mah Jong".into()),
                        present_mode: PresentMode::AutoNoVsync,
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
        );

        app.world_mut().spawn(Camera2d);
    }
}
