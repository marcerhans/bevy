use bevy::{
    asset::AssetMetaCheck, camera::ScalingMode, log::LogPlugin, prelude::*, window::PresentMode,
};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins((
            bevy::DefaultPlugins
                .set(LogPlugin {
                    filter: "error,bevy=info,mahjong=debug".into(),
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
        ));

        app.world_mut().spawn((
            Camera2d,
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: 1080.0,
                },
                ..OrthographicProjection::default_2d()
            }),
            Msaa::Off,
        ));
    }
}
