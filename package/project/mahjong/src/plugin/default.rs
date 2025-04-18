use bevy::{
    app::TaskPoolThreadAssignmentPolicy, asset::AssetMetaCheck, log::LogPlugin, prelude::*,
    tasks::TaskPoolBuilder, window::PresentMode,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins((
            bevy::DefaultPlugins
                .set(TaskPoolPlugin {
                    task_pool_options: TaskPoolOptions {
                        min_total_threads: 1,
                        max_total_threads: 1,
                        io: TaskPoolThreadAssignmentPolicy {
                            min_threads: 1,
                            max_threads: 1,
                            percent: 1.0,
                            on_thread_spawn: None,
                            on_thread_destroy: None,
                        },
                        async_compute: TaskPoolThreadAssignmentPolicy {
                            min_threads: 1,
                            max_threads: 1,
                            percent: 1.0,
                            on_thread_spawn: None,
                            on_thread_destroy: None,
                        },
                        compute: TaskPoolThreadAssignmentPolicy {
                            min_threads: 1,
                            max_threads: 1,
                            percent: 1.0,
                            on_thread_spawn: None,
                            on_thread_destroy: None,
                        },
                    },
                })
                .set(LogPlugin {
                    filter: "error,mahjong=debug".into(),
                    level: bevy::log::Level::DEBUG,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Mahjong".into(),
                        name: Some("Mahjong".into()),
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
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::new(),
        ));
        // .insert_resource(bevy::winit::WinitSettings::desktop_app());
    }
}
