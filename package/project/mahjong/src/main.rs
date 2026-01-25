mod plugin;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(plugin::Plugin).run();
}