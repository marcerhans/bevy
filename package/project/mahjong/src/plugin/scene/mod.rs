mod greeter;
mod in_game;
mod main_menu;

use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.init_state::<Startup>()
            .insert_resource(MyTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Startup, startup)
            .add_systems(Update, print_fps)
            .add_plugins((greeter::Plugin, main_menu::Plugin));
    }
}

#[derive(States, Hash, Eq, PartialEq, Clone, Debug, Default)]
#[states(scoped_entities)]
pub enum Startup {
    #[default]
    Root,
    Greeter,
    MainMenu,
}

#[derive(Resource)]
struct MyTimer(Timer);

fn startup(mut next_state: ResMut<NextState<Startup>>) {
    info!("Initializing...");
    next_state.set(Startup::Greeter);
}

fn print_fps(
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
    mut timer: ResMut<MyTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            info!("FPS: {}", value);
        }
    }
}
