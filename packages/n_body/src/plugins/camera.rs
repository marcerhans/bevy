use super::*;

#[derive(Resource, Copy, Clone)]
pub struct CameraOffset {
    pub x: f32,
    pub y: f32,
}

pub struct CameraPlugin {
    pub cam_offset: CameraOffset,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.cam_offset)
            .add_startup_system(initialize);
    }
}

fn initialize(mut commands: Commands, cam_offset: Res<CameraOffset>) {
    commands.spawn(Camera2dBundle {
        // projection: Projection::Orthographic(OrthographicProjection::default()), // For 3D
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    });
}
