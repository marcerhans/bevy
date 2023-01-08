use super::*;
use bevy::window::PresentMode;
// use bevy_inspector_egui::WorldInspectorPlugin;

#[deprecated]
pub struct InitializePlugin;

impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        // #[cfg(debug_assertions)]
        // app.add_plugin(WorldInspectorPlugin::default())
        //     .add_system(bevy::window::close_on_esc);
    }
}
