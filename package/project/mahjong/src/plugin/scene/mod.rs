mod greeter;
mod a;

use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.init_state::<Startup>()
            .add_systems(Startup, startup)
            .add_plugins(greeter::Plugin)
            .add_plugins(a::Plugin);
    }
}

#[derive(States, Hash, Eq, PartialEq, Clone, Debug, Default)]
#[states(scoped_entities)]
pub enum Startup {
    #[default]
    Root,
    Greeter,
    Running,
}

fn startup(mut next_state: ResMut<NextState<Startup>>) {
    info!("Initializing...");
    next_state.set(Startup::Greeter);
}
