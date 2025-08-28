use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
    }
}

// fn get_cursor_world_pos(
//     mut cursor_world_pos: ResMut<CursorWorldPos>,
//     primary_window: Single<&Window, With<PrimaryWindow>>,
//     q_camera: Single<(&Camera, &GlobalTransform)>,
// ) {
//     let (main_camera, main_camera_transform) = *q_camera;

//     // Get the cursor position in the world
//     cursor_world_pos.0 = primary_window.cursor_position().and_then(|cursor_pos| {
//         main_camera
//             .viewport_to_world_2d(main_camera_transform, cursor_pos)
//             .ok()
//     });
// }
