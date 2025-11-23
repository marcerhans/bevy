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

    #[derive(Component, Clone)]
    pub struct Position {
        pub val: Vec3,
    }

    pub struct Factory {
        texture_tile: Handle<Image>,
        texture_alliance: Handle<Image>,
        texture_horde: Handle<Image>,
        custom_size: Option<Vec2>,
        tile_center_offset: Option<Vec3>,
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
            tile_center_offset: Option<Vec2>,
        ) -> Self {
            let tile_center_offset = if let Some(tile_center_offset) = tile_center_offset {
                Some(tile_center_offset.extend(0.0))
            } else {
                None
            };

            Self {
                texture_tile,
                texture_alliance,
                texture_horde,
                custom_size,
                tile_center_offset,
            }
        }

        pub fn get_tile(
            &self,
            variant: Variant,
            tint: Option<Color>,
        ) -> impl Bundle {
            let offset = Transform {
                translation: Vec3 {
                    x: if self.tile_center_offset.is_none() {
                        0.0
                    } else {
                        self.tile_center_offset.unwrap().x
                    },
                    y: if self.tile_center_offset.is_none() {
                        0.0
                    } else {
                        self.tile_center_offset.unwrap().y
                    },
                    z: 0.1,
                },
                ..default()
            };

            (
                Marker,
                Sprite {
                    custom_size: self.custom_size,
                    color: if tint.is_none() {
                        Color::default()
                    } else {
                        tint.unwrap()
                    },
                    ..Sprite::from_image(self.texture_tile.clone())
                },
                children![match variant {
                    Variant::Horde(icons) => (
                        // Text2d::new(icons.to_string()),
                        // TextColor::WHITE,
                        // TextFont::from_font_size(self.custom_size.unwrap().y / 5.0),
                        // TextBackgroundColor::BLACK,
                        Sprite::from_color(Color::BLACK, Vec2::new(50.0, 50.0)),
                        offset,
                    ),
                    Variant::Alliance(icons) => (
                        // Text2d::new(icons.to_string()),
                        // TextColor::WHITE,
                        // TextFont::from_font_size(self.custom_size.unwrap().y / 5.0),
                        // TextBackgroundColor::BLACK,
                        Sprite::from_color(Color::BLACK, Vec2::new(50.0, 50.0)),
                        offset,
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

        // Load assets and set texture constants
        let texture_tile: Handle<Image> = asset_server.load("misc/rev2/Tile3_700x1000.png");
        let texture_alliance: Handle<Image> = asset_server.load("misc/rev2/Alliance_1104x882.png");
        let texture_horde: Handle<Image> = asset_server.load("misc/rev2/Horde_740x1093.png");
        const TEXTURE_BOTTOM_BORDER_PERCENTAGE_Y: f32 = 260.0 / 1000.0; // (Just "ONE" border)
        const TEXTURE_LEFT_BORDER_PERCENTAGE_X: f32 = 200.0 / 700.0; // (Juse "ONE" border)
        const TEXTURE_BOTTOM_BORDER_PERCENTAGE2_Y: f32 = 175.0 / 1000.0; // (Just the "thickness" of the tile, excluding the border)
        const TEXTURE_LEFT_BORDER_PERCENTAGE2_X: f32 = 124.0 / 700.0; // (Just the "thickness" of the tile, excluding the border)

        // Determine size and position(s) for tiles
        let tile_height =
            projection.area.height() as f32 / PositionGenerator::<Turtle>::ROWS as f32;
        let tile_height = tile_height + TEXTURE_BOTTOM_BORDER_PERCENTAGE2_Y * tile_height; // NOTE:1: This is hacky, but allows for tile height to fill height of window.
        let tile_width = tile_height * 0.7;
        let tile_size = Vec2::new(tile_width, tile_height);
        let tile_center_offset = Vec2::new(
            TEXTURE_LEFT_BORDER_PERCENTAGE2_X,
            TEXTURE_BOTTOM_BORDER_PERCENTAGE2_Y,
        ) * tile_size
            / 2.0;
        let tile_thickness_offset = Vec2::new(
            TEXTURE_LEFT_BORDER_PERCENTAGE2_X,
            TEXTURE_BOTTOM_BORDER_PERCENTAGE2_Y,
        ) * tile_size;
        let tile_factory = tile::Factory::new(
            texture_tile.clone(),
            texture_alliance,
            texture_horde,
            Some(tile_size),
            Some(tile_center_offset),
        );

        let mut rng = rand::rng();
        let mut tile_positions: Vec<Vec3> =
            PositionGenerator::<Turtle>::new(tile_size, projection.area)
                .into_iter()
                .collect();
        tile_positions.shuffle(&mut rng);

        // Spawn loop
        let tile_components = (DespawnOnExit(InGame::Root), Pickable::default());
        let tvs = PositionGenerator::<Turtle>::TILE_VARIANT_SIZE;

        for (index, tile_position) in tile_positions.windows(tvs).step_by(tvs).enumerate() {
            for variant_index in 0..tvs {
                let column_index = tile_position[variant_index].x / tile_size.x;
                let row_index = tile_position[variant_index].y / tile_size.y;

                commands
                    .spawn((
                        tile_components.clone(),
                        tile_factory.get_tile(tile::Variant::Horde(index), None),
                        tile::Position {
                            val: Vec3 {
                                x: tile_position[variant_index].x,
                                y: tile_position[variant_index].y,
                                z: tile_position[variant_index].z,
                            },
                        },
                        Transform {
                            // RealPosition(x,y) + Adjustments for "overlaps" + Adustments for layer offsets
                            // RealPosition(z) * 100.0 - Adjustments for row and column (such that overlaps are correct)
                            translation: Vec3 {
                                x: tile_position[variant_index].x
                                    - (column_index * tile_thickness_offset.x)
                                    + (tile_position[variant_index].z * tile_thickness_offset.x),
                                y: tile_position[variant_index].y
                                    - (row_index * tile_thickness_offset.y)
                                    + (tile_position[variant_index].z * tile_thickness_offset.y)
                                    - tile_height * 0.5, // NOTE:2: See NOTE:1 - This is simply to adjust the offset
                                z: tile_position[variant_index].z * 100.0
                                    - column_index
                                    - row_index as f32,
                            },
                            ..default()
                        },
                    ))
                    .with_child((
                        Sprite {
                            custom_size: Some(tile_size.clone()),
                            color: Color::hsla(0.0, 0.0, 0.0, 0.8),
                            ..Sprite::from_image(texture_tile.clone())
                        },
                        Transform {
                            translation: Vec3 {
                                x: -tile_thickness_offset.x * 1.0,
                                y: -tile_thickness_offset.y * 0.8,
                                z: -1.0,
                            },
                            ..default()
                        },
                    ))
                    .observe(|mut event: On<Pointer<Click>>| {
                        dbg!(event);
                    });
            }
        }
    }
}

mod generator {
    use bevy::prelude::*;
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
        projection_area: Rect,
        _variant: PhantomData<T>,
    }

    impl<T> PositionGenerator<T> {
        pub fn new(
            tile_size: Vec2,
            projection_area: Rect,
        ) -> Self {
            Self {
                tile_size,
                projection_area,
                _variant: PhantomData::<T>,
            }
        }
    }

    impl PositionGenerator<Turtle> {
        pub const TILES: usize = 144;
        pub const TILE_VARIANT_SIZE: usize = 4;
        pub const ROWS: usize = 8;
        pub const COLUMNS: usize = 15;
        pub const LAYERS: usize = 5;

        fn local2global(
            &self,
            local_position: &Vec3,
        ) -> Vec3 {
            Vec3::new(
                local_position.x - (Self::COLUMNS as f32 * self.tile_size.x) / 2.0
                    + 0.5 * self.tile_size.x,
                local_position.y - self.projection_area.height() / 2.0,
                local_position.z,
            )
        }
    }

    impl PositionGeneratorTrait for PositionGenerator<Turtle> {
        fn generate(
            &self,
            current: usize,
        ) -> Option<Vec3> {
            if current >= Self::TILES {
                return None;
            }

            let offsets = Vec3::new(1.5, 0.5, 0.0) * self.tile_size.extend(1.0);

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
                    let row = 3.5;
                    let column = 5.5;
                    let layer = 4.0;
                    let local_position = Vec3::new(column, row, layer) * self.tile_size.extend(1.0);
                    return Some(self.local2global(&local_position) + offsets);
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
                                let row = 3.5;
                                let column = -1.0;
                                let local_position = Vec3::new(column, row, layer as f32)
                                    * self.tile_size.extend(1.0);
                                return Some(self.local2global(&local_position) + offsets);
                            },
                            1 => {
                                let row = 3.5;
                                let column = 12.0;
                                let local_position = Vec3::new(column, row, layer as f32)
                                    * self.tile_size.extend(1.0);
                                return Some(self.local2global(&local_position) + offsets);
                            },
                            2 => {
                                let row = 3.5;
                                let column = 13.0;
                                let local_position = Vec3::new(column, row, layer as f32)
                                    * self.tile_size.extend(1.0);
                                return Some(self.local2global(&local_position) + offsets);
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

            let local_position =
                Vec3::new(column as f32, row as f32, layer as f32) * self.tile_size.extend(1.0);
            Some(self.local2global(&local_position) + offsets)
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
