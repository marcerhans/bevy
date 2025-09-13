use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, on_startup)
        .add_systems(Update, (on_update, on_update2.after(on_update)))
        .run();
}

#[derive(Event)]
struct Exploded;

fn on_startup(mut commands: Commands) {
    let e1 = commands
        .spawn(Name::new("1"))
        // .observe(on_exploded) //
        .observe(|trigger: Trigger<Exploded>| {
            info!("WHAT1? {:?}", trigger.target());
        })
        .id();
    info!("Spawned: {:?}", e1);

    let e2 = commands
        .spawn((Name::new("2"), Observer::new(on_exploded)))
        .id();
    info!("Spawned: {:?}", e2);

    let e3 = commands
        .spawn(Name::new("3"))
        // .observe(on_exploded) //
        .observe(|trigger: Trigger<Exploded>| {
            info!("WHAT3? {:?}", trigger.target());
        })
        .id();
    info!("Spawned: {:?}", e3);
}

fn on_update(
    mut commands: Commands,
    entities: Query<Entity>,
    mut local: Local<u32>,
) {
    if *local < 2 {
        info!("Before trigger(s)");
        for e in entities {
            commands.trigger_targets(Exploded, e);
        }
        info!("After trigger(s)");

        *local += 1;
    }
}

fn on_update2(mut local: Local<u32>) {
    if *local < 2 {
        info!("Just random update system, after the main update system.");
        * local += 1;
    }
}

fn on_exploded(
    trigger: Trigger<Exploded>,
    mut counter: Local<u32>,
) {
    info!("on_exploded {:?}|{:?}", *counter, trigger.target());
    *counter += 1;
}
