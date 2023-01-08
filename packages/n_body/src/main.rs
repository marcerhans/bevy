mod constants;
mod plugins;

use bevy::prelude::*;
use plugins::*;

fn main() {
    App::new()
        .add_plugin(InitializePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(MiscPlugin)
        .run();
}
