use crate::plugin::scene::main_menu::MainMenu;
use bevy::{
    gizmos::grid,
    input::keyboard::KeyCode,
    platform::collections::HashMap,
    prelude::*,
    sprite::{Anchor, Text2dShadow},
};
use rand::{
    Rng, SeedableRng,
    rngs::StdRng,
    seq::{IndexedRandom, IteratorRandom, SliceRandom},
};
use std::collections::{HashSet, VecDeque};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<InGame>()
            .add_message::<HelpMsg>()
            .insert_resource(SelectedTile::default())
            .insert_resource(History::default())
            .insert_resource(HelpEnabled::default())
            .add_systems(
                OnEnter(InGame::Root),
                (spawn_background, spawn_tiles, spawn_buttons),
            )
            .add_systems(Update, resize_background.run_if(in_state(InGame::Root)))
            .add_systems(
                Update,
                (
                    undo_keyboard,
                    redo_keyboard,
                    help_keyboard,
                    help_toggle,
                    help,
                )
                    .run_if(in_state(InGame::Root)),
            );
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

#[derive(Resource, Default, Deref, DerefMut, PartialEq, Eq)]
pub struct HelpEnabled(bool);

#[derive(Message)]
pub struct HelpMsg;

#[derive(Clone)]
pub enum HistoryItem {
    ValidPair(Entity, Entity),
    Shuffle(Vec<(Entity, tile::Variant)>),
}

#[derive(Resource, Default)]
pub struct History {
    undo: VecDeque<HistoryItem>,
    redo: VecDeque<HistoryItem>,
}

impl History {
    const MAX: usize = 32;

    pub fn push_front(
        &mut self,
        item: HistoryItem,
    ) {
        if self.undo.len() >= Self::MAX {
            self.undo.pop_back();
        }
        self.undo.push_front(item);
        self.redo.clear();
    }

    pub fn pop_front(&mut self) -> Option<HistoryItem> {
        let item = self.undo.pop_front();

        if let Some(item) = item.clone() {
            self.redo.push_front(item);
        }

        item
    }

    pub fn push_front_redo(
        &mut self,
        item: HistoryItem,
    ) {
        self.undo.push_front(item);
    }

    pub fn pop_front_redo(&mut self) -> Option<HistoryItem> {
        self.redo.pop_front()
    }
}

mod marker {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Background;

    #[derive(Component)]
    pub struct Hidden;
}

mod tile {
    use bevy::prelude::*;
    use std::marker::PhantomData;

    pub mod asset {
        pub mod texture {
            pub const TILE: &'static str = "misc/rev2/Tile2.png";
            pub const ALLIANCE: &'static str = "misc/rev2/Alliance.png";
            pub const HORDE: &'static str = "misc/rev2/Horde.png";
            pub const FROSTMOURNE: &'static str = "misc/rev2/Frostmourne.png";
            pub const ASHBRINGER: &'static str = "misc/rev2/Ashbringer.png";

            pub const TILE_WIDTH: u32 = 962;
            pub const TILE_HEIGHT: u32 = 1238;
            pub const TILE_NO_BORDER_WIDTH: u32 = 872;
            pub const TILE_NO_BORDER_HEIGHT: u32 = 1149;
            pub const TILE_BORDER_LENGTH: u32 = 90;
        }
    }

    pub const DEFAULT_COLOR: Color = Color::Hsla(Hsla {
        hue: 0.0,
        saturation: 0.0,
        lightness: 1.1,
        alpha: 1.0,
    });

    #[derive(Bundle)]
    pub struct Tile {
        pub marker: Marker<0>,
        pub position: Position,
        pub variant: Variant,
    }

    /// "DEPTH" implies on which "Child" level the marker is at.
    #[derive(Component)]
    pub struct Marker<const DEPTH: u32>;

    #[derive(Component, Deref, DerefMut, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Position(pub UVec3);

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
        pub const TILE_VARIANT_GROUP_SIZE: usize = 4;
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

    #[derive(Component, Deref, DerefMut, Clone, Copy, Eq)]
    pub struct Variant(pub u32);

    /// TODO: This is a bit hacky...
    impl PartialEq for Variant {
        fn eq(
            &self,
            other: &Self,
        ) -> bool {
            self.0 / PositionGenerator::<Turtle>::TILE_VARIANT_GROUP_SIZE as u32
                == other.0 / PositionGenerator::<Turtle>::TILE_VARIANT_GROUP_SIZE as u32
        }
    }

