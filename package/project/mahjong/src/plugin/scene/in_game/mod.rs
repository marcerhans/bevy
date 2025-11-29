use crate::plugin::scene::main_menu::MainMenu;
use bevy::{prelude::*, window::WindowResized};
use generator::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        use on_enter::*;

        app.add_sub_state::<InGame>()
            .insert_resource(PreviouslySelectedTile(None))
            .add_systems(OnEnter(InGame::Root), spawn_tiles)
            .add_systems(
                Update,
                resize_background_sprite.run_if(in_state(InGame::Root)),
            )
            .add_systems(Update, (rotate, help).run_if(in_state(InGame::Root)));
    }
}

#[derive(SubStates, Default, Debug, Hash, Eq, PartialEq, Clone)]
#[source(MainMenu = MainMenu::Play)]
#[states(scoped_entities)]
enum InGame {
    #[default]
    Root,
}

#[derive(Resource)]
struct PreviouslySelectedTile(pub Option<(Entity, tile::Variant)>);

#[derive(Component)]
struct BackgroundSprite;

#[derive(Component, Clone)]
struct ButtonSprite;

mod tile {
    use bevy::prelude::*;

    #[derive(Component, Clone)]
    pub struct Marker;

    #[derive(Component, Clone, Deref)]
    pub struct Size {
        pub val: Vec2,
    }

    #[derive(Component, Clone, Deref, Debug)]
    pub struct Position {
        pub val: Vec3,
    }

    pub struct Factory {
        texture_tile: Handle<Image>,
        texture_alliance: Handle<Image>,
        texture_horde: Handle<Image>,
        logical_size: Vec2,
        custom_size: Option<Vec2>,
        tile_center_offset: Option<Vec3>,
    }

    #[derive(Component, Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
    pub enum Variant {
        Alliance(usize),
        Horde(usize),
    }

    impl Factory {
        pub fn new(
            texture_tile: Handle<Image>,
            texture_alliance: Handle<Image>,
            texture_horde: Handle<Image>,
            logical_size: Vec2,
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
                logical_size,
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
                Size {
                    val: self.logical_size,
                },
                children![match variant {
                    Variant::Horde(icons) => (
                        Variant::Horde(icons),
                        Text2d::new(icons.to_string()),
                        TextColor::WHITE,
                        TextFont::from_font_size(self.custom_size.unwrap().y / 5.0),
                        Sprite::from_color(Color::BLACK, Vec2::new(50.0, 50.0)),
                        offset,
                    ),
                    Variant::Alliance(icons) => (
                        Variant::Alliance(icons),
                        Text2d::new(icons.to_string()),
                        TextColor::WHITE,
                        TextFont::from_font_size(self.custom_size.unwrap().y / 5.0),
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
        projection: Query<&Projection, With<Camera>>,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
            panic!();
        };

        // Load assets and set texture constants
        // Tile
        let texture_tile: Handle<Image> = asset_server.load("misc/rev2/Tile4_700x1000.png");
        let texture_alliance: Handle<Image> = asset_server.load("misc/rev2/Alliance_1104x882.png");
        let texture_horde: Handle<Image> = asset_server.load("misc/rev2/Horde_740x1093.png");
        const TEXTURE_BOTTOM_BORDER_PERCENTAGE_Y: f32 = 175.0 / 1000.0; // (Just the "thickness" of the tile, excluding the border)
        const TEXTURE_LEFT_BORDER_PERCENTAGE_X: f32 = 124.0 / 700.0; // (Just the "thickness" of the tile, excluding the border)

        // Button
        let texture_button_atlas: Handle<Image> =
            asset_server.load("misc/rev2/button-atlas_1998x429.png");
        let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(666, 429), 3, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        // Background
        let texture_bg: Handle<Image> =
            asset_server.load("misc/rev2/original/Arthas_LichKing_GPT2.png");

        // Determine size and position(s) for tiles
        let tile_height =
            projection.area.height() as f32 / PositionGenerator::<Turtle>::ROWS as f32;
        let tile_height = tile_height + TEXTURE_BOTTOM_BORDER_PERCENTAGE_Y * tile_height; // NOTE:1: This is hacky, but allows for tile height to fill height of window.
        let tile_width = tile_height * 0.7;
        let tile_size = Vec2::new(tile_width, tile_height);
        let tile_center_offset = Vec2::new(
            TEXTURE_LEFT_BORDER_PERCENTAGE_X,
            TEXTURE_BOTTOM_BORDER_PERCENTAGE_Y,
        ) * tile_size
            / 2.0;
        let tile_thickness_offset = Vec2::new(
            TEXTURE_LEFT_BORDER_PERCENTAGE_X,
            TEXTURE_BOTTOM_BORDER_PERCENTAGE_Y,
        ) * tile_size;
        let tile_logical_size = tile_size - tile_thickness_offset * 1.2;
        let tile_factory = tile::Factory::new(
            texture_tile.clone(),
            texture_alliance,
            texture_horde,
            tile_logical_size,
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
                        // Shadow
                        Sprite {
                            custom_size: Some(tile_size.clone().with_y(tile_size.y - 10.0)),
                            color: Color::hsla(
                                0.0,
                                0.0,
                                0.0,
                                if tile_position[variant_index].z == 0.0 {
                                    0.0
                                } else {
                                    0.75
                                },
                            ),
                            ..Sprite::from_image(texture_tile.clone())
                        },
                        Transform {
                            translation: Vec3 {
                                x: tile_thickness_offset.x * 0.4,
                                y: tile_thickness_offset.y * 0.4,
                                z: -10.0,
                            },
                            scale: Vec3 {
                                x: 1.2,
                                y: 1.0,
                                z: 1.0,
                            },
                            ..default()
                        },
                    ))
                    .observe(on_click);
            }
        }

