mod root;

use crate::plugin::screen::Screen;
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<Menu>().add_plugins(root::Plugin);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates, Component)]
#[source(Screen = Screen::Menu)]
#[states(scoped_entities)]
enum Menu {
    #[default]
    Root,
    Play,
    Settings,
    Quit,
}
