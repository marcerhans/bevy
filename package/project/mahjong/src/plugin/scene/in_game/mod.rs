use crate::plugin::scene::main_menu::MainMenu;
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<InGame>()
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
            pub const BORDER: &'static str = "misc/rev2/Tile4_700x1000.png";
            pub const ALLIANCE: &'static str = "misc/rev2/Alliance_1104x882.png";
            pub const HORDE: &'static str = "misc/rev2/Horde_740x1093.png";
            pub const BOTTOM_BORDER_PERCENTAGE_Y: f32 = 175.0 / 1000.0; // (Just the "thickness" of the tile, excluding the border)
            pub const LEFT_BORDER_PERCENTAGE_X: f32 = 124.0 / 700.0; // (Just the "thickness" of the tile, excluding the border)
        }
    }

    #[derive(Bundle)]
    pub struct Tile {
        marker: Marker,
        pub position: Position,
        pub variant: Variant,
        pub sprite: Sprite,
    }

    #[derive(Component)]
    pub struct Marker;

    #[derive(Component, Deref, DerefMut)]
    pub struct Position(UVec3);

    pub struct Turtle;

    impl Turtle {
        pub const TILES: usize = 144;
        pub const TILE_VARIANT_SIZE: usize = 4;
        pub const ROWS: usize = 8;
        pub const COLUMNS: usize = 15;
        pub const LAYERS: usize = 5;
        pub const TILE_SIZE: usize = 2;
    }

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

    impl Iterator for PositionGenerator<Turtle> {
        type Item = Position;

        fn next(&mut self) -> Option<Self::Item> {
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
                143 => {
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

    #[derive(Component, Deref, DerefMut)]
    pub struct Variant(u32);
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

    let tile_size = Vec2::new(
        (projection.area.height() / tile::Turtle::ROWS as f32) * 0.7,
        projection.area.height() / tile::Turtle::ROWS as f32,
    );
    let tile_pos_offset = Vec3::new(
        -(tile_size.x * tile::Turtle::COLUMNS as f32 / 2.0) + tile_size.x * 1.0,
        -projection.area.height() / 2.0 + tile_size.y * 0.5,
        0.0,
    );
    let tile_grid_size = 2;
    let position_generator =
        tile::PositionGenerator::<tile::Turtle>::new(UVec2::splat(tile_grid_size));

    for pos in position_generator {
        spawn(
            &mut commands,
            (
                Sprite {
                    ..Sprite::from_color(Color::WHITE, tile_size)
                },
                Transform {
                    translation: ((pos.as_vec3() / tile_grid_size as f32) * tile_size.extend(1.0))
                        + tile_pos_offset,
                    ..default()
                },
            ),
        )
        .with_child((Sprite {
            ..Sprite::from_color(Color::BLACK, tile_size * 0.9)
        },))
        .observe(tile_clicked);
    }
}

pub fn tile_clicked(on_click: On<Pointer<Press>>) {
    info!("hej");
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
