use crate::plugin::scene::main_menu::MainMenu;
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<InGame>()
            .insert_resource(SelectedTile::default())
            .add_systems(
                OnEnter(InGame::Root),
                (spawn_background, spawn_tiles, spawn_buttons),
            )
            .add_systems(Update, resize_background.run_if(in_state(InGame::Root)));
    }
}

fn spawn<'a>(
    commands: &'a mut Commands,
    bundle: impl Bundle,
) -> EntityCommands<'a> {
    let mut ec = commands.spawn(bundle);
    ec.insert((DespawnOnExit(InGame::Root), Pickable::default()));
    ec
}

#[derive(SubStates, Default, Debug, Hash, Eq, PartialEq, Clone)]
#[source(MainMenu = MainMenu::Play)]
#[states(scoped_entities)]
pub enum InGame {
    #[default]
    Root,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct SelectedTile(Option<Entity>);

mod marker {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Background;

    #[derive(Component)]
    pub struct Button;
}

mod tile {
    use std::marker::PhantomData;

    use bevy::prelude::*;

    pub mod asset {
        pub mod texture {
            pub const TILE: &'static str = "misc/rev2/Tile.png";
            pub const ALLIANCE: &'static str = "misc/rev2/Alliance.png";
            pub const HORDE: &'static str = "misc/rev2/Horde.png";
            pub const BLADE: &'static str = "misc/rev2/Frostmourne.png";
            pub const HEARTHSTONE: &'static str = "misc/rev2/Hearthstone.png";

            pub const TILE_WIDTH: u32 = 962;
            pub const TILE_HEIGHT: u32 = 1238;
            pub const TILE_NO_BORDER_WIDTH: u32 = 872;
            pub const TILE_NO_BORDER_HEIGHT: u32 = 1149;
            pub const TILE_BORDER_LENGTH: u32 = 110;
        }
    }

    #[derive(Bundle)]
    pub struct Tile {
        pub marker: Marker<0>,
        pub position: Position,
        pub variant: Variant,
    }

    /// "DEPTH" implies on which "Child" level the marker is at.
    #[derive(Component)]
    pub struct Marker<const DEPTH: u32>;

    #[derive(Component, Deref, DerefMut, Clone, Copy)]
    pub struct Position(UVec3);

    pub struct Turtle;

    pub struct PositionGenerator<T> {
        counter: u32,
        tile_grid_size: UVec2,
        _type: PhantomData<T>,
    }

    impl<T> PositionGenerator<T> {
        pub fn new(tile_grid_size: UVec2) -> Self {
            Self {
                counter: 0,
                tile_grid_size,
                _type: PhantomData,
            }
        }
    }

    impl PositionGenerator<Turtle> {
        pub const TILES: usize = 144;
        pub const TILE_VARIANT_SIZE: usize = 4;
        pub const ROWS: usize = 8;
        pub const COLUMNS: usize = 15;
        pub const LAYERS: usize = 5;
        pub const TILE_GRID_SIZE: usize = 2;
    }

    impl Iterator for PositionGenerator<Turtle> {
        type Item = Position;

