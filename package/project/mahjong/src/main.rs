mod plugin;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(plugin::Plugin).run();

    // .insert_resource(Textures::default())
    // .insert_resource(resource::State::default())
    // .add_event::<event::Clicked>()
    // .add_systems(Startup, (load_assets, setup, prepare_board).chain())
    // .add_systems(Update, (clickable_system, logic).chain())
    // .run();
}

// fn load_assets(
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
//     mut textures: ResMut<Textures>,
// ) {
//     const WIDTH: u32 = 300;
//     const HEIGHT: u32 = 400;
//     const COLS: u32 = 10;
//     const ROWS: u32 = 4;

//     let texture =
//         asset_server.load::<Image>("riichi_mahjong_tiles/generated/Black/character_atlas.png");
//     let atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
//         UVec2::new(WIDTH, HEIGHT),
//         COLS,
//         ROWS,
//         None,
//         None,
//     ));
//     textures.insert(TileBundle::texture_type(), (texture, Some(atlas_layout)));
// }

// fn prepare_board(
//     mut commands: Commands,
//     texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
//     textures: Res<Textures>,
// ) {
//     let (tile_texture, tile_atlas_layout) = textures
//         .get(&TileBundle::texture_type())
//         .expect("Texture not yet loaded");
//     let tile_atlas_layout = tile_atlas_layout
//         .as_ref()
//         .expect("Atlas layout missing for Tile.");
//     let tile_atlas_layout_asset = texture_atlas_layouts
//         .get(tile_atlas_layout)
//         .expect("Could not find texture atlas layout.");

//     let atlas_size = &tile_atlas_layout_asset.size;
//     let length = tile_atlas_layout_asset.textures.len();
//     let cell_size = tile_atlas_layout_asset.textures[0].size();
//     let cols = tile_atlas_layout_asset.size.x / cell_size.x;
//     let rows = tile_atlas_layout_asset.size.y / cell_size.y;
//     let scale = 0.2;

//     for i in 0..length {
//         let layer = 0;
//         commands.spawn((
//             RenderLayers::layer(layer + 1),
//             TileBundle {
//                 sprite: Sprite {
//                     image: tile_texture.clone(),
//                     texture_atlas: Some(TextureAtlas {
//                         layout: tile_atlas_layout.clone(),
//                         index: i as usize,
//                     }),
//                     custom_size: Some(Vec2::new(
//                         cell_size.x as f32 * scale,
//                         cell_size.y as f32 * scale,
//                     )),
//                     ..default()
//                 },
//                 transform: Transform {
//                     translation: Vec3::new(i as f32 * cell_size.x as f32 * scale, 0.0, 0.0),
//                     ..default()
//                 },
//                 tile: Tile { kind: Some(i / 2) },
//                 ..default()
//             },
//         ));
//     }
// }

// fn clickable_system(
//     mouse_button_input: Res<ButtonInput<MouseButton>>,
//     window: Single<&Window>,
//     camera_transform: Single<(&Camera, &GlobalTransform, &CameraMain)>,
//     mut query: Query<(Entity, &Transform, &Sprite), With<Clickable>>,
//     mut writer: EventWriter<event::Clicked>,
// ) {
//     if let Some(cursor_pos) = window.cursor_position() {
//         if mouse_button_input.just_pressed(MouseButton::Left) {
//             let (camera, camera_transform, _) = camera_transform.into_inner();
//             if let Ok(click_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
//                 for (entity, transform, sprite) in &mut query {
//                     let translation = transform.translation;
//                     let size = sprite
//                         .custom_size
//                         .expect("Clickable object is lacking custom size.");
//                     let half_size = size / 2.0;

//                     if (click_pos.x >= translation.x - half_size.x)
//                         && (click_pos.x <= translation.x + half_size.x)
//                         && (click_pos.y >= translation.y - half_size.y)
//                         && (click_pos.y <= translation.y + half_size.y)
//                     {
//                         warn!("{}", "Writing!");
//                         writer.write(event::Clicked(entity));
//                     }
//                 }
//             }
//         }
//     }
// }

// fn logic(
//     mut commands: Commands,
//     mut reader: EventReader<event::Clicked>,
//     mut state: ResMut<resource::State>,
//     tiles: Query<&Tile>,
//     sprites: Query<&Sprite>,
// ) {
//     if reader.is_empty() {
//         return;
//     }

//     if let Some(previously_clicked_tile) = state.previously_clicked_tile {
//         info!("Checking if previously clicked tile matches current!");
//         for event in reader.read() {
//             let prev_tile = tiles.get(previously_clicked_tile).unwrap();
//             let curr_tile = tiles.get(event.0).unwrap();

//             if previously_clicked_tile != event.0 && prev_tile.kind == curr_tile.kind {
//                 info!("It did!");
//                 state.previously_clicked_tile = None;
//                 commands
//                     .get_entity(previously_clicked_tile)
//                     .unwrap()
//                     .despawn();
//                 commands.get_entity(event.0).unwrap().despawn();
//             } else {
//                 info!("It did NOT!");
//                 commands
//                     .get_entity(previously_clicked_tile)
//                     .unwrap()
//                     .despawn_related::<Children>();

//                 if let Ok(sprite) = sprites.get(event.0) {
//                     let bloom_id = commands
//                         .spawn((
//                             RenderLayers::layer(0),
//                             Sprite {
//                                 color: Color::srgb(8.0, 8.0, 8.0),
//                                 custom_size: Some(Vec2::new(
//                                     sprite.custom_size.unwrap().x,
//                                     sprite.custom_size.unwrap().y,
//                                 )),
//                                 ..default()
//                             },
//                         ))
//                         .id();
//                     commands.get_entity(event.0).unwrap().add_child(bloom_id);
//                 }

//                 state.previously_clicked_tile = Some(event.0);
//             }
//         }
//     } else {
//         info!("Setting first previously clicked tile!");
//         for event in reader.read() {
//             if let Ok(sprite) = sprites.get(event.0) {
//                 let bloom_id = commands
//                     .spawn((
//                         RenderLayers::layer(0),
//                         Sprite {
//                             color: Color::srgb(8.0, 8.0, 8.0),
//                             custom_size: Some(Vec2::new(
//                                 sprite.custom_size.unwrap().x,
//                                 sprite.custom_size.unwrap().y,
//                             )),
//                             ..default()
//                         },
//                     ))
//                     .id();
//                 commands.get_entity(event.0).unwrap().add_child(bloom_id);
//             }

//             state.previously_clicked_tile = Some(event.0);
//         }
//     }
// }
