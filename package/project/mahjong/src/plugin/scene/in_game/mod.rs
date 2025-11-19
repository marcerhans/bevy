use crate::plugin::scene::main_menu::MainMenu;
use bevy::prelude::*;
use generator::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        use on_enter::*;

        app.add_sub_state::<InGame>()
            .add_systems(OnEnter(InGame::Root), spawn_tiles);
    }
}

#[derive(SubStates, Default, Debug, Hash, Eq, PartialEq, Clone)]
#[source(MainMenu = MainMenu::Play)]
#[states(scoped_entities)]
enum InGame {
    #[default]
    Root,
}

mod tile {
    use bevy::prelude::*;

    #[derive(Component, Clone)]
    pub struct Marker;

    #[derive(Component, Clone)]
    pub struct ID(pub usize);

    pub struct Factory {
        texture_tile: Handle<Image>,
        texture_alliance: Handle<Image>,
        texture_horde: Handle<Image>,
        custom_size: Option<Vec2>,
    }

    pub enum Variant {
        Alliance(usize),
        Horde(usize),
    }

    impl Factory {
        pub fn new(
            texture_tile: Handle<Image>,
            texture_alliance: Handle<Image>,
            texture_horde: Handle<Image>,
            custom_size: Option<Vec2>,
        ) -> Self {
            Self {
                texture_tile,
                texture_alliance,
                texture_horde,
                custom_size,
            }
        }

        pub fn get_tile(
            &self,
            variant: Variant,
        ) -> impl Bundle {
            (
                Marker,
                Sprite {
                    custom_size: self.custom_size,
                    ..Sprite::from_image(self.texture_tile.clone())
                },
                children![match variant {
                    Variant::Horde(icons) => (
                        Text2d::new(icons.to_string()),
                        TextColor::WHITE,
                        TextFont::from_font_size(self.custom_size.unwrap().y / 5.0)
                    ),
                    Variant::Alliance(icons) => (
                        Text2d::new(icons.to_string()),
                        TextColor::WHITE,
                        TextFont::from_font_size(self.custom_size.unwrap().y / 5.0)
                    ),
                }],
            )
        }
    }
}

mod on_enter {
    use rand::seq::SliceRandom;

    use super::*;

    pub fn spawn_tiles(
        mut commands: Commands,
        projection: Single<&Projection, With<Camera>>,
        asset_server: Res<AssetServer>,
    ) {
        let Projection::Orthographic(projection) = *projection else {
            panic!();
        };

        // Load assets
        let texture_tile: Handle<Image> = asset_server.load("misc/rev2/Tile_897x1237.png");
        let texture_alliance: Handle<Image> = asset_server.load("misc/rev2/Alliance_1104x882.png");
        let texture_horde: Handle<Image> = asset_server.load("misc/rev2/Horde_740x1093.png");

        // Determine size and position(s) for tiles
        let rows = PositionGenerator::<Turtle>::ROWS as f32;
        let cols = PositionGenerator::<Turtle>::COLUMNS;

        let tile_height = projection.area.height() as f32 / rows;
        let tile_width = tile_height * 0.7;
        let tile_factory = tile::Factory::new(
            texture_tile,
            texture_alliance,
            texture_horde,
            Some(Vec2::new(tile_width, tile_height)),
        );

        let mut rng = rand::rng();

        let mut tile_positions: Vec<Vec3> =
            PositionGenerator::<Turtle>::new(Vec2::new(tile_height, tile_width))
                .into_iter()
                .collect();
        tile_positions.shuffle(&mut rng);
        let tile_positions: Vec<(usize, Vec3)> = tile_positions.into_iter().enumerate().collect();

        let mut tile_pairs: Vec<usize> = (0..PositionGenerator::<Turtle>::TILES
            / PositionGenerator::<Turtle>::TILE_PAIR_SIZE)
            .collect();
        tile_pairs.shuffle(&mut rng);


        // Offsets
        let start_x: f32 = (projection.area.width() / 2.0) - (tile_width * PositionGenerator::<Turtle>::COLUMNS as f32);
        let start_y: f32 = 0.0;
        let start_z: f32 = 0.0;

        // Spawn loop
        let tile_components = (DespawnOnExit(InGame::Root), Pickable::default());
        for ((index, tile_pair), position_pair) in tile_pairs.iter().enumerate().zip(
            tile_positions
                .windows(PositionGenerator::<Turtle>::TILE_PAIR_SIZE)
                .step_by(PositionGenerator::<Turtle>::TILE_PAIR_SIZE),
        ) {
            for i in 0..PositionGenerator::<Turtle>::TILE_PAIR_SIZE {
            commands.spawn((
                tile_components.clone(),
                tile_factory.get_tile(tile::Variant::Alliance(*tile_pair)),
                tile::ID(*tile_pair),
                Transform {
                    translation: Vec3 {
                        x: start_x + position_pair[i].1.x,
                        y: start_y + position_pair[i].1.y,
                        z: start_z + position_pair[i].1.z,
                    },
                    ..default()
                }
            ));

            //     commands
            //         .spawn((
            //             tile_components.clone(),
            //             tile_factory.get_tile(TileVariant::Alliance(*tile_pair)),
            //             ID(*tile_pair),
            //             Transform {
            //                 translation: Vec3 {
            //                     x: start_x + logic_position_pair[i].1.x - (x_index * x_offset)
            //                         + (z_index * x_offset * 2.0),
            //                     y: start_y - logic_position_pair[i].1.y
            //                         + (y_index * y_offset)
            //                         + (z_index * y_offset * 1.0),
            //                     z: start_z + logic_position_pair[i].1.z * 10.0 - x_index + y_index,
            //                 },
            //                 ..default()
            //             },
            //             Position {
            //                 pos: Vec3::new(
            //                     logic_position_pair[i].1.x,
            //                     logic_position_pair[i].1.y,
            //                     logic_position_pair[i].1.z,
            //                 ),
            //             },
            //         ))
            //         .observe(on_click);
            }
        }
    }
}

