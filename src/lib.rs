//! Hub for various applications/games made with bevy and egui.

mod constants;
mod plugins;

use bevy::prelude::*;
use n_body::{CameraOffset, NBodyPlugins};
use plugins::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    App::new()
        .add_plugin(InitializePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(MiscPlugin)
        .add_plugins(NBodyPlugins {
            cam_offset: CameraOffset {
                x: WIDTH_MARGIN,
                y: HEIGHT_MARGIN,
            },
        })

        .run();
}
