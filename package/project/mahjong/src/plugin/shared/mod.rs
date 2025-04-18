pub mod component;
pub mod event;
pub mod resource;
pub mod system;
pub mod util;

use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins((
            component::Plugin,
            event::Plugin,
            resource::Plugin,
            system::Plugin,
            util::Plugin,
        ));
    }
}
