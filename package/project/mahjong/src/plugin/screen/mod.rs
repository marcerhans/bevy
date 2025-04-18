mod greeter;
mod in_game;
mod menu;

use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins((greeter::Plugin, menu::Plugin, in_game::Plugin))
            .init_state::<Screen>()
            .add_systems(OnEnter(Screen::Greeter), || {
                debug!("Entering screen: Greeter")
            })
            .add_systems(OnEnter(Screen::Menu), || {
                debug!("Entering screen: Menu ")
            })
            .add_systems(OnEnter(Screen::InGame), || {
                debug!("Entering screen: InGame")
            });
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
#[states(scoped_entities)]
enum Screen {
    #[default]
    Greeter,
    Menu,
    InGame,
}
