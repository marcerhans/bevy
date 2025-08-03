use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, on_startup)
        .run();
}

#[derive(Event)]
struct Exploded;

fn on_startup(mut commands: Commands) {
    let e1 = commands
        .spawn(Name::new("1"))
        .observe(on_exploded) //
        .observe(|trigger: Trigger<Exploded>| {
            info!("WHAT? {:?}", trigger.target());
        })
        .id();
    info!("Spawned: {:?}", e1);

    // let e2 = commands
    //     .spawn((Name::new("2")))//, Observer::new(on_exploded)))
    //     .id();
    // info!("Spawned: {:?}", e2);

    // let e3 = commands.spawn(Name::new("3")).id();
    // info!("Spawned: {:?}", e3);

    info!("Before trigger");
    commands.trigger(Exploded);
    commands.trigger_targets(Exploded, e1);
    info!("After trigger");
}

fn on_exploded(trigger: Trigger<Exploded>) {
    info!("on_exploded {:?}", trigger.target());
}
