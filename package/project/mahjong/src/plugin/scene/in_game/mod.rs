use std::marker::PhantomData;

use crate::plugin::{global::WindowScaling, scene::main_menu::MainMenu};
use bevy::prelude::*;
use rand::seq::SliceRandom;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<InGame>()
            .insert_resource(PreviouslySelectedTile::default())
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

#[derive(Resource, Default)]
struct PreviouslySelectedTile(Option<Entity>);

#[derive(Component)]
struct Marker;

#[derive(Component)]
struct ID(usize);

fn on_enter(
    mut commands: Commands,
    window: Single<&Window>,
) {
    let height = window.height() / 8.0;
    let width = height * 0.7;
    let mut rng = rand::rng();
    let mut tiles: Vec<usize> = (0..Generator::<Turtle>::TILES).collect();
    tiles.shuffle(&mut rng);

    let placer = Placer::new(Vec2::new(width, height), Generator::<Turtle>::new());

    let columns = 14.0;
    let rows = 8.0;
    let start_x = -width * columns / 2.0;
    let start_y = height * rows / 2.0;

    for ((index, tile), pos) in tiles.iter().enumerate().zip(placer.into_iter()) {
        commands
            .spawn((
                Marker,
                ID(*tile),
                StateScoped(InGame::Root),
                Sprite::from_color(
                    match index {
                        ..87 => Color::srgb_u8(255, 0, 0),
                        87..123 => Color::srgb_u8(0, 255, 0),
                        123..139 => Color::srgb_u8(0, 0, 255),
                        139..143 => Color::srgb_u8(255, 255, 0),
                        143.. => Color::srgb_u8(255, 0, 255),
                    },
                    Vec2::new(width, height),
                ),
                Pickable::default(),
                Text2d::new(tile.to_string()),
                TextFont::from_font_size(height / 5.0),
                Transform {
                    translation: Vec3 {
                        x: start_x + pos.x,
                        y: start_y - pos.y,
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
            )
            .observe(
                |click: Trigger<Pointer<Click>>,
                 mut previous: ResMut<PreviouslySelectedTile>,
                 id: Query<&ID, With<Marker>>| {
                    if previous.0.is_none() {
                        info!("{:?}", click.target);
                        previous.0 = Some(click.target);
                        return;
                    }

                    if id.get(click.target).unwrap().0 == id.get(previous.0.unwrap()).unwrap().0 {
                        info!("Match! Do something!");
                        previous.0 = None;
                    } else {
                        info!("Not a match :(");
                        previous.0 = Some(click.target);
                    }
                },
            );
    }

    // Determine position(s)
    // Spawn
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

struct Turtle;

struct Generator<T>(PhantomData<T>);

impl<T> Generator<T> {
    pub fn new() -> Self {
        Self(PhantomData::<T>)
    }
}

impl Generator<Turtle> {
    const TILES: usize = 144;
}

impl PositionGenerator for Generator<Turtle> {
    fn generate(
        &self,
        tile_size: Vec2,
        current: usize,
    ) -> Option<Vec2> {
        if current >= Self::TILES {
            return None;
        }

        let layer;
        let row;
        match current {
            ..87 => {
                layer = 0;
                match current {
                    0..12 => row = 0,
                    12..20 => row = 1,
                    20..30 => row = 2,
                    30..42 => row = 3,
                    42..54 => row = 4,
                    54..64 => row = 5,
                    64..72 => row = 6,
                    72..84 => row = 7,
                    84..87 => row = 8,
                    _ => unreachable!(),
                };
            },
            87..123 => {
                layer = 1;
                row = (current - 87) / 6 + 1;
            },
            123..139 => {
                layer = 2;
                row = (current - 123) / 4 + 2;
            },
            139..143 => {
                layer = 3;
                row = (current - 139) / 2 + 3;
            },
            143 => {
                // Special case. Just return value immediately.
                return Some(Vec2::new(5.5, 3.5) * tile_size);
            },
            _ => return None,
        }

        let column = match layer {
            0 => {
                match row {
                    0 => 0 + current - 0,
                    1 => 2 + current - 12,
                    2 => 1 + current - 20,
                    3 => 0 + current - 30,
                    4 => 0 + current - 42,
                    5 => 1 + current - 54,
                    6 => 2 + current - 64,
                    7 => 0 + current - 72,
                    8 => match current - 84 {
                        // Last 3 are special cases. Do not follow a pattern.
                        0 => return Some(Vec2::new(-1.0, 3.5) * tile_size),
                        1 => return Some(Vec2::new(12.0, 3.5) * tile_size),
                        2 => return Some(Vec2::new(13.0, 3.5) * tile_size),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            },
            1 => 3 + ((current - 87) % 6),
            2 => 4 + ((current - 123) % 4),
            3 => 5 + ((current - 139) % 2),
            _ => unreachable!(),
        };

        Some(Vec2::new(column as f32, row as f32) * tile_size)
    }
}

pub trait PositionGenerator {
    fn generate(
        &self,
        tile_size: Vec2,
        current: usize,
    ) -> Option<Vec2>;
}

pub struct Placer<G: PositionGenerator> {
    tile_size: Vec2,
    generator: G,
}

impl<G: PositionGenerator> Placer<G> {
    pub fn new(
        tile_size: Vec2,
        generator: G,
    ) -> Self {
        Self {
            tile_size,
            generator,
        }
    }
}

pub struct PlacerIterator<'a, G: PositionGenerator> {
    placer: &'a Placer<G>,
    counter: usize,
}

type PlaceIteratorItem = Vec2;

impl<'a, G: PositionGenerator> Iterator for PlacerIterator<'a, G> {
    type Item = PlaceIteratorItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        self.placer
            .generator
            .generate(self.placer.tile_size, self.counter - 1)
    }
}

impl<'a, G: PositionGenerator> IntoIterator for &'a Placer<G> {
    type Item = PlaceIteratorItem;
    type IntoIter = PlacerIterator<'a, G>;

    fn into_iter(self) -> Self::IntoIter {
        PlacerIterator {
            placer: self,
            counter: 0,
        }
    }
}
