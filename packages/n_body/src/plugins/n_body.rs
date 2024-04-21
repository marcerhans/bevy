use super::*;

use rand::Rng;

pub struct NBodyPlugin;

impl Plugin for NBodyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(state::State::default())
            .add_startup_system(init_assets)
            .add_system(read_state)
            .add_system(physics_dot.after(read_state));
    }
}

pub mod state {
    use super::*;
    pub const MAX_BODIES: usize = 1000;

    #[derive(Resource)]
    pub struct State {
        pub body_amount: usize,
        pub body_color: Color,
        pub line_distance_limit: f32,
        pub line_color: Color,
        pub line_draw: bool,

        pub asset_dot: Option<Handle<Image>>,
        pub asset_pixel: Option<Handle<Image>>,
    }

    impl Default for State {
        fn default() -> Self {
            Self {
                body_amount: 100,
                body_color: Color::Rgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: 1.0,
                },
                line_distance_limit: 200.0,
                line_color: Color::Rgba {
                    red: 0.5,
                    green: 1.0,
                    blue: 1.0,
                    alpha: 1.0,
                },
                line_draw: true,
                asset_dot: None,
                asset_pixel: None,
            }
        }
    }
}

#[derive(Component)]
struct Velocity(f32, f32);

#[derive(Component)]
struct Dot;

impl Dot {
    fn new_sprite(
        position: (f32, f32),
        size: f32,
        color: &Color,
        image: Handle<Image>,
    ) -> SpriteBundle {
        SpriteBundle {
            texture: image,
            transform: Transform::from_xyz(position.0, position.1, default()),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: size, y: size }),
                color: color.clone(),
                ..default()
            },
            ..default()
        }
    }
}

#[derive(Component)]
struct Line;

