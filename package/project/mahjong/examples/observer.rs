use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: None,
            ..default()
        }))
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

    let e2 = commands
        .spawn((Name::new("2"), Observer::new(on_exploded)))
        .id();
    info!("Spawned: {:?}", e2);

    let e3 = commands
        .spawn(Name::new("3"))
        .observe(on_exploded) //
        .observe(|trigger: Trigger<Exploded>| {
            info!("WHAT? {:?}", trigger.target());
        })
        .id();
    info!("Spawned: {:?}", e3);

    info!("Before trigger");
    commands.trigger(Exploded);
    commands.trigger_targets(Exploded, e1);
    commands.trigger_targets(Exploded, e2);
    commands.trigger_targets(Exploded, e3);
    info!("After trigger");
}

fn on_exploded(
    trigger: Trigger<Exploded>,
    name: Query<&Name>,
    mut counter: Local<u32>,
) {
    info!("on_exploded! count: {:?}", *counter);
    *counter += 1;

    if let Ok(name) = name.get(trigger.target()) {
        info!("on_exploded {:?}|{:?}", name, trigger.target());
    }
}
