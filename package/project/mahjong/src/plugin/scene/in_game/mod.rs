use crate::plugin::{global::WindowScaling, scene::main_menu::MainMenu};
use bevy::prelude::*;
use rand::{Rng, seq::SliceRandom};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<InGame>()
            .add_systems(OnEnter(InGame::Root), on_enter)
            .add_systems(Update, update.run_if(in_state(InGame::Root)));
    }
}

#[derive(Component, SubStates, Hash, Eq, PartialEq, Clone, Debug, Default)]
#[source(MainMenu = MainMenu::Play)]
#[states(scoped_entities)]
pub enum InGame {
    #[default]
    Root,
}

#[derive(Component)]
struct Marker;

fn on_enter(
    mut commands: Commands,
    window: Single<&Window>,
) {
    let height = window.height() / 10.0;
    let width = height * 0.7;
    let mut rng = rand::rng();
    let mut tiles: Vec<u32> = (0..144).collect();
    tiles.shuffle(&mut rng);

    let start_x = -width * 14.0 / 2.0;
    let start_y = height * 8.0 / 2.0;

    for (index, tile) in tiles.iter().enumerate() {
        commands
            .spawn((
                Marker,
                StateScoped(InGame::Root),
                Sprite::from_color(Color::BLACK, Vec2::new(width, height)),
                Pickable::default(),
                Text2d::new(tile.to_string()),
                TextFont::from_font_size(height / 5.0),
                Transform {
                    translation: Vec3 {
                        x: start_x + width * (index % 14) as f32,
                        y: start_y - height * (index / 14) as f32,
                        z: index as f32,
                    },
                    ..default()
                },
            ))
            .observe(
                |drag: Trigger<Pointer<Drag>>,
                 mut transform: Query<&mut Transform>,
                 window_scaling: Res<WindowScaling>| {
                    let mut transform = transform.get_mut(drag.target).unwrap();
                    transform.translation.x += drag.delta.x * window_scaling.value();
                    transform.translation.y -= drag.delta.y * window_scaling.value();
                },
            );
    }
}

fn update(
    // window: Single<&Window, Changed<Window>>,
    // mut height_prev: Local<Option<f32>>,
    // query: Query<(&mut Transform, &mut TextFont, &mut Sprite, &Marker)>,
    // query: Single<&Projection, With<Camera>>
    // window: Single<&Window, Changed<Window>>
    window_scaling: Res<WindowScaling>,
) {
    // info!("Scaling: {}", window_scaling.value());
    // info!("{:?}", window.resolution)
    // let height = window.height() / 10.0;
    // if let None = *height_prev {
    //     *height_prev = Some(height);
    // }

    // let height_prev = height_prev.as_mut().unwrap();
    // if height == *height_prev {
    //     return;
    // }

    // let scale = height / *height_prev;
    // let height = height * scale;
    // let width = height * 0.7;

    // for (mut transform, mut font, mut sprite, marker) in query {
    //     transform.translation.x *= scale;
    //     transform.translation.y *= scale;
    //     font.font_size *= scale;
    //     sprite.custom_size = Some(sprite.custom_size.unwrap().with_x(width).with_y(height));
    // }

    // *height_prev = height;
}
