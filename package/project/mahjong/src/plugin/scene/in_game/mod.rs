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

    impl Position {
        fn generator<T>() -> PositionGenerator<T> {
            PositionGenerator::<T> {
                counter: 0,
                _type: PhantomData,
            }
        }
    }

    pub struct Turtle;

    pub struct PositionGenerator<T> {
        counter: u32,
        _type: PhantomData<T>,
    }

    impl Iterator for PositionGenerator<Turtle> {
        type Item = Position;

        fn next(&mut self) -> Option<Self::Item> {
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
                    let row = 3.5 * 2.0;
                    let column = 5.5 * 2.0;
                    let layer = 4.0 * 2.0;
                    let local_position = Position(UVec3::new(column as u32, row as u32, layer as u32));
                    return Some(local_position);
                },
                _ => return None,
            }

            // let column = match layer {
            //     0 => {
            //         match row {
            //             0 => 0 + current - 0,
            //             1 => 2 + current - 12,
            //             2 => 1 + current - 20,
            //             3 => 0 + current - 30,
            //             4 => 0 + current - 42,
            //             5 => 1 + current - 54,
            //             6 => 2 + current - 64,
            //             7 => 0 + current - 72,
            //             8 => match current - 84 {
            //                 // Last 3 are special cases. Do not follow a pattern.
            //                 0 => {
            //                     let row = 3.5;
            //                     let column = -1.0;
            //                     let local_position = Vec3::new(column, row, layer as f32)
            //                         * self.tile_size.extend(1.0);
            //                     return Some(self.local2global(&local_position) + offsets);
            //                 },
            //                 1 => {
            //                     let row = 3.5;
            //                     let column = 12.0;
            //                     let local_position = Vec3::new(column, row, layer as f32)
            //                         * self.tile_size.extend(1.0);
            //                     return Some(self.local2global(&local_position) + offsets);
            //                 },
            //                 2 => {
            //                     let row = 3.5;
            //                     let column = 13.0;
            //                     let local_position = Vec3::new(column, row, layer as f32)
            //                         * self.tile_size.extend(1.0);
            //                     return Some(self.local2global(&local_position) + offsets);
            //                 },
            //                 _ => unreachable!(),
            //             },
            //             _ => unreachable!(),
            //         }
            //     },
            //     1 => 3 + ((current - 87) % 6),
            //     2 => 4 + ((current - 123) % 4),
            //     3 => 5 + ((current - 139) % 2),
            //     _ => unreachable!(),
            // };

            // let local_position =
            //     Vec3::new(column as f32, row as f32, layer as f32) * self.tile_size.extend(1.0);
            // Some(self.local2global(&local_position) + offsets)
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

    commands.spawn((
        marker::Background,
        DespawnOnExit(InGame::Root),
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
    ));
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