        fn next(&mut self) -> Option<Self::Item> {
            const TILES_INDEX_MAX: u32 = PositionGenerator::<Turtle>::TILES as u32 - 1;
            let layer;
            let row;

            match self.counter {
                ..87 => {
                    layer = 0;
                    match self.counter {
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
                    row = (self.counter - 87) / 6 + 1;
                },
                123..139 => {
                    layer = 2;
                    row = (self.counter - 123) / 4 + 2;
                },
                139..143 => {
                    layer = 3;
                    row = (self.counter - 139) / 2 + 3;
                },
                TILES_INDEX_MAX => {
                    // Special case. Just return value immediately.
                    let row = 3.5 * self.tile_grid_size.y as f32;
                    let column = 6.5 * self.tile_grid_size.x as f32;
                    let layer = 4.0;
                    let local_position =
                        Position(UVec3::new(column as u32, row as u32, layer as u32));
                    self.counter += 1;
                    return Some(local_position);
                },
                _ => return None,
            }

            let column = match layer {
                0 => {
                    match row {
                        0 => 1 + self.counter - 0,
                        1 => 3 + self.counter - 12,
                        2 => 2 + self.counter - 20,
                        3 => 1 + self.counter - 30,
                        4 => 1 + self.counter - 42,
                        5 => 2 + self.counter - 54,
                        6 => 3 + self.counter - 64,
                        7 => 1 + self.counter - 72,
                        8 => match self.counter - 84 {
                            // Last 3 are special cases. Do not follow a pattern.
                            0 => {
                                let row = 3.5 * self.tile_grid_size.y as f32;
                                let column = 0.0 * self.tile_grid_size.x as f32;
                                let local_position =
                                    Position(UVec3::new(column as u32, row as u32, layer as u32));
                                self.counter += 1;
                                return Some(local_position);
                            },
                            1 => {
                                let row = 3.5 * self.tile_grid_size.y as f32;
                                let column = 13.0 * self.tile_grid_size.x as f32;
                                let local_position =
                                    Position(UVec3::new(column as u32, row as u32, layer as u32));
                                self.counter += 1;
                                return Some(local_position);
                            },
                            2 => {
                                let row = 3.5 * self.tile_grid_size.y as f32;
                                let column = 14.0 * self.tile_grid_size.x as f32;
                                let local_position =
                                    Position(UVec3::new(column as u32, row as u32, layer as u32));
                                self.counter += 1;
                                return Some(local_position);
                            },
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                },
                1 => 4 + ((self.counter - 87) % 6),
                2 => 5 + ((self.counter - 123) % 4),
                3 => 6 + ((self.counter - 139) % 2),
                _ => unreachable!(),
            };

            let row = row * self.tile_grid_size.y;
            let column = column * self.tile_grid_size.x;
            let local_position = Position(UVec3::new(column as u32, row as u32, layer as u32));
            self.counter += 1;
            return Some(local_position);
        }
    }

    #[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq, Eq)]
    pub struct Variant(pub u32);

    impl Variant {
        pub fn insert_sprite_as_child(
            asset_server: &Res<AssetServer>,
            entity_commands: &mut EntityCommands,
            variant: u32,
            max_size: &Vec2,
        ) {
            const MAX_VARIANTS: u32 = (PositionGenerator::<Turtle>::TILES
                / PositionGenerator::<Turtle>::TILE_VARIANT_SIZE)
                as u32;
            const MAX_VARIANTS_HALF: u32 = MAX_VARIANTS / 2;
            const TVR: u32 = PositionGenerator::<Turtle>::TILE_VARIANT_SIZE as u32;
            let index = variant / TVR;
            let large = max_size * 0.8;
            let medium = large * 0.5;
            let small = large * 0.40;
            let small2 = large * 0.25;

            let alliance: Handle<Image> = asset_server.load(asset::texture::ALLIANCE);
            let horde: Handle<Image> = asset_server.load(asset::texture::HORDE);
            let blades: Handle<Image> = asset_server.load(asset::texture::BLADE);
            let hs: Handle<Image> = asset_server.load(asset::texture::HEARTHSTONE);

            let common = (
                Transform::default().with_translation(Vec3::default().with_z(0.1)),
                Visibility::Inherited,
            );

            fn template(
                x: f32,
                y: f32,
                size: Vec2,
                image: Handle<Image>,
                shading: Option<Color>,
            ) -> impl Bundle {
                (
                    Transform {
                        translation: Vec3 { x, y, ..default() },
                        ..default()
                    },
                    Sprite {
                        custom_size: Some(size),
                        color: shading.unwrap_or_default(),
                        ..Sprite::from_image(image)
                    },
                )
            }

            match index {
                0 | 1 | 2 | 3 => {
                    let image = match index {
                        0 => alliance,
                        1 => horde,
                        2 => blades,
                        3 => hs,
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![template(0.0, 0.0, large.clone(), image.clone(), None),],
                    ));
                },
                4 | 5 | 6 | 7 => {
                    let (image, size) = match index {
                        4 => (alliance, medium),
                        5 => (horde, medium),
                        6 => (blades, medium),
                        7 => (hs, medium),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 8.0,
                                max_size.y / 8.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 8.0,
                                -max_size.y / 8.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                8 | 9 | 10 | 11 => {
                    let (image, size) = match index {
                        8 => (alliance, small),
                        9 => (horde, small),
                        10 => (blades, medium),
                        11 => (hs, medium),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0,
                                max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, 0.0, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0,
                                -max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                12 | 13 | 14 | 15 => {
                    let (image, size) = match index {
                        12 => (alliance, small2),
                        13 => (horde, small2),
                        14 => (blades, small),
                        15 => (hs, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 7.0,
                                -max_size.y / 7.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 7.0,
                                max_size.y / 7.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 7.0,
                                -max_size.y / 7.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 7.0,
                                max_size.y / 7.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                16 | 17 | 18 | 19 => {
                    let (image, color, size) = match index {
                        16 => (alliance, Color::hsl(0.0, 0.0, 1.0), small),
                        17 => (horde, Color::hsl(240.0, 1.0, 0.5), small),
                        18 => (blades, Color::BLACK, small),
                        19 => (hs, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0,
                                -max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, 0.0, size.clone(), image.clone(), Some(color),),
                            template(
                                max_size.x / 5.0,
                                -max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 5.0,
                                max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                20 | 21 | 22 | 23 => {
                    let (image, color, size) = match index {
                        20 => (alliance, Color::hsl(0.0, 1.0, 0.5), small),
                        21 => (horde, Color::hsl(240.0, 1.0, 0.5), small),
                        22 => (blades, Color::BLACK, small),
                        23 => (hs, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 6.0,
                                -max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(-max_size.x / 6.0, 0.0, size.clone(), image.clone(), None,),
                            template(
                                -max_size.x / 6.0,
                                max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 6.0,
                                -max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(max_size.x / 6.0, 0.0, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 6.0,
                                max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                24 | 25 | 26 | 27 => {
                    let (image, color, size) = match index {
                        24 => (alliance, Color::hsl(0.0, 1.0, 0.5), small),
                        25 => (horde, Color::hsl(240.0, 1.0, 0.5), small),
                        26 => (blades, Color::BLACK, small),
                        27 => (hs, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0,
                                -max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, -max_size.y / 5.0, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0,
                                -max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(-max_size.x / 5.0, 0.0, size.clone(), image.clone(), None,),
                            template(0.0, 0.0, size.clone(), image.clone(), None,),
                            template(max_size.x / 5.0, 0.0, size.clone(), image.clone(), None,),
                            template(
                                0.0,
                                max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                Some(color),
                            ),
                        ],
                    ));
                },
                28 | 29 | 30 | 31 => {
                    let (image, color, size) = match index {
                        28 => (alliance, Color::hsl(0.0, 1.0, 0.5), small),
                        29 => (horde, Color::hsl(240.0, 1.0, 0.5), small),
                        30 => (blades, Color::BLACK, small),
                        31 => (hs, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0,
                                max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, max_size.y / 5.0, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0,
                                max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(-max_size.x / 5.0, 0.0, size.clone(), image.clone(), None,),
                            template(0.0, 0.0, size.clone(), image.clone(), None,),
                            template(max_size.x / 5.0, 0.0, size.clone(), image.clone(), None,),
                            template(
                                -max_size.x / 5.0,
                                -max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                Some(color),
                            ),
                            template(
                                max_size.x / 5.0,
                                -max_size.y / 5.0,
                                size.clone(),
                                image.clone(),
                                Some(color),
                            ),
                        ],
                    ));
                },
                //     1 => {
                //         size = &medium;
                //         positions.append(&mut vec![
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 5.0,
                //                     y: max_size.y / 5.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 5.0,
                //                     y: -max_size.y / 5.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //         ]);
                //     },
                //     2 => {
                //         size = &small;
                //         positions.append(&mut vec![
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     y: max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (Vec3 { ..default() }, ImageType::Regular),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     y: -max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //         ]);
                //     },
                //     3 => {
                //         size = &small;
                //         positions.append(&mut vec![
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 5.0,
                //                     y: -max_size.y / 5.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 5.0,
                //                     y: -max_size.y / 5.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 5.0,
                //                     y: max_size.y / 5.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 5.0,
                //                     y: max_size.y / 5.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //         ]);
                //     },
                //     4 => {
                //         size = &small;
                //         positions.append(&mut vec![
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 5.0,
                //                     y: -max_size.y / 5.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 5.0,
                //                     y: -max_size.y / 5.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 5.0,
                //                     y: max_size.y / 5.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 5.0,
                //                     y: max_size.y / 5.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (Vec3 { ..default() }, ImageType::Regular),
                //         ]);
                //     },
                //     5 => {
                //         size = &small;
                //         positions.append(&mut vec![
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     y: -max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     y: max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     y: -max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     y: max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //         ]);
                //     },
                //     6 => {
                //         size = &small;
                //         positions.append(&mut vec![
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     y: -max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     y: max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     y: -max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     y: max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (Vec3 { ..default() }, ImageType::Regular),
                //         ]);
                //     },
                //     7 => {
                //         size = &small;
                //         positions.append(&mut vec![
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     y: -max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     y: max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     y: -max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     y: max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     y: -max_size.y / 7.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     y: max_size.y / 7.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //         ]);
                //     },
                //     8 => {
                //         size = &small;
                //         positions.append(&mut vec![
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     y: -max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: -max_size.x / 4.0,
                //                     y: max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     y: -max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     x: max_size.x / 4.0,
                //                     y: max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (
                //                 Vec3 {
                //                     y: -max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //             (Vec3 { ..default() }, ImageType::Regular),
                //             (
                //                 Vec3 {
                //                     y: max_size.y / 4.0,
                //                     ..default()
                //                 },
                //                 ImageType::Regular,
                //             ),
                //         ]);
                //     },
                //     9 => (),
                //     10 => (),
                //     11 => (),
                //     12 => (),
                //     13 => (),
                //     14 => (),
                //     15 => (),
                //     16 => (),
                //     17 => (),
                _ => warn!("Unsupported variant!"),
            };

            // entity_commands.with_children(|parent| {
            //     parent.spawn(common).with_children(|common| {
            //         for (position, image_type) in positions {
            //             let image = match image_type {
            //                 ImageType::Regular => images.0.clone(),
            //                 ImageType::Button => images.1.clone(),
            //             };
            //             common.spawn((
            //                 Transform {
            //                     translation: position,
            //                     ..default()
            //                 },
            //                 Sprite {
            //                     custom_size: Some(size.clone()),
            //                     ..Sprite::from_image(image.clone())
            //                 },
            //             ));
            //         }
            //     });
            // });
        }
    }
}

pub fn spawn_background(
    mut commands: Commands,
    projection: Query<&Projection, With<Camera>>,
    asset_server: Res<AssetServer>,
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let handle: Handle<Image> = asset_server.load("misc/rev2/original/Arthas_LichKing_GPT2.png");

    spawn(
        &mut commands,
        (
            marker::Background,
            Sprite {
                custom_size: Some(Vec2::new(projection.area.width(), projection.area.height())),
                color: Color::srgb(1.0, 1.0, 1.0).with_luminance(0.3),
                ..Sprite::from_image(handle)
            },
            Transform {
                translation: Vec3 {
                    z: -10.0,
                    ..default()
                },
                ..default()
            },
        ),
    );
}

pub fn spawn_tiles(
    mut commands: Commands,
    projection: Query<&Projection, With<Camera>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let tile_texture: Handle<Image> = asset_server.load(tile::asset::texture::TILE);
    let tile_size = Vec2::new(
        (projection.area.height() / tile::PositionGenerator::<tile::Turtle>::ROWS as f32) * 0.7,
        projection.area.height() / tile::PositionGenerator::<tile::Turtle>::ROWS as f32,
    );
    let tile_grid_size = tile::PositionGenerator::<tile::Turtle>::TILE_GRID_SIZE as u32;
    let position_generator =
        tile::PositionGenerator::<tile::Turtle>::new(UVec2::splat(tile_grid_size));
    let tile_size_full = Vec2::new(
        (tile_size.x / tile::asset::texture::TILE_NO_BORDER_WIDTH as f32)
            * tile::asset::texture::TILE_WIDTH as f32,
        (tile_size.y / tile::asset::texture::TILE_NO_BORDER_HEIGHT as f32)
            * tile::asset::texture::TILE_HEIGHT as f32,
    );
    let tile_size_ratio = tile_size_full / tile_size;

    let tile_pos_offset = Vec3::new(
        -(tile_size_full.x * tile::PositionGenerator::<tile::Turtle>::COLUMNS as f32 / 2.0)
            + tile_size_full.x * 1.0,
        -projection.area.height() / 2.0 + tile_size_full.y * 0.5,
        0.0,
    );

    info!("tile_size: {:?}", tile_size);
    info!(
        "tile_size_ratio * tile_size_full: {:?}",
        tile_size_ratio * tile_size_full
    );

    for (variant, pos) in position_generator.enumerate() {
        let index = variant; // TODO: Have to enumerate them!!!
        let layer = pos.z;
        let order_offset_factor = Vec3::default().with_z(0.1);
        let layer_offset_factor = Vec3::default().with_z(10.0);

        let variant = variant as u32;
        let mut entity_commands = spawn(
            &mut commands,
            (
                tile::Tile {
                    marker: tile::Marker::<0>,
                    position: pos,
                    variant: tile::Variant(variant),
                },
                Sprite {
                    custom_size: Some(tile_size),
                    ..Sprite::from_image(tile_texture.clone())
                },
                Transform {
                    translation: (((pos.as_vec3() / tile_grid_size as f32)
                        * tile_size_full.extend(1.0))
                        + tile_pos_offset)
                        - (order_offset_factor * index as f32)
                        + (layer_offset_factor * layer as f32),
                    ..default()
                },
            ),
        );
        entity_commands
            // .with_child((
            // Sprite {
            //     custom_size: Some(tile_size_full),
            //     ..Sprite::from_image(tile_texture.clone())
            // },
            //     Transform {
            //         translation: Vec3 {
            //             x: -tile_size_ratio.x / tile::asset::texture::TILE_WIDTH as f32,
            //             y: -tile_size_ratio.y / tile::asset::texture::TILE_HEIGHT as f32,
            //             ..default()
            //         },
            //         ..default()
            //     },
            // ))
            .observe(tile_pressed);
        // tile::Variant::insert_sprite_as_child(
        //     &asset_server,
        //     &mut entity_commands,
        //     variant,
        //     &tile_size,
        // );

        if variant > 95 {
            break;
        }
    }
}

pub fn tile_pressed(
    on_press: On<Pointer<Press>>,
    mut commands: Commands,
    mut tiles: Query<(Entity, &tile::Variant, &tile::Position, &mut Sprite), With<tile::Marker<0>>>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    let (pressed_entity, pressed_variant, pressed_position, mut pressed_sprite) = tiles
        .iter_mut()
        .find(|tile| tile.0 == on_press.entity)
        .unwrap();

    let Some(selected_entity) = selected_tile.0.take() else {
        selected_tile.0 = Some(pressed_entity);
        // pressed_sprite.color = Color::hsl(0.5, 1.0, 1.5); // TODO!
        return;
    };

    let (selected_entity, selected_variant, selected_position, mut selected_sprite) =
        tiles.get_mut(selected_entity).unwrap();
    // selected_sprite.color = Color::hsl(1.0, 1.0, 1.0); // TODO!

    // selected_color =

    // if pressed_entity == selected_entity {
    //     info!("Tile cannot be matched against itself!");
    //     return;
    // }
}

pub fn valid_removal() -> bool {
    false
}

pub fn spawn_buttons(
    mut commands: Commands,
    projection: Query<&Projection, With<Camera>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };
}

fn resize_background(
    mut transform: Query<(&mut Transform, &mut Sprite), With<marker::Background>>,
    projection: Query<&Projection, With<Camera>>,
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let Some((_, mut sprite)) = transform.iter_mut().next() else {
        panic!();
    };

    if sprite.custom_size.unwrap().x != projection.area.width()
        || sprite.custom_size.unwrap().y != projection.area.height()
    {
        sprite.custom_size = Some(Vec2 {
            x: projection.area.width(),
            y: projection.area.height(),
        });
    }
}
