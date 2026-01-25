use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Bar::default())
        .add_systems(Update, || {
            info!("Update!");
        })
        .run();
}

#[derive(Resource)]
struct Bar {
    val: i32,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            val: Default::default(),
            ..default() // <- stack overflow due to infinite recursion... SO OBVIOUS YET SUBTLE!
        }
    }
}
