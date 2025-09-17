pub mod default;
pub mod external;
pub mod scene;
pub mod global;

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
            global::Plugin,
            scene::Plugin,
        ));
    }
}