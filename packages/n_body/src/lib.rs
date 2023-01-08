mod constants;
mod plugins;

use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};
pub use plugins::gen_ui;
pub use plugins::state::{State, MAX_BODIES};
pub use plugins::CameraOffset;
use plugins::*;

pub struct NBodyPlugins {
    pub cam_offset: CameraOffset,
}

impl PluginGroup for NBodyPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();
        group
            .add(CameraPlugin {
                cam_offset: self.cam_offset,
            })
            .add(NBodyPlugin)
    }
}
