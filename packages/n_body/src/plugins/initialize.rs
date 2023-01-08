use super::*;
use bevy::window::PresentMode;

pub struct InitializePlugin;

impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: APPLICATION_NAME.to_string(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            },
            ..default()
        }));

        // #[cfg(debug_assertions)]
        // app.add_plugin(WorldInspectorPlugin::default())
        //     .add_system(bevy::window::close_on_esc);
    }
}
