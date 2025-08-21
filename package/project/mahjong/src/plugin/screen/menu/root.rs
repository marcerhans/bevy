use bevy::prelude::*;
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems(OnEnter(super::Menu::Root), on_enter);
    }
}

fn on_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const SCENE_FILE_PATH: &str = "scene/menu/root.scn.ron";
    let scene = asset_server.load(SCENE_FILE_PATH);
    commands.spawn(DynamicSceneRoot(scene));
}