    impl Variant {
        pub fn insert_sprite_as_child(
            asset_server: &Res<AssetServer>,
            entity_commands: &mut EntityCommands,
            variant: u32,
            max_size: &Vec2,
            offset: &Vec3,
        ) {
            const TVR: u32 = PositionGenerator::<Turtle>::TILE_VARIANT_GROUP_SIZE as u32;
            let index = variant / TVR;
            let large = max_size * 0.8;
            let medium = large * 0.5;
            let medium2 = large * 0.7;
            let small = large * 0.40;

            let alliance: Handle<Image> = asset_server.load(asset::texture::ALLIANCE);
            let horde: Handle<Image> = asset_server.load(asset::texture::HORDE);
            let frostmourne: Handle<Image> = asset_server.load(asset::texture::FROSTMOURNE);
            let ashbringer: Handle<Image> = asset_server.load(asset::texture::ASHBRINGER);

            let common = (
                Transform::default().with_translation(Vec3::default().with_z(0.1) + offset),
                Visibility::Inherited,
            );

            fn template(
                x: f32,
                y: f32,
                z: f32,
                size: Vec2,
                image: Handle<Image>,
                shading: Option<Color>,
            ) -> impl Bundle {
                (
                    Transform {
                        translation: Vec3 { x, y, z },
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
                        2 => frostmourne,
                        3 => ashbringer,
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![template(0.0, 0.0, 0.0, large.clone(), image.clone(), None),],
                    ));
                },
                4 | 5 | 6 | 7 => {
                    let (image, size, inverted) = match index {
                        4 => (alliance, medium, 1.0),
                        5 => (horde, medium, 1.0),
                        6 => (frostmourne, medium2, 1.0),
                        7 => (ashbringer, medium2, -1.0),
                        _ => unreachable!(),
                    };

                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 8.0 * inverted,
                                max_size.y / 8.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 8.0 * inverted,
                                -max_size.y / 8.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                8 | 9 | 10 | 11 => {
                    let (image, size, inverted) = match index {
                        8 => (alliance, small, 1.0),
                        9 => (horde, small, 1.0),
                        10 => (frostmourne, medium, 1.0),
                        11 => (ashbringer, medium, -1.0),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0 * inverted,
                                max_size.y / 5.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, 0.0, 0.1, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0 * inverted,
                                -max_size.y / 5.0,
                                0.3,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                12 | 13 | 14 | 15 => {
                    let (image, size) = match index {
                        12 => (alliance, small),
                        13 => (horde, small),
                        14 => (frostmourne, medium),
                        15 => (ashbringer, medium),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 7.0,
                                -max_size.y / 7.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 7.0,
                                max_size.y / 7.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 7.0,
                                -max_size.y / 7.0,
                                0.2,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 7.0,
                                max_size.y / 7.0,
                                0.3,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                16 | 17 | 18 | 19 => {
                    let (image, color, size) = match index {
                        16 => (alliance, Color::BLACK, small),
                        17 => (horde, Color::BLACK, small),
                        18 => (frostmourne, Color::BLACK, small),
                        19 => (ashbringer, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                max_size.y / 5.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, 0.0, 0.2, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.3,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 5.0,
                                max_size.y / 5.0,
                                0.4,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                20 | 21 | 22 | 23 => {
                    let (image, color, size) = match index {
                        20 => (alliance, Color::BLACK, small),
                        21 => (horde, Color::BLACK, small),
                        22 => (frostmourne, Color::BLACK, small),
                        23 => (ashbringer, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 6.0,
                                -max_size.y / 5.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 6.0,
                                0.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 6.0,
                                max_size.y / 5.0,
                                0.2,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 6.0,
                                -max_size.y / 5.0,
                                0.3,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 6.0,
                                0.0,
                                0.4,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 6.0,
                                max_size.y / 5.0,
                                0.5,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                24 | 25 | 26 | 27 => {
                    let (image, color, size) = match index {
                        24 => (alliance, Color::BLACK, small),
                        25 => (horde, Color::BLACK, small),
                        26 => (frostmourne, Color::BLACK, small),
                        27 => (ashbringer, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                0.0,
                                -max_size.y / 5.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.2,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                0.0,
                                0.3,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, 0.0, 0.4, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0,
                                0.0,
                                0.5,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                0.0,
                                max_size.y / 5.0,
                                0.6,
                                size.clone(),
                                image.clone(),
                                Some(color),
                            ),
                        ],
                    ));
                },
                28 | 29 | 30 | 31 => {
                    let (image, color, size) = match index {
                        28 => (alliance, Color::BLACK, small),
                        29 => (horde, Color::BLACK, small),
                        30 => (frostmourne, Color::BLACK, small),
                        31 => (ashbringer, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0,
                                max_size.y / 5.0,
                                0.7,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                0.0,
                                max_size.y / 5.0,
                                0.6,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 5.0,
                                max_size.y / 5.0,
                                0.5,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                0.0,
                                0.4,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, 0.0, 0.3, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0,
                                0.0,
                                0.2,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                Some(color),
                            ),
                            template(
                                max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                Some(color),
                            ),
                        ],
                    ));
                },
                32 | 33 | 34 | 35 => {
                    let (image, color, size) = match index {
                        32 => (alliance, Color::BLACK, small),
                        33 => (horde, Color::BLACK, small),
                        34 => (frostmourne, Color::BLACK, small),
                        35 => (ashbringer, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0,
                                max_size.y / 5.0,
                                0.6,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                0.0,
                                max_size.y / 5.0,
                                0.7,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 5.0,
                                max_size.y / 5.0,
                                0.8,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                0.0,
                                0.3,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, 0.0, 0.4, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0,
                                0.0,
                                0.5,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                0.0,
                                -max_size.y / 5.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.2,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
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

mod button {
    use bevy::prelude::*;

    pub mod asset {
        pub const BUTTON: &'static str = "misc/rev2/button-atlas_1998x429.png";
    }

    #[derive(Component, Clone, PartialEq)]
    pub enum Marker {
        Undo,
        Redo,
        Help,
        NewGame,
    }

    impl Marker {
        pub fn as_string(&self) -> &'static str {
            use Marker::*;

            match self {
                Undo => "[U]ndo",
                Redo => "[R]edo",
                Help => "[H]elp",
                NewGame => "NewGame",
            }
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
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let tile_texture: Handle<Image> = asset_server.load(tile::asset::texture::TILE);
    let tile_size = Vec2::new(
        (projection.area.height() / tile::PositionGenerator::<tile::Turtle>::ROWS as f32) * 0.8,
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
    let tile_size_ratio = tile_size.y / tile::asset::texture::TILE_NO_BORDER_HEIGHT as f32;
    let tile_border_length_scaled =
        tile::asset::texture::TILE_BORDER_LENGTH as f32 * tile_size_ratio;
    let tile_pos_offset = Vec3::new(
        -(tile_size.x * tile::PositionGenerator::<tile::Turtle>::COLUMNS as f32 / 2.0)
            + tile_size.x * 1.0
            - tile_border_length_scaled / 2.0,
        -projection.area.height() / 2.0 + tile_size_full.y * 0.5 - tile_border_length_scaled,
        0.0,
    );

    let positions: Vec<tile::Position> = position_generator.collect();
    // let positions = vec![
    //     tile::Position(UVec3::new(0, 0, 0)),
    //     tile::Position(UVec3::new(2, 0, 0)),
    //     tile::Position(UVec3::new(0, 2, 0)),
    //     tile::Position(UVec3::new(2, 2, 0)),
    // ];
    let positions = generate_solvable_board(
        &positions,
        tile::PositionGenerator::<tile::Turtle>::TILE_GRID_SIZE as u32,
        Some(2),
    );

    for (pos, variant) in positions {
        let default_depth = Vec3::default().with_z(100.0);
        let column_depth_offset_factor = Vec3::default().with_z(-0.1);
        let row_depth_offset_factor =
            column_depth_offset_factor * tile::PositionGenerator::<tile::Turtle>::COLUMNS as f32;
        let layer_depth_offset_factor = Vec3::default().with_z(10.0);
        let layer_offset_factor = Vec3 {
            x: tile_border_length_scaled,
            y: tile_border_length_scaled,
            ..default()
        };

        let special = match pos.x / tile_grid_size {
            0 => Vec3::default().with_z(
                -column_depth_offset_factor.z
                    * tile::PositionGenerator::<tile::Turtle>::COLUMNS as f32,
            ),
            13 | 14 => Vec3::default().with_z(
                column_depth_offset_factor.z
                    * (tile::PositionGenerator::<tile::Turtle>::COLUMNS as f32),
            ),
            _ => Vec3::default(),
        };

        let mut entity_commands = spawn(
            &mut commands,
            (
                tile::Tile {
                    marker: tile::Marker::<0>,
                    position: pos,
                    variant: variant,
                },
                Sprite {
                    custom_size: Some(tile_size_full),
                    color: tile::DEFAULT_COLOR,
                    ..Sprite::from_image(tile_texture.clone())
                },
                Transform {
                    translation: (((pos.as_vec3() / tile_grid_size as f32)
                        * tile_size.extend(1.0))
                        + tile_pos_offset)
                        + default_depth
                        + (layer_offset_factor * pos.z as f32)
                        + (column_depth_offset_factor * pos.x as f32)
                        + (row_depth_offset_factor * pos.y as f32)
                        + (layer_depth_offset_factor * pos.z as f32)
                        + special,
                    ..default()
                },
            ),
        );

        entity_commands.observe(tile_pressed);

        if pos.z != 0 {
            entity_commands.with_child((
                Sprite {
                    custom_size: Some(tile_size_full),
                    color: Color::hsla(0.0, 0.0, 0.0, 0.75),
                    ..Sprite::from_image(tile_texture.clone())
                },
                Transform {
                    scale: Vec3 {
                        x: 1.2,
                        y: 1.03,
                        ..Vec3::splat(1.0)
                    },
                    translation: Vec3 {
                        x: -tile_size_full.x / 2.0,
                        y: -tile_size_full.y / 2.0,
                        z: column_depth_offset_factor.z * pos.x as f32,
                        ..default()
                    },
                    ..default()
                },
                Anchor::BOTTOM_LEFT,
            ));
        }

        let offset = layer_offset_factor / 2.0;
        tile::Variant::insert_sprite_as_child(
            &asset_server,
            &mut entity_commands,
            variant.0,
            &tile_size,
            &offset,
        );
    }
}

pub fn generate_solvable_board(
    available_positions: &Vec<tile::Position>,
    grid_resolution: u32,
    seed: Option<u64>,
) -> Vec<(tile::Position, tile::Variant)> {
    let grid_resolution = grid_resolution as usize;

    if available_positions.len() % 2 != 0 {
        panic!();
    }

    let mut result: Vec<(tile::Position, tile::Variant)> = Vec::new();

    // Set rng seed
    let seed = seed.unwrap_or(0);
    let mut rng = StdRng::seed_from_u64(seed);

    // Generate [tile::Variant] pairs
    let tile_pairs: u32 = available_positions.len() as u32 / 2;
    let tile_pair_group_size = 4; // I.e., there are two pairs for each variant => 4
    let mut available_tile_variants: Vec<(tile::Variant, tile::Variant)> = (0..tile_pairs)
        .map(|variant| {
            (
                tile::Variant(variant),
                tile::Variant(variant + available_positions.len() as u32 / tile_pair_group_size),
            )
        })
        .collect();
    available_tile_variants.shuffle(&mut rng);

    // Generate lookup tensors (Row x Layer x Column)
    let mut rows = 0;
    let mut layers = 0;
    let mut columns = 0;

    for tile::Position(UVec3 { x, y, z }) in available_positions {
        if *y + 1 > rows {
            rows = *y + 1;
        }
        if *z + 1 > layers {
            layers = *z + 1;
        }
        if *x + 1 > columns {
            columns = *x + 1;
        }
    }

    let mut lookup: Vec<Vec<Vec<(&tile::Position, usize)>>> =
        vec![vec![vec![]; layers as usize]; rows as usize];
    for (index, pos) in available_positions.iter().enumerate() {
        lookup[pos.y as usize][pos.z as usize].push((pos, index));
    }
    let mut occupied: Vec<Vec<Vec<(&tile::Position, usize)>>> =
        vec![vec![vec![]; layers as usize]; rows as usize];

    fn place_seed_tile<'a>(
        lookup: &mut Vec<Vec<Vec<(&'a tile::Position, usize)>>>,
        occupied: &mut Vec<Vec<Vec<(&'a tile::Position, usize)>>>,
        result: &mut Vec<(tile::Position, tile::Variant)>,
        rng: &mut StdRng,
        row: usize,
        layer: usize,
        variant: tile::Variant,
    ) -> tile::Position {
        assert!(lookup[row][layer].len() != 0);
        let random_column = rng.random_range(0..lookup[row][layer].len());
        let (pos, index) = lookup[row][layer].swap_remove(random_column);
        occupied[row][layer].push((pos, index));
        result.push((*pos, variant));
        *pos
    }

    // Pick positions from the lookup table...
    // ...and move them into the occupied table (and add to the result vector when doing so)
    for (variant0, variant1) in available_tile_variants {
        let variants = [variant0, variant1];
        let random_row = rng.random_range(0..lookup.len());
        let mut banned_position: Option<UVec3> = None; // Decided by first tile placement.
        debug!(random_row);

        for variant in variants {
            let mut lookup_original: Option<Vec<Vec<Vec<(&tile::Position, usize)>>>> = None;

            if let Some(pos) = banned_position {
                // Replace lookup table with filtered version.
                // This is done to ensure that the SECOND tile place in a pair is every placed on top of the FIRST.
                lookup_original = Some(lookup.clone());

                let row = pos.y;
                if let Some(row_vec) = lookup.get_mut(row as usize) {
                    for layer in (pos.z as usize + 1)..(layers as usize) {
                        row_vec[layer].retain(|(pos_, _index)| {
                            let pos = pos.truncate();
                            let pos_ = pos_.truncate();
                            let overlaps_banned_position = (pos == pos_)
                                || (pos.x - 1 == pos_.x && pos.y - 1 == pos_.y)
                                || (pos.x - 1 == pos_.x && pos.y == pos_.y)
                                || (pos.x - 1 == pos_.x && pos.y + 1 == pos_.y)
                                || (pos.x == pos_.x && pos.y + 1 == pos_.y)
                                || (pos.x + 1 == pos_.x && pos.y + 1 == pos_.y)
                                || (pos.x + 1 == pos_.x && pos.y == pos_.y)
                                || (pos.x + 1 == pos_.x && pos.y - 1 == pos_.y)
                                || (pos.x == pos_.x && pos.y - 1 == pos_.y);
                            !overlaps_banned_position
                        });
                    }
                }
            }

            for layer in 0..layers as usize {
                debug!(layer);
                let lookup_layer_is_empty = lookup[random_row][layer].is_empty();
                debug!(lookup_layer_is_empty);
                if lookup_layer_is_empty {
                    // Nothing to pick - Go next.
                    // continue;
                    break;
                }

                let occupied_row_is_empty = occupied[random_row][layer].is_empty()
                    && (if random_row + 1 < occupied.len() {
                        debug!("One above");
                        occupied[random_row + 1][layer].is_empty()
                    } else {
                        true
                    })
                    && (if random_row > 0 {
                        debug!("One below");
                        occupied[random_row - 1][layer].is_empty()
                    } else {
                        true
                    });
                debug!(occupied_row_is_empty);
                if occupied_row_is_empty {
                    debug!("Place tile! Row: {:?} | Layer {:?}", random_row, layer);
                    let pos = place_seed_tile(
                        &mut lookup,
                        &mut occupied,
                        &mut result,
                        &mut rng,
                        random_row,
                        layer,
                        variant,
                    );
                    if banned_position.is_none() {
                        banned_position = Some(*pos);
                    }
                    break;
                }
            }

            if let Some(lookup_original) = lookup_original {
                // Restore lookup table
                lookup = lookup_original;
            }
        }

        // REMOVE ANY EMPTY LOOKUP TABLES!!!
        // (Performance) ONLY REMOVE THOSE THAT WE HAVE REMOVED FROM!
    }

    return result;
}

pub fn tile_pressed(
    on_press: On<Pointer<Press>>,
    mut commands: Commands,
    mut tiles: Query<
        (
            Entity,
            &tile::Variant,
            &tile::Position,
            &mut Sprite,
            &mut Visibility,
        ),
        (With<tile::Marker<0>>, Without<marker::Hidden>),
    >,
    mut selected_tile: ResMut<SelectedTile>,
    mut history: ResMut<History>,
) {
    let (pressed_entity, _, _, _, _) = tiles.iter().find(|tile| tile.0 == on_press.entity).unwrap();

    let Some(selected_entity) = selected_tile.0.take() else {
        let (_, _, _, mut pressed_sprite, _) = tiles.get_mut(pressed_entity).unwrap();
        pressed_sprite.color = Color::hsl(0.5, 1.0, 1.5);
        selected_tile.0 = Some(pressed_entity);
        return;
    };

    if selected_entity == pressed_entity {
        let (_, _, _, mut pressed_sprite, _) = tiles.get_mut(pressed_entity).unwrap();
        pressed_sprite.color = tile::DEFAULT_COLOR;
        return;
    }

    let (
        _selected_entity,
        _selected_variant,
        _selected_position,
        mut selected_sprite,
        _selected_visibility,
    ) = tiles.get_mut(selected_entity).unwrap();
    selected_sprite.color = tile::DEFAULT_COLOR;

    let [
        (pressed_entity, pressed_variant, pressed_position, pressed_sprite, pressed_visibility),
        (
            selected_entity,
            selected_variant,
            selected_position,
            _selected_sprite,
            _selected_visibility,
        ),
    ] = tiles.get_many([pressed_entity, selected_entity]).unwrap();

    if valid_removal(
        pressed_entity,
        selected_entity,
        pressed_variant,
        selected_variant,
        pressed_position,
        selected_position,
        &tiles,
    ) == false
    {
        let (_, _, _, mut pressed_sprite, _) = tiles.get_mut(pressed_entity).unwrap();
        pressed_sprite.color = Color::hsl(0.5, 1.0, 1.5);
        selected_tile.0 = Some(pressed_entity);
        return;
    }

    let [
        (
            pressed_entity,
            pressed_variant,
            pressed_position,
            mut pressed_sprite,
            mut pressed_visibility,
        ),
        (
            selected_entity,
            selected_variant,
            selected_position,
            mut selected_sprite,
            mut selected_visibility,
        ),
    ] = tiles
        .get_many_mut([pressed_entity, selected_entity])
        .unwrap();

    history.push_front(HistoryItem::ValidPair(pressed_entity, selected_entity));
    commands.entity(pressed_entity).insert(marker::Hidden);
    commands.entity(selected_entity).insert(marker::Hidden);
    *pressed_visibility = Visibility::Hidden;
    *selected_visibility = Visibility::Hidden;
}

pub fn valid_removal(
    pressed_entity: Entity,
    selected_entity: Entity,
    pressed_variant: &tile::Variant,
    selected_variant: &tile::Variant,
    pressed_position: &tile::Position,
    selected_position: &tile::Position,
    tiles: &Query<
        (
            Entity,
            &tile::Variant,
            &tile::Position,
            &mut Sprite,
            &mut Visibility,
        ),
        (With<tile::Marker<0>>, Without<marker::Hidden>),
    >,
) -> bool {
    fn matching_variants(
        pressed_entity: Entity,
        selected_entity: Entity,
        pressed_variant: &tile::Variant,
        selected_variant: &tile::Variant,
    ) -> bool {
        let m = pressed_entity != selected_entity && pressed_variant == selected_variant;

        if !m {
            info!("Tiles are not matching!");
        }

        m
    }

    fn free_horizontally(
        entity: Entity,
        position: &tile::Position,
        tiles: &Query<
            (
                Entity,
                &tile::Variant,
                &tile::Position,
                &mut Sprite,
                &mut Visibility,
            ),
            (With<tile::Marker<0>>, Without<marker::Hidden>),
        >,
    ) -> bool {
        const TGS: u32 = tile::PositionGenerator::<tile::Turtle>::TILE_GRID_SIZE as u32;

        let mut blocked_left = false;
        let mut blocked_right = false;

        for (other, _, other_pos, _, _) in tiles.iter() {
            if other == entity {
                continue;
            }
            if position.z != other_pos.z {
                continue;
            }

            let overlapping_row = position.y + TGS > other_pos.y && position.y < other_pos.y + TGS;

            if !overlapping_row {
                continue;
            }

            if position.x == other_pos.x + TGS {
                blocked_left = true;
            }

            if position.x + TGS == other_pos.x {
                blocked_right = true;
            }
        }

        !(blocked_left && blocked_right)
    }

    fn free_above(
        entity: Entity,
        position: &tile::Position,
        tiles: &Query<
            (
                Entity,
                &tile::Variant,
                &tile::Position,
                &mut Sprite,
                &mut Visibility,
            ),
            (With<tile::Marker<0>>, Without<marker::Hidden>),
        >,
    ) -> bool {
        const TGS: u32 = tile::PositionGenerator::<tile::Turtle>::TILE_GRID_SIZE as u32;

        !tiles.iter().any(|(other, _, other_pos, _, _)| {
            other != entity
                && other_pos.z > position.z
                && position.y + TGS > other_pos.y
                && position.y < other_pos.y + TGS
                && position.x + TGS > other_pos.x
                && position.x < other_pos.x + TGS
        })
    }

    matching_variants(
        pressed_entity,
        selected_entity,
        pressed_variant,
        selected_variant,
    ) && free_horizontally(pressed_entity, pressed_position, tiles)
        && free_horizontally(selected_entity, selected_position, tiles)
        && free_above(pressed_entity, pressed_position, tiles)
        && free_above(selected_entity, selected_position, tiles)
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

    let texture_handle: Handle<Image> = asset_server.load(button::asset::BUTTON);
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(666, 429), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let button_size = Vec2::new(
        (projection.area.height() / tile::PositionGenerator::<tile::Turtle>::ROWS as f32) / 0.7,
        projection.area.height() / tile::PositionGenerator::<tile::Turtle>::ROWS as f32,
    );
    let font = (
        TextFont {
            font_size: button_size.y / 5.0,
            ..default()
        },
        Text2dShadow {
            offset: Vec2 { x: 3.0, y: -3.0 },
            color: Color::srgba(0.0, 0.0, 0.0, 0.95),
        },
        TextColor(Color::srgb_u8(239, 191, 4)),
    );

    struct Button {
        marker: button::Marker,
        flip_x: bool,
        offset: Vec3,
    }

    let buttons = [
        Button {
            marker: button::Marker::Undo,
            flip_x: false,
            offset: Vec3::default(),
        },
        Button {
            marker: button::Marker::Redo,
            flip_x: false,
            offset: Vec3 {
                y: button_size.y,
                ..default()
            },
        },
        Button {
            marker: button::Marker::NewGame,
            flip_x: true,
            offset: Vec3::default(),
        },
        Button {
            marker: button::Marker::Help,
            flip_x: true,
            offset: Vec3 {
                y: button_size.y,
                ..default()
            },
        },
    ];

    for button in buttons {
        let mut ec = spawn(
            &mut commands,
            (
                button.marker.clone(),
                Sprite {
                    custom_size: Some(button_size),
                    ..Sprite::from_atlas_image(
                        texture_handle.clone(),
                        TextureAtlas {
                            layout: texture_atlas_handle.clone(),
                            index: 0,
                        },
                    )
                },
                Transform {
                    translation: Vec3 {
                        x: (-projection.area.width() / 2.0)
                            * if button.flip_x { -1.0 } else { 1.0 },
                        y: -projection.area.height() / 2.0,
                        ..default()
                    } + button.offset,
                    ..default()
                },
                if button.flip_x {
                    Anchor::BOTTOM_RIGHT
                } else {
                    Anchor::BOTTOM_LEFT
                },
                children![(
                    Text2d(button.marker.as_string().to_owned()),
                    font.clone(),
                    Transform {
                        translation: button_size.extend(0.0) / 2.0
                            * if button.flip_x {
                                Vec3 {
                                    x: -1.0,
                                    y: 1.0,
                                    z: 1.0,
                                }
                            } else {
                                Vec3 {
                                    x: 1.0,
                                    y: 1.0,
                                    z: 1.0,
                                }
                            },
                        ..default()
                    },
                )],
            ),
        );

        ec.observe(mouse_over)
            .observe(mouse_out)
            .observe(mouse_press)
            .observe(mouse_release);

        match button.marker {
            button::Marker::Undo => ec.observe(undo_mouse),
            button::Marker::Redo => ec.observe(redo_mouse),
            button::Marker::Help => ec.observe(help_mouse),
            button::Marker::NewGame => ec.observe(new_game_mouse),
        };
    }
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

fn mouse_activity(
    entity: Entity,
    buttons: &mut Query<(Entity, &mut Sprite)>,
    new_index: usize,
) {
    let (_entity, mut sprite) = buttons
        .iter_mut()
        .find(|(entity_, _)| *entity_ == entity)
        .unwrap();
    sprite.texture_atlas.as_mut().unwrap().index = new_index;
}

fn mouse_over(
    on_over: On<Pointer<Over>>,
    mut buttons: Query<(Entity, &mut Sprite)>,
) {
    mouse_activity(on_over.entity, &mut buttons, 1);
}

fn mouse_out(
    on_out: On<Pointer<Out>>,
    mut buttons: Query<(Entity, &mut Sprite)>,
) {
    mouse_activity(on_out.entity, &mut buttons, 0);
}

fn mouse_press(
    on_press: On<Pointer<Press>>,
    mut buttons: Query<(Entity, &mut Sprite)>,
) {
    mouse_activity(on_press.entity, &mut buttons, 2);
}

fn mouse_release(
    on_release: On<Pointer<Release>>,
    mut buttons: Query<(Entity, &mut Sprite)>,
) {
    mouse_activity(on_release.entity, &mut buttons, 1);
}

fn undo_mouse(
    _on_press: On<Pointer<Press>>,
    mut commands: Commands,
    mut history_valid_pair_tiles: Query<
        &mut Visibility,
        (With<tile::Marker<0>>, With<marker::Hidden>),
    >,
    mut history: ResMut<History>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    undo(
        &mut commands,
        &mut history_valid_pair_tiles,
        &mut history,
        &mut selected_tile,
    )
}

fn undo_keyboard(
    key: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut history_valid_pair_tiles: Query<
        &mut Visibility,
        (With<tile::Marker<0>>, With<marker::Hidden>),
    >,
    mut history: ResMut<History>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    if key.just_pressed(KeyCode::KeyU) {
        undo(
            &mut commands,
            &mut history_valid_pair_tiles,
            &mut history,
            &mut selected_tile,
        )
    }
}

fn undo(
    commands: &mut Commands,
    history_valid_pair_tiles: &mut Query<
        &mut Visibility,
        (With<tile::Marker<0>>, With<marker::Hidden>),
    >,
    history: &mut ResMut<History>,
    selected_tile: &mut ResMut<SelectedTile>,
) {
    if let Some(history_item) = history.pop_front() {
        ***selected_tile = None;

        match history_item {
            HistoryItem::ValidPair(entity0, entity1) => {
                let [mut a, mut b] = history_valid_pair_tiles
                    .get_many_mut([entity0, entity1])
                    .unwrap();
                commands.entity(entity0).remove::<marker::Hidden>();
                commands.entity(entity1).remove::<marker::Hidden>();
                *a = Visibility::Inherited;
                *b = Visibility::Inherited;
            },
            HistoryItem::Shuffle(items) => todo!(),
        }
    }
}

fn redo_mouse(
    _on_press: On<Pointer<Press>>,
    mut commands: Commands,
    mut history_valid_pair_tiles: Query<&mut Visibility, With<tile::Marker<0>>>,
    mut history: ResMut<History>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    redo(
        &mut commands,
        &mut history_valid_pair_tiles,
        &mut history,
        &mut selected_tile,
    )
}

fn redo_keyboard(
    key: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut history_valid_pair_tiles: Query<&mut Visibility, With<tile::Marker<0>>>,
    mut history: ResMut<History>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    if key.just_pressed(KeyCode::KeyR) {
        redo(
            &mut commands,
            &mut history_valid_pair_tiles,
            &mut history,
            &mut selected_tile,
        )
    }
}

fn redo(
    commands: &mut Commands,
    history_valid_pair_tiles: &mut Query<&mut Visibility, With<tile::Marker<0>>>,
    history: &mut ResMut<History>,
    selected_tile: &mut ResMut<SelectedTile>,
) {
    if let Some(history_item) = history.pop_front_redo() {
        ***selected_tile = None;

        match history_item {
            HistoryItem::ValidPair(entity0, entity1) => {
                let [mut a, mut b] = history_valid_pair_tiles
                    .get_many_mut([entity0, entity1])
                    .unwrap();
                history.push_front_redo(HistoryItem::ValidPair(entity0, entity1));
                commands.entity(entity0).insert(marker::Hidden);
                commands.entity(entity1).insert(marker::Hidden);
                *a = Visibility::Hidden;
                *b = Visibility::Hidden;
            },
            HistoryItem::Shuffle(items) => todo!(),
        }
    }
}

fn help_mouse(
    _on_press: On<Pointer<Press>>,
    mut help_msg: MessageWriter<HelpMsg>,
) {
    help_msg.write(HelpMsg);
}

fn help_keyboard(
    key: Res<ButtonInput<KeyCode>>,
    mut help_msg: MessageWriter<HelpMsg>,
) {
    if key.just_pressed(KeyCode::KeyH) {
        help_msg.write(HelpMsg);
    }
}

fn help_toggle(
    mut help_msg: MessageReader<HelpMsg>,
    mut buttons: Query<(Entity, &button::Marker, &mut Sprite)>,
    mut help_enabled: ResMut<HelpEnabled>,
) {
    if help_msg.is_empty() {
        return;
    }
    help_msg.clear();

    **help_enabled = !(**help_enabled);

    let (_, _, mut button_sprite) = buttons
        .iter_mut()
        .find(|(_entity, marker, _sprite)| **marker == button::Marker::Help)
        .unwrap();

    match **help_enabled {
        true => button_sprite.color = Color::hsl(120.0, 1.0, 0.5),
        false => button_sprite.color = Color::default(),
    }
}

fn help(
    mut tiles: Query<
        (
            Entity,
            &tile::Variant,
            &tile::Position,
            &mut Sprite,
            &mut Visibility,
        ),
        (With<tile::Marker<0>>, Without<marker::Hidden>),
    >,
    selected_tile: Res<SelectedTile>,
    help_enabled: Res<HelpEnabled>,
    mut prev_selection: Local<Option<Entity>>,
) {
    match selected_tile.0 {
        Some(selected_tile) => {
            if !**help_enabled {
                *prev_selection = None;

                let (
                    selected_entity,
                    selected_variant,
                    _selected_position,
                    _selected_sprite,
                    _selected_visiblity,
                ) = tiles.get(selected_tile).unwrap();

                let mut entities_to_reset = vec![];

                for (entity, variant, _position, _sprite, _visibility) in &tiles {
                    if selected_entity != entity && *selected_variant == *variant {
                        entities_to_reset.push(entity);
                    }
                }

                for entity in entities_to_reset {
                    let (_entity, _variant, _position, mut sprite, _visiblity) =
                        tiles.get_mut(entity).unwrap();
                    sprite.color = tile::DEFAULT_COLOR;
                }
                return;
            }

            if let Some(prev_selection) = *prev_selection {
                if prev_selection == selected_tile {
                    return;
                }
            }

            *prev_selection = Some(selected_tile);

            let (
                selected_entity,
                selected_variant,
                _selected_position,
                _selected_sprite,
                _selected_visiblity,
            ) = tiles.get(selected_tile).unwrap();

            let mut entities_to_update = vec![];
            let mut entities_to_reset = vec![];

            for (entity, variant, _position, _sprite, _visibility) in &tiles {
                if selected_entity != entity && *selected_variant == *variant {
                    entities_to_update.push(entity);
                } else if selected_entity != entity {
                    entities_to_reset.push(entity);
                }
            }

            for entity in entities_to_update {
                let (_entity, _variant, _position, mut sprite, _visiblity) =
                    tiles.get_mut(entity).unwrap();
                sprite.color = Color::hsl(120.0, 1.0, 0.5);
            }

            for entity in entities_to_reset {
                let (_entity, _variant, _position, mut sprite, _visiblity) =
                    tiles.get_mut(entity).unwrap();
                sprite.color = tile::DEFAULT_COLOR;
            }
        },
        None => {
            *prev_selection = None;
            for (_entity, _variant, _position, mut sprite, _visibility) in &mut tiles {
                sprite.color = tile::DEFAULT_COLOR;
            }
        },
    }
}

fn new_game_mouse(
    _on_press: On<Pointer<Press>>,
    mut commands: Commands,
    mut history_valid_pair_tiles: Query<&mut Visibility, With<tile::Marker<0>>>,
    mut history: ResMut<History>,
) {
    info!("new game");
}
