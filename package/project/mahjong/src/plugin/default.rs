use std::time::Duration;

use bevy::{
    asset::AssetMetaCheck,
    camera::{ScalingMode, visibility::RenderLayers},
    diagnostic::FrameTimeDiagnosticsPlugin,
    log::LogPlugin,
    prelude::*,
    window::PresentMode,
    winit::{UpdateMode, WinitSettings},
};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        let default_winit_settings = DefaultWinitSettings(WinitSettings {
            focused_mode: UpdateMode::Reactive {
                wait: Duration::from_millis((1000.0 / 1.0) as u64),
                react_to_device_events: true,
                react_to_user_events: true,
                react_to_window_events: true,
            },
            unfocused_mode: UpdateMode::Reactive {
                wait: Duration::from_millis(1000),
                react_to_device_events: false,
                react_to_user_events: false,
                react_to_window_events: true,
            },
        });
        app.insert_resource(default_winit_settings.clone())
            .insert_resource(default_winit_settings);

        app.add_plugins((
            bevy::DefaultPlugins
                .set(LogPlugin {
                    filter: "error,bevy=info,mahjong=info".into(),
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
            MeshPickingPlugin,
            FrameTimeDiagnosticsPlugin::default(),
        ));

        let base_cam = (
            Camera2d,
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: 1080.0,
                },
                ..OrthographicProjection::default_2d()
            }),
            Msaa::Off,
        );

        for layer in 0..=1 as usize {
            app.world_mut().spawn((
                base_cam.clone(),
                Camera {
                    clear_color: ClearColorConfig::None,
                    order: layer as isize,
                    ..Camera::default()
                },
                RenderLayers::layer(layer),
            ));
        }
    }
}

#[derive(Resource, Clone)]
pub struct DefaultWinitSettings(pub WinitSettings);