        // Spawn buttons
        struct Button {
            translation: Vec3,
            text: &'static str,
        }
        let button_base = (ButtonSprite, Pickable::default());
        let button_size = Vec2::new(tile_height * 1.5, tile_height * 0.75);
        let button_margin = Vec2::new(5.0, 5.0);
        let button_pos_start = Vec3::new(
            -(tile_size.x - tile_thickness_offset.x) * PositionGenerator::<Turtle>::COLUMNS as f32
                / 2.0
                - tile_size.x / 2.0,
            -(tile_size.y - tile_thickness_offset.y) * PositionGenerator::<Turtle>::ROWS as f32
                / 2.0
                + button_size.y * 0.5,
            999.0,
        );
        let buttons = [
            Button {
                translation: button_pos_start,
                text: "Help (h)",
            },
            Button {
                translation: Vec3 {
                    x: button_pos_start.x + (button_size.x + button_margin.x) * 0.0,
                    y: button_pos_start.y + (button_size.y + button_margin.y) * 1.0,
                    ..button_pos_start
                },
                text: "Rotate (r)",
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
                    Text2d::new(button.text),
                    TextFont {
                        font_size: button_size.y / 5.0,
                        ..default()
                    },
                    TextColor(Color::srgb_u8(255, 215, 0)),
                ))
                .observe(button_over)
                .observe(button_press)
                .observe(button_release)
                .observe(button_out);
        }

