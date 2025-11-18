use bevy::{prelude::*, camera::ScalingMode};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        // app.insert_resource(WindowScaling::default())
        //     .add_systems(PreUpdate, on_preupdate);
    }
}

// #[derive(Resource, Default)]
// pub struct WindowScaling {
//     value: f32,
// }

// impl WindowScaling {
//     pub fn value(&self) -> f32 {
//         self.value
//     }
// }

// fn on_preupdate(
//     window: Single<&Window>,
//     projection: Single<&Projection>,
//     mut window_scaling: ResMut<WindowScaling>,
// ) {
//     if let Projection::Orthographic(projection) = *projection {
//         if let ScalingMode::FixedVertical { viewport_height } = projection.scaling_mode {
//             window_scaling.value = viewport_height / window.resolution.height() as f32;
//         }
//     }
// }
