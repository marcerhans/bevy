use bevy::prelude::*;
use gravity::GravityPlugins;

fn main() {
    App::new()
        .add_plugins(GravityPlugins)
        .add_startup_system(add_entities)
        .add_system(hello_world)
        .add_system(hello_entities)
        .add_system(hello_entities_named)
        .run();
}

#[derive(Component)]
struct Entity(String);

#[derive(Component)]
struct EntityNamed {
    name: String,
}

fn add_entities(mut commands: Commands) {
    commands.spawn(Entity("Klas".to_string()));
    commands.spawn(Entity("Göran".to_string()));
    commands.spawn(Entity("Marcus".to_string()));
    commands.spawn(EntityNamed {
        name: "Klas".to_string(),
    });
    commands.spawn(EntityNamed {
        name: "Klas".to_string(),
    });
    commands.spawn(EntityNamed {
        name: "Klas".to_string(),
    });
}

fn hello_world() {
    println!("Hejsan! :D");
}

fn hello_entities(query: Query<&Entity>) {
    for entity in query.iter() {
        println!("No name name: {}", entity.0);
    }
}

fn hello_entities_named(query: Query<&EntityNamed>) {
    for entity in query.iter() {
        println!("Name name: {}", entity.name);
    }
}