        // Spawn background
        commands.spawn((
            BackgroundSprite,
            Sprite {
                custom_size: Some(Vec2::new(projection.area.width(), projection.area.height())),
                color: Color::srgb(1.0, 1.0, 1.0).with_luminance(0.3),
                ..Sprite::from_image(texture_bg)
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
}

fn button_over(
    click: On<Pointer<Over>>,
    mut query: Query<&mut Sprite, With<ButtonSprite>>,
) {
    let Ok(mut sprite) = query.get_mut(click.entity) else {
        panic!();
    };

    let Some(texture_atlas) = sprite.texture_atlas.as_mut() else {
        panic!();
    };

    texture_atlas.index = 1;
}

fn button_press(
    click: On<Pointer<Press>>,
    mut query: Query<&mut Sprite, With<ButtonSprite>>,
) {
    let Ok(mut sprite) = query.get_mut(click.entity) else {
        panic!();
    };

    let Some(texture_atlas) = sprite.texture_atlas.as_mut() else {
        panic!();
    };

    texture_atlas.index = 2;
}

fn button_release(
    click: On<Pointer<Release>>,
    mut query: Query<&mut Sprite, With<ButtonSprite>>,
) {
    let Ok(mut sprite) = query.get_mut(click.entity) else {
        panic!();
    };

    let Some(texture_atlas) = sprite.texture_atlas.as_mut() else {
        panic!();
    };

    texture_atlas.index = 1;
}

fn button_out(
    click: On<Pointer<Out>>,
    mut query: Query<&mut Sprite, With<ButtonSprite>>,
) {
    let Ok(mut sprite) = query.get_mut(click.entity) else {
        panic!();
    };

    let Some(texture_atlas) = sprite.texture_atlas.as_mut() else {
        panic!();
    };

    texture_atlas.index = 0;
}

fn on_click(
    click: On<Pointer<Press>>,
    mut commands: Commands,
    children: Query<&Children>,
    variants: Query<&tile::Variant>,
    mut tile_query: Query<
        (Entity, &mut tile::Position, &mut tile::Size, &mut Sprite),
        With<tile::Marker>,
    >,
    mut prev_tile: ResMut<PreviouslySelectedTile>,
) {
    let Ok(children) = children.get(click.entity) else {
        info!("Clicked entity is missing children");
        return;
    };

    let mut variant = None;
    for &child in children {
        if let Ok(variant_) = variants.get(child) {
            variant = Some(variant_);
            break;
        }
    }
    let variant = variant.unwrap();
    // let id = match variant {
    //     tile::Variant::Horde(id) | tile::Variant::Alliance(id) => id,
    // };
    // info!("{}", id);

    // Update appearance of selected tile (and restore previous)
    let mut e = tile_query.get_mut(click.entity).unwrap();
    e.3.color = Color::hsl(0.0, 0.0, 1.5);

    if let Some(prev_tile) = &mut prev_tile.0 {
        if prev_tile.0 != click.entity {
            // Restore previous tile
            let mut e = tile_query.get_mut(prev_tile.0).unwrap();
            e.3.color = Color::hsl(0.0, 0.0, 1.0);
        }
    }

    let Some((prev_entity, prev_variant)) = &mut prev_tile.0 else {
        prev_tile.0 = Some((click.entity, variant.clone()));
        return;
    };

    info!(
        "Prev: {:?} | Now: {:?}",
        prev_variant.clone(),
        variant.clone()
    );

    if *prev_entity != click.entity && *prev_variant == *variant {
        info!("It's a match!");

        if rule_check(
            prev_entity,
            prev_variant,
            &click.entity,
            variant,
            &tile_query
                .transmute_lens_filtered::<(Entity, &mut tile::Position, &mut tile::Size), With<tile::Marker>>()
                .query(),
        ) {
            commands.entity(*prev_entity).despawn();
            commands.entity(click.entity).despawn();
            prev_tile.0 = None;
        } else {
            info!("Failed rule check!");
            prev_tile.0 = Some((click.entity, variant.clone()));
        }
    } else {
        prev_tile.0 = Some((click.entity, variant.clone()));
    }
}

fn rule_check(
    prev_entity: &Entity,
    prev_variant: &tile::Variant,
    this_entity: &Entity,
    this_variant: &tile::Variant,
    tile_query: &Query<(Entity, &mut tile::Position, &mut tile::Size), With<tile::Marker>>,
) -> bool {
    fn are_intersecting(
        a_center: &Vec2,
        a_size: &Vec2,
        b_center: &Vec2,
        b_size: &Vec2,
    ) -> bool {
        let a_half = a_size * 0.5;
        let b_half = b_size * 0.5;

        let d = (b_center - a_center).abs();
        let allowed = a_half + b_half;

        d.x < allowed.x && d.y < allowed.y
    }

    fn tile_is_obscured(
        tile_position: &Vec3,
        tile_size: &tile::Size,
        tile_query: &Query<(Entity, &mut tile::Position, &mut tile::Size), With<tile::Marker>>,
    ) -> bool {
        tile_query.iter().any(|(_, position, size)| {
            are_intersecting(
                &tile_position.truncate(),
                &tile_size,
                &position.truncate(),
                &size,
            ) && tile_position.z < position.z
        })
    }

    /// "Open" being, not a tile to the left AND right of it.
    fn tile_is_open(
        tile_entity: Entity,
        tile_position: &Vec3,
        tile_size: &tile::Size,
        tile_query: &Query<(Entity, &mut tile::Position, &mut tile::Size), With<tile::Marker>>,
    ) -> bool {
        // (Same row counts as being within half another block along the y-axis.)
        let mut tiles_on_same_layer_and_row = tile_query.iter().filter(|(entity, pos, size)| {
            let not_same_entity = tile_entity != *entity;
            let same_layer = tile_position.z == pos.z;
            let same_row = {
                let a_half = tile_size.y / 2.0;
                let b_half = size.y / 2.0;
                let d = (tile_position.y - pos.y).abs();
                let allowed = a_half + b_half;
                d < allowed
            };
            not_same_entity && same_layer && same_row
        });
        !tiles_on_same_layer_and_row
            .clone()
            .any(|(_, pos, _)| pos.x < tile_position.x)
            || !tiles_on_same_layer_and_row.any(|(_, pos, _)| pos.x > tile_position.x)
    }

    let prev_tile = tile_query.get(*prev_entity).unwrap();
    let this_tile = tile_query.get(*this_entity).unwrap();

    if tile_is_obscured(&prev_tile.1, prev_tile.2, tile_query)
        || tile_is_obscured(&this_tile.1, this_tile.2, tile_query)
    {
        info!("One of the tiles is obscured.");
        return false;
    }

    if !tile_is_open(prev_tile.0, &prev_tile.1, &prev_tile.2, tile_query)
        || !tile_is_open(this_tile.0, &this_tile.1, &this_tile.2, tile_query)
    {
        info!("One of the tiles is not open.");
        return false;
    }

    true
}

fn resize_background_sprite(
    mut transform: Query<(&mut Transform, &mut Sprite), With<BackgroundSprite>>,
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

fn rotate(children: Query<&Children>) {
    info!("todo");
}

fn help() {
    info!("todo");
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
