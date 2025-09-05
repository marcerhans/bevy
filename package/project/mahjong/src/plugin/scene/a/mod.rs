use bevy::prelude::*;
use crate::plugin::scene::Startup;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
    }
}


#[derive(SubStates, Hash, Eq, PartialEq, Clone, Debug, Default)]
#[source(Startup = Startup::Running)]
#[states(scoped_entities)]
pub enum Menu {
    #[default]
    Root,
    Play,
    Settings,
    Quit,
}
