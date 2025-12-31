use crate::plugin::scene::main_menu::MainMenu;
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<InGame>().add_systems(
            OnEnter(InGame::Root),
            (spawn_tiles, spawn_buttons, spawn_background),
        );
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

pub fn spawn_buttons() {}

pub fn spawn_background() {}
