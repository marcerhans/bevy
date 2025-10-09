use crate::plugin::scene::main_menu::MainMenu;
use bevy::prelude::*;
use generator::*;
use rand::seq::SliceRandom;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<InGame>()
            .insert_resource(PreviouslySelectedTile::default())
            .add_systems(OnEnter(InGame::Root), on_enter);
        // .add_systems(Update, update.run_if(in_state(InGame::Root)));
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
struct Tile;

#[derive(Component)]
struct ID(usize);

fn on_enter(
    mut commands: Commands,
    window: Single<&Window>,
) {
    let height = window.height() / 8.0;
    let width = height * 0.7;
    let mut rng = rand::rng();
    let mut tiles: Vec<usize> = (0..Generator::<Turtle>::TILES).map(|x| x / 2).collect();
    tiles.shuffle(&mut rng);

    let placer = Placer::new(Vec2::new(width, height), Generator::<Turtle>::new());

    let columns = 14.0;
    let rows = 8.0;
    let start_x = -width * columns / 2.0;
    let start_y = height * rows / 2.0;

    for ((index, tile), pos) in tiles.iter().enumerate().zip(placer.into_iter()) {
        commands
            .spawn((
                Tile,
                ID(*tile),
                DespawnOnExit(InGame::Root),
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
                TextColor::BLACK,
                Transform {
                    translation: Vec3 {
                        x: start_x + pos.x,
                        y: start_y - pos.y,
                        z: index as f32,
                    },
                    ..default()
                },
            ))
            // .observe(
            //     |drag: On<Pointer<Drag>>,
            //      mut transform: Query<&mut Transform>,
            //      window_scaling: Res<WindowScaling>| {
            //         let mut transform = transform.get_mut(drag.target).unwrap();
            //         transform.translation.x += drag.delta.x * window_scaling.value();
            //         transform.translation.y -= drag.delta.y * window_scaling.value();
            //     },
            // )
            .observe(on_click);
    }
}

fn on_click(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    mut previous_res: ResMut<PreviouslySelectedTile>,
    query: Query<(&ID, &Transform, &Sprite), With<Tile>>,
) {
    if previous_res.0.is_none() {
        info!("{:?}", click);
        previous_res.0 = Some(click.original_event_target());
        return;
    }

    let previous_entity = previous_res.0.unwrap();
    let previous_id = query.get(previous_entity).unwrap().0.0;
    let current_entity = click.original_event_target();
    let current_id = query.get(current_entity).unwrap().0.0;

    if previous_entity != current_entity && previous_id == current_id {
        info!("Match! Do something!");

        let valid_removal = {
            todo!();
            // // Removal is valid if there is no tile to the left, right, or above the selected pair (individually).
            // for (_, transform, sprite) in query {}

            // // fn no_tile_to_left_or_right() -> bool {
            // //     false
            // // }

            false
        };

        if valid_removal {
            commands.entity(previous_entity).despawn();
            commands.entity(current_entity).despawn();
            previous_res.0 = None;
            return;
        }
    }

    info!("Not a match :(");
    previous_res.0 = Some(click.original_event_target());
}

mod generator {
    use bevy::prelude::Vec2;
    use std::marker::PhantomData;

    pub struct Turtle;

    pub struct Generator<T>(PhantomData<T>);

    impl<T> Generator<T> {
        pub fn new() -> Self {
            Self(PhantomData::<T>)
        }
    }

    impl Generator<Turtle> {
        pub const TILES: usize = 144;
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
}