impl Line {
    fn new_sprite(image: Handle<Image>) -> SpriteBundle {
        SpriteBundle {
            texture: image,
            transform: Transform {
                translation: Vec3::NEG_Z,
                rotation: default(),
                scale: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
            sprite: Sprite {
                custom_size: Some(Vec2::default()),
                color: Color::WHITE,
                ..default()
            },
            visibility: Visibility::INVISIBLE,
            ..default()
        }
    }

    fn gen_tween_line(
        from: (f32, f32),
        to: (f32, f32),
        distance_limit: f32,
        image: Handle<Image>,
    ) -> SpriteBundle {
        let x_delta = from.0 - to.0;
        let y_delta = from.1 - to.1;
        let hypot = x_delta.hypot(y_delta);
        let angle = (-x_delta / hypot).acos() * if y_delta < 0.0 { 1.0 } else { -1.0 };

        SpriteBundle {
            texture: image,
            transform: Transform {
                translation: Vec3 {
                    x: from.0 + (hypot / 2.0 + 1.0 / 2.0) * angle.cos(),
                    y: from.1 + (1.0 / 2.0 + hypot / 2.0) * angle.sin(),
                    z: default(),
                },
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
            sprite: Sprite {
                custom_size: Some(Vec2 { x: hypot, y: 1.0 }),
                color: Color::rgba_linear(
                    0.5,
                    1.0,
                    1.0,
                    1.0 - (hypot.min(distance_limit)) / distance_limit,
                ),
                ..default()
            },
            visibility: if hypot < distance_limit {
                Visibility::VISIBLE
            } else {
                Visibility::INVISIBLE
            },
            ..default()
        }
    }

    fn update_tween_line(
        line_transform: &mut Transform,
        line_sprite: &mut Sprite,
        line_visibility: &mut Visibility,
        distance_limit: f32,
        color: &Color,
        from: (f32, f32),
        to: (f32, f32),
    ) {
        // TODO: Duplicated code
        let x_delta = from.0 - to.0;
        let y_delta = from.1 - to.1;
        let hypot = x_delta.hypot(y_delta);

        // Defer from updating if we are not going to draw it anyway.
        *line_visibility = if hypot < distance_limit {
            Visibility::VISIBLE
        } else {
            Visibility::INVISIBLE
        };

        if line_visibility.is_visible == false {
            return;
        }

        let angle = (-x_delta / hypot).acos() * if y_delta < 0.0 { 1.0 } else { -1.0 };

        // Update position
        line_transform.translation.x = from.0 + (hypot / 2.0 + 1.0 / 2.0) * angle.cos();
        line_transform.translation.y = from.1 + (1.0 / 2.0 + hypot / 2.0) * angle.sin();

        // Update sprite length (size)
        line_sprite.custom_size = Some(Vec2 { x: hypot, y: 1.0 });

        // Update rotation
        line_transform.rotation = Quat::from_rotation_z(angle);

        // Update colors
        line_sprite.color = *color
            .clone()
            .set_a(color.a() - (hypot.min(distance_limit)) / distance_limit);
    }
}

fn init_assets(mut state: ResMut<state::State>, asset_server: Res<AssetServer>) {
    state.asset_dot = Some(
        asset_server.load::<Image, &str>("images/misc/white_dot.png"),
    );
    state.asset_pixel = Some(
        asset_server.load::<Image, &str>("images/misc/white_pixel.png"),
    );
}

fn read_state(
    mut commands: Commands,
    state: Res<state::State>,
    windows: Res<Windows>,
    camera_offset: Res<CameraOffset>,
    dots: Query<Entity, With<Dot>>,
    lines: Query<Entity, With<Line>>,
) {
    if dots.iter().len() > state.body_amount {
        let mut lines = lines.iter();

        for (i, dot) in dots.iter().skip(state.body_amount).enumerate() {
            commands.entity(dot).despawn();

            if let Some(upper) = (dots.iter().len() - i).checked_sub(1) {
                for _ in 0..upper {
                    commands
                        .entity(lines.next().expect("Tried to despawn line for dot."))
                        .despawn();
                }
            }
        }
    } else if dots.iter().len() < state.body_amount {
        let window = windows.get_primary().unwrap();
        let mut rng = rand::thread_rng();
        let dots_to_create = state.body_amount - dots.iter().len();

        for dots_created in 0..dots_to_create {
            let velocity = Velocity(rng.gen_range(-2.0..2.0), rng.gen_range(-2.0..2.0));
            let dot = Dot::new_sprite(
                (
                    rng.gen_range(
                        (-window.width() / 2.0 + camera_offset.x)..(window.width() / 2.0),
                    ),
                    rng.gen_range(
                        (-window.height() / 2.0)..(window.height() / 2.0 - camera_offset.y),
                    ),
                ),
                rng.gen_range(5.0..10.0),
                &state.body_color,
                state.asset_dot.as_ref().unwrap().clone(),
            );

            commands.spawn((Dot, dot, velocity));

            if let Some(upper) = (dots.iter().len() + dots_created).checked_sub(1) {
                for _ in 0..=upper {
                    commands.spawn((
                        Line,
                        Line::new_sprite(state.asset_pixel.as_ref().unwrap().clone()),
                    ));
                }
            }
        }
    }
}

fn physics_dot(
    state: Res<state::State>,
    mut dots: Query<(&mut Transform, &mut Velocity, &mut Sprite), (With<Dot>, Without<Line>)>,
    mut lines: Query<(&mut Transform, &mut Sprite, &mut Visibility), (With<Line>, Without<Dot>)>,
    window: ResMut<Windows>,
    camera_offset: Res<CameraOffset>,
) {
    let window = window.get_primary().unwrap();
    let view_width = window.width() / 2.0;
    let view_height = window.height() / 2.0;

    for (mut transform, velocity, mut sprite) in dots.iter_mut() {
        enum Axis {
            X,
            Y,
        }

        fn warped_translation(
            axis: Axis,
            axis_pos: &mut f32,
            axis_velocity: &f32,
            axis_camera_offset: &f32,
            view_len: &f32,
        ) {
            let axis_term_x = match axis {
                Axis::X => *axis_camera_offset,
                Axis::Y => 0.0,
            };
            let axis_term_y = match axis {
                Axis::X => 0.0,
                Axis::Y => *axis_camera_offset,
            };

            if *axis_pos > *view_len - axis_term_y {
                *axis_pos = -*view_len + *axis_velocity + axis_term_x;
            } else if *axis_pos < -*view_len + axis_term_x {
                *axis_pos = *view_len + *axis_velocity - axis_term_y;
            } else {
                *axis_pos += *axis_velocity;
            }
        }

        warped_translation(
            Axis::X,
            &mut transform.translation.x,
            &velocity.0,
            &camera_offset.x,
            &view_width,
        );
        warped_translation(
            Axis::Y,
            &mut transform.translation.y,
            &velocity.1,
            &camera_offset.y,
            &view_height,
        );

        sprite.color = state.body_color.clone();
    }

    let n_dots = dots.iter().len();

    if n_dots < 2 {
        return;
    }

    let dots: Vec<(&Transform, _, _)> = dots.iter().collect();
    let mut lines = lines.iter_mut();
    for i in 0..(n_dots - 1) {
        for j in (i + 1)..n_dots {
            let (mut transform, mut sprite, mut visibility) =
                lines.next().expect("Failed to fetch next line to draw.");

            if !state.line_draw {
                *visibility = Visibility::INVISIBLE;
                continue;
            }

            let from = (dots[i].0.translation.x, dots[i].0.translation.y);
            let to = (dots[j].0.translation.x, dots[j].0.translation.y);

            Line::update_tween_line(
                &mut transform,
                &mut sprite,
                &mut visibility,
                state.line_distance_limit,
                &state.line_color,
                from,
                to,
            );
        }
    }
}
