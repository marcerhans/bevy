pub mod default;
pub mod external;
pub mod scene;

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
            scene::Plugin,
        ));
    }
}