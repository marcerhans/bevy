use bevy::prelude::*;

use crate::plugin::screen::Screen;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<Menu>().add_plugins(root::Plugin);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates, Component)]
#[source(Screen = Screen::Menu)]
#[states(scoped_entities)]
enum Menu {
    #[default]
    Root,
    Settings,
}

mod root {
    use avian2d::{math::*, prelude::*};
    use bevy::prelude::*;

    pub struct Plugin;

    impl bevy::prelude::Plugin for Plugin {
        fn build(
            &self,
            app: &mut App,
        ) {
            app.add_plugins(PhysicsPlugins::default())
                .insert_resource(Time::<Physics>::default().with_relative_speed(2.0))
                .add_systems(OnEnter(super::Menu::Root), spawn_root);
        }
    }

    fn spawn_root(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn((
                super::Menu::Root,
                StateScoped(super::Menu::Root),
                Node {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_child(Text::new("Menu!"));

        // Spawn blue platform that belongs on the blue layer and collides with blue
        commands.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.7, 0.9),
                custom_size: Some(Vec2::new(500.0, 25.0)),
                ..default()
            },
            Transform::from_xyz(0.0, -50.0, 0.0),
            RigidBody::Static,
            Collider::rectangle(500.0, 25.0),
        ));

        let marble_radius = 7.5;
        let marble_mesh = meshes.add(Circle::new(marble_radius));

        // Spawn blue marbles that belong on the blue layer and collide with blue
        let blue_material = materials.add(Color::srgb(0.2, 0.7, 0.9));
        for x in -6..6 {
            for y in 0..4 {
                commands.spawn((
                    Mesh2d(marble_mesh.clone()),
                    MeshMaterial2d(blue_material.clone()),
                    Transform::from_xyz(
                        x as f32 * 2.5 * marble_radius,
                        y as f32 * 2.5 * marble_radius + 200.0,
                        0.0,
                    ),
                    RigidBody::Dynamic,
                    Collider::circle(marble_radius as Scalar),
                ));
            }
        }
    }
}
