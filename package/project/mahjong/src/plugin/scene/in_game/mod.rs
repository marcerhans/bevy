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

// #[derive(Resource)]
// pub struct PreviouslySelectedTile(pub Option<(Entity, tile::Variant)>);

#[derive(Component)]
pub struct Background;

mod tile {
    use bevy::prelude::*;

    pub mod asset {
        pub mod texture {
            pub const BORDER: &'static str = "misc/rev2/Tile4_700x1000.png";
            pub const TEXTURE_BOTTOM_BORDER_PERCENTAGE_Y: f32 = 175.0 / 1000.0; // (Just the "thickness" of the tile, excluding the border)
            pub const TEXTURE_LEFT_BORDER_PERCENTAGE_X: f32 = 124.0 / 700.0; // (Just the "thickness" of the tile, excluding the border)
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

    pub trait VariantTrait {
        fn get_sprite(
            &self,
            size: Vec2,
        ) -> impl Bundle;
    }

    #[derive(Component)]
    pub enum Variant {
        WoW(variant::wow::Variant),
    }

    pub mod variant {
        pub mod wow {
            use super::super::VariantTrait;
            use bevy::prelude::*;

            pub enum Variant {
                Alliance(u32),
                Horde(u32),
            }

            impl VariantTrait for Variant {
                fn get_sprite(
                    &self,
                    size: Vec2,
                ) -> impl Bundle {
                    Sprite::from_color(Color::WHITE, size)
                }
            }

            pub mod asset {
                pub mod texture {
                    pub const ALLIANCE: &'static str = "misc/rev2/Alliance_1104x882.png";
                    pub const HORDE: &'static str = "misc/rev2/Horde_740x1093.png";
                }
            }
        }

        pub mod warhammer {
            use super::super::VariantTrait;
            use bevy::prelude::*;

            pub enum Variant {
                TheImperiumOfMan(u32),
                TheForcesOfChaos(u32),
                Orks(u32),
                Tyranids(u32),
            }

            impl VariantTrait for Variant {
                fn get_sprite(
                    &self,
                    size: Vec2,
                ) -> impl Bundle {
                    Sprite::from_color(Color::WHITE, size)
                }
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

    commands.spawn((
        Background,
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

    // Load assets and set texture constants
    // Tile

    // Determine size and position(s) for tiles
    // let tile_height = projection.area.height() as f32 / PositionGenerator::<Turtle>::ROWS as f32;
    // let tile_height = tile_height + TEXTURE_BOTTOM_BORDER_PERCENTAGE_Y * tile_height; // NOTE:1: This is hacky, but allows for tile height to fill height of window.
    // let tile_width = tile_height * 0.7;
    // let tile_size = Vec2::new(tile_width, tile_height);
    // let tile_center_offset = Vec2::new(
    //     TEXTURE_LEFT_BORDER_PERCENTAGE_X,
    //     TEXTURE_BOTTOM_BORDER_PERCENTAGE_Y,
    // ) * tile_size
    //     / 2.0;
    // let tile_thickness_offset = Vec2::new(
    //     TEXTURE_LEFT_BORDER_PERCENTAGE_X,
    //     TEXTURE_BOTTOM_BORDER_PERCENTAGE_Y,
    // ) * tile_size;
    // let tile_logical_size = tile_size - tile_thickness_offset * 1.2;
    // let tile_factory = tile::Factory::new(
    //     texture_tile.clone(),
    //     texture_alliance,
    //     texture_horde,
    //     tile_logical_size,
    //     Some(tile_size),
    //     Some(tile_center_offset),
    // );
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

    let texture_button_atlas: Handle<Image> =
        asset_server.load("misc/rev2/button-atlas_1998x429.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(666, 429), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    struct Button {
        translation: Vec3,
        text: &'static str,
    }
    let button_base = (
        DespawnOnExit(InGame::Root),
        // ButtonSprite,
        Pickable::default(),
    );

    let tile_height = projection.area.height() / 8.0 * 1.5;
    let button_size = Vec2::new(tile_height * 1.5, tile_height * 0.75);
    let button_margin = Vec2::new(5.0, 5.0);
    let button_pos_start = Vec3::new(
        -(tile_size.x - tile_thickness_offset.x) * PositionGenerator::<Turtle>::COLUMNS as f32
            / 2.0
            - tile_size.x / 2.0,
        -(tile_size.y - tile_thickness_offset.y) * PositionGenerator::<Turtle>::ROWS as f32 / 2.0
            + button_size.y * 0.5,
        999.0,
    );
    let button_pos_start_right = Vec3::new(
        (tile_size.x - tile_thickness_offset.x) * PositionGenerator::<Turtle>::COLUMNS as f32 / 2.0
            + tile_size.x / 2.0,
        -(tile_size.y - tile_thickness_offset.y) * PositionGenerator::<Turtle>::ROWS as f32 / 2.0
            + button_size.y * 0.5,
        999.0,
    );
    let buttons = [
        Button {
            translation: button_pos_start,
            text: "[H]elp",
        },
        Button {
            translation: Vec3 {
                x: button_pos_start.x + (button_size.x + button_margin.x) * 0.0,
                y: button_pos_start.y + (button_size.y + button_margin.y) * 1.0,
                ..button_pos_start
            },
            text: "[S]huffle",
        },
        Button {
            translation: button_pos_start_right,
            text: "  [U]ndo\n(limit: 32)",
        },
    ];

    for button in buttons {
        commands
            .spawn((
                button_base.clone(),
                Sprite {
                    custom_size: Some(button_size.clone()),
                    ..Sprite::from_atlas_image(
                        texture_button_atlas.clone(),
                        TextureAtlas {
                            layout: texture_atlas_handle.clone(),
                            index: 0,
                        },
                    )
                },
                Transform {
                    translation: button.translation,
                    ..default()
                },
            ))
            .with_child((
                ButtonSprite,
                Text2d::new(button.text),
                TextFont {
                    font_size: button_size.y / 5.0,
                    ..default()
                },
                TextColor(Color::srgb_u8(255, 215, 0)),
            ));
            // .observe(button_over)
            // .observe(button_press)
            // .observe(button_release)
            // .observe(button_out);
    }
}

fn resize_background(
    mut transform: Query<(&mut Transform, &mut Sprite), With<Background>>,
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
