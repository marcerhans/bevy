mod greeter;

use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.init_state::<Startup>().add_plugins();
    }
}

#[derive(States, Hash, Eq, PartialEq, Clone, Debug, Default)]
pub enum Startup {
    #[default]
    Init,
    Greeter,
    Running,
}
