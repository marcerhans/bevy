use crate::plugin::shared::*;
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(Dimensions::default())
            .add_systems(Startup, load_assets);
    }
}

#[derive(Resource)]
struct Dimensions {
    rows: u32,
    cols: u32,
    width: u32,
    height: u32,
}

impl Default for Dimensions {
    fn default() -> Self {
        Self {
            rows: 4,
            cols: 10,
            width: 300,
            height: 400,
        }
    }
}

#[derive(Component, Default)]
pub struct Tile {
    kind: Option<usize>,
}

#[derive(Bundle, Default)]
pub struct TileBundle {
    sprite: Sprite,
    transform: Transform,
    tile: Tile,
}

fn load_assets(
    mut assets: ResMut<resource::asset::Assets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    dim: Res<Dimensions>,
) {
    assets.load::<Image>(
        "riichi_mahjong_tiles/generated/Black/character_atlas.png",
        "tile::atlas",
    );

    assets.add(
        texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(dim.width, dim.height),
            dim.cols,
            dim.rows,
            None,
            None,
        )),
        "tile::atlas_layout",
    );
}
