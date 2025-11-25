use bevy::{
    camera::{ScalingMode, visibility::RenderLayers},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(|app: &mut App| {
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
        })
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                RenderLayers::layer(1),
                Sprite::from_color(Color::BLACK, Vec2::new(50.0, 50.0)),
            ));
            commands.spawn((
                RenderLayers::layer(0),
                Sprite::from_color(Color::WHITE, Vec2::new(50.0, 50.0)),
                Transform {
                    translation: Vec3::splat(25.0).with_z(0.0),
                    ..default()
                },
            ));
        })
        .run();
}
