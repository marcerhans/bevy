use bevy::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, foo)
        .run();
}

fn foo() {
}