mod generator {
    use bevy::prelude::{Vec2, Vec3};
    use std::marker::PhantomData;

    pub struct Turtle;

    trait PositionGeneratorTrait {
        fn generate(
            &self,
            current: usize,
        ) -> Option<Vec3>;
    }

    pub struct PositionGenerator<T> {
        tile_size: Vec2,
        _variant: PhantomData<T>,
    }

    impl<T> PositionGenerator<T> {
        pub fn new(tile_size: Vec2) -> Self {
            Self {
                tile_size,
                _variant: PhantomData::<T>,
            }
        }
    }

    impl PositionGenerator<Turtle> {
        pub const TILES: usize = 144;
        pub const TILE_PAIR_SIZE: usize = 4;
        pub const ROWS: usize = 8;
        pub const COLUMNS: usize = 15;
    }

    impl PositionGeneratorTrait for PositionGenerator<Turtle> {
        fn generate(
            &self,
            current: usize,
        ) -> Option<Vec3> {
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
                    layer = 4;
                    return Some(Vec3::new(5.5, 3.5, layer as f32) * self.tile_size.extend(1.0));
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
                            0 => {
                                return Some(
                                    Vec3::new(-1.0, 3.5, layer as f32) * self.tile_size.extend(1.0),
                                );
                            },
                            1 => {
                                return Some(
                                    Vec3::new(12.0, 3.5, layer as f32) * self.tile_size.extend(1.0),
                                );
                            },
                            2 => {
                                return Some(
                                    Vec3::new(13.0, 3.5, layer as f32) * self.tile_size.extend(1.0),
                                );
                            },
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

            Some(Vec3::new(column as f32, row as f32, layer as f32) * self.tile_size.extend(1.0))
        }
    }

    pub struct PositionGeneratorIterator<G: PositionGeneratorTrait> {
        generator: G,
        counter: usize,
    }

    impl<G: PositionGeneratorTrait> PositionGeneratorIterator<G> {
        pub fn new(generator: G) -> Self {
            Self {
                generator,
                counter: 0,
            }
        }
    }

    pub struct PositionGeneratorRefIterator<'a, G: PositionGeneratorTrait> {
        generator: &'a G,
        counter: usize,
    }

    impl<'a, G: PositionGeneratorTrait> PositionGeneratorRefIterator<'a, G> {
        pub fn new(generator: &'a G) -> Self {
            Self {
                generator,
                counter: 0,
            }
        }
    }

    type PositionGeneratorItem = Vec3;

    impl<G: PositionGeneratorTrait> Iterator for PositionGeneratorIterator<G> {
        type Item = PositionGeneratorItem;

        fn next(&mut self) -> Option<Self::Item> {
            self.counter += 1;
            self.generator.generate(self.counter - 1)
        }
    }

    impl<'a, G: PositionGeneratorTrait> Iterator for PositionGeneratorRefIterator<'a, G> {
        type Item = PositionGeneratorItem;

        fn next(&mut self) -> Option<Self::Item> {
            self.counter += 1;
            self.generator.generate(self.counter - 1)
        }
    }

    impl IntoIterator for PositionGenerator<Turtle> {
        type Item = PositionGeneratorItem;
        type IntoIter = PositionGeneratorIterator<PositionGenerator<Turtle>>;

        fn into_iter(self) -> Self::IntoIter {
            PositionGeneratorIterator::new(self)
        }
    }

    impl<'a> IntoIterator for &'a PositionGenerator<Turtle> {
        type Item = PositionGeneratorItem;
        type IntoIter = PositionGeneratorRefIterator<'a, PositionGenerator<Turtle>>;

        fn into_iter(self) -> Self::IntoIter {
            PositionGeneratorRefIterator::new(self)
        }
    }
}
