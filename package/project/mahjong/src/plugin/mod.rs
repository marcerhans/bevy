pub mod camera;
pub mod default;
pub mod external;
pub mod input;
pub mod screen;
pub mod shared;

use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins((
            default::Plugin,
            external::Plugin,
            input::Plugin,
            shared::Plugin,
            camera::Plugin,
            screen::Plugin,
        ));
    }
}
