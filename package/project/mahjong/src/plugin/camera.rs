use bevy::{
    core_pipeline::{
        bloom::{Bloom, BloomCompositeMode, BloomPrefilter},
        tonemapping::{DebandDither, Tonemapping},
    },
    prelude::*,
    render::view::RenderLayers,
};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems(Startup, init);
    }
}

mod camera {
    use super::*;

    pub enum Type {
        Main,
        Bloom,
        Custom(String)
    }

    #[derive(Component)]
    pub struct ID(pub Type);
}

fn init(mut commands: Commands) {
    // Bloom layer
    let layer: usize = 0;
    commands.spawn((
        camera::ID(camera::Type::Bloom),
        RenderLayers::layer(layer),
        Camera2d::default(),
        Camera {
            order: layer as isize,
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::TonyMcMapface,
        DebandDither::Enabled,
        Bloom {
            intensity: 0.1,
            low_frequency_boost: 0.1,
            low_frequency_boost_curvature: 1.0,
            high_pass_frequency: 0.2,
            composite_mode: BloomCompositeMode::Additive,
            prefilter: BloomPrefilter {
                threshold: 0.8,
                threshold_softness: 0.2,
            },
            ..default()
        },
    ));

    // Sprite layer
    let layer: usize = 1;
    commands.spawn((
        camera::ID(camera::Type::Main),
        RenderLayers::layer(layer),
        Camera2d::default(),
        Camera {
            order: layer as isize,
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        DebandDither::Enabled,
    ));
}
