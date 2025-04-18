use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
    }
}

fn window_space_to_world_space_coordinates(

) {

}
// fn clickable_system(
//     mouse_button_input: Res<ButtonInput<MouseButton>>,
//     window: Single<&Window>,
//     camera_transform: Single<(&Camera, &GlobalTransform, &CameraMain)>,
//     mut query: Query<(Entity, &Transform, &Sprite), With<Clickable>>,
//     mut writer: EventWriter<event::Clicked>,
// ) {
//     if let Some(cursor_pos) = window.cursor_position() {
//         if mouse_button_input.just_pressed(MouseButton::Left) {
//             let (camera, camera_transform, _) = camera_transform.into_inner();
//             if let Ok(click_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
//                 for (entity, transform, sprite) in &mut query {
//                     let translation = transform.translation;
//                     let size = sprite
//                         .custom_size
//                         .expect("Clickable object is lacking custom size.");
//                     let half_size = size / 2.0;

//                     if (click_pos.x >= translation.x - half_size.x)
//                         && (click_pos.x <= translation.x + half_size.x)
//                         && (click_pos.y >= translation.y - half_size.y)
//                         && (click_pos.y <= translation.y + half_size.y)
//                     {
//                         warn!("{}", "Writing!");
//                         writer.write(event::Clicked(entity));
//                     }
//                 }
//             }
//         }
//     }
// }