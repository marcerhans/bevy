use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_framepace::FramepacePlugin;

use super::*;

pub struct MiscPlugin;

impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(FramepacePlugin)
            .insert_resource(fps::Counter::default())
            .add_startup_system(initialize)
            .add_system(fps::system);
    }
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn initialize(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut counter: ResMut<fps::Counter>,
) {
}

pub mod fps {
    use super::*;
    use bevy::diagnostic::Diagnostics;

    #[derive(Resource, Default)]
    pub struct Counter {
        fps: f32,
    }

    impl Counter {
        pub fn get(&self) -> f32 {
            self.fps
        }

        fn set(&mut self, val: f32) {
            self.fps = val;
        }
    }

    pub fn system(diagnostics: Res<Diagnostics>, mut counter: ResMut<Counter>) {
        counter.fps = 100.0;

        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                counter.set(average as f32);
            }
        };
    }
}
