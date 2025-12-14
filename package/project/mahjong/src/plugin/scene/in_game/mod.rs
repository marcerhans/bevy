use std::collections::VecDeque;

use crate::plugin::scene::main_menu::MainMenu;
use bevy::prelude::*;
use generator::*;
use rand::seq::SliceRandom;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        use on_enter::*;

        app.add_sub_state::<InGame>()
            .add_message::<OnClick>()
            .add_message::<msg::Shuffle>()
            .add_message::<msg::Help>()
            .add_message::<msg::Undo>()
            .insert_resource(PreviouslySelectedTile(None))
            .insert_resource(History::default())
            .add_systems(OnEnter(InGame::Root), spawn_tiles)
            .add_systems(
                Update,
                resize_background_sprite.run_if(in_state(InGame::Root)),
            )
            .add_systems(
                Update,
                (
                    on_click,
                    shuffle.run_if(run_shuffle),
                    shuffle_button,
                    help.run_if(run_help),
                    help_button,
                    undo.run_if(run_undo),
                    undo_button,
                )
                    .chain()
                    .run_if(in_state(InGame::Root)),
            );
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

enum Undo {
    Pair((Entity, f32), (Entity, f32)),
    Shuffle(Vec<(Entity, tile::Variant, Option<Text2d>)>),
}

#[derive(Resource, Deref, DerefMut, Default)]
struct History(VecDeque<Undo>);

impl History {
    const SIZE: usize = 32;
}

#[derive(Message, Deref, DerefMut)]
struct OnClick(Entity);

#[derive(Component)]
struct BackgroundSprite;

#[derive(Component, Clone)]
struct ButtonSprite;

mod msg {
    use bevy::prelude::*;

    #[derive(Message)]
    pub struct Shuffle;

    #[derive(Message)]
    pub struct Help;

    #[derive(Message)]
    pub struct Undo;
}

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

    #[derive(Component, Clone)]
    pub struct Inactive;

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
                        Marker,
                        Variant::Horde(icons),
                        Text2d::new(icons.to_string()),
                        TextColor::WHITE,
                        TextFont::from_font_size(self.custom_size.unwrap().y / 5.0),
                        Sprite::from_color(Color::BLACK, Vec2::new(50.0, 50.0)),
                        offset,
                    ),
                    Variant::Alliance(icons) => (
                        Marker,
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

    pub fn spawn_tile_with_extras(
        commands: &mut Commands,
        tile_factory: &Factory,
        tile_size: &Vec2,
        tile_thickness_offset: &Vec2,
        index: usize,
        position: &Position,
        translation: Vec3,
        texture: Handle<Image>,
    ) {
        commands
            .spawn((
                DespawnOnExit(super::InGame::Root),
                Pickable::default(),
                tile_factory.get_tile(Variant::Horde(index), None),
                position.clone(),
                Transform {
                    translation,
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
                        if position.val.z == 0.0 { 0.0 } else { 0.75 },
                    ),
                    ..Sprite::from_image(texture)
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
            .observe(
                |on_press: On<Pointer<Press>>, mut msg: MessageWriter<super::OnClick>| {
                    msg.write(super::OnClick(on_press.entity));
                },
            );
    }
}

mod on_enter {
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
        let tvs = PositionGenerator::<Turtle>::TILE_VARIANT_SIZE;

        for (index, tile_position) in tile_positions.windows(tvs).step_by(tvs).enumerate() {
            for variant_index in 0..tvs {
                let column_index = tile_position[variant_index].x / tile_size.x;
                let row_index = tile_position[variant_index].y / tile_size.y;

                tile::spawn_tile_with_extras(
                    &mut commands,
                    &tile_factory,
                    &tile_size,
                    &tile_thickness_offset,
                    index,
                    &tile::Position {
                        val: Vec3 {
                            x: tile_position[variant_index].x,
                            y: tile_position[variant_index].y,
                            z: tile_position[variant_index].z,
                        },
                    },
                    Vec3 {
                        x: tile_position[variant_index].x
                            - (column_index * tile_thickness_offset.x)
                            + (tile_position[variant_index].z * tile_thickness_offset.x),
                        y: tile_position[variant_index].y - (row_index * tile_thickness_offset.y)
                            + (tile_position[variant_index].z * tile_thickness_offset.y)
                            - tile_height * 0.5, // NOTE:2: See NOTE:1 - This is simply to adjust the offset
                        z: tile_position[variant_index].z * 100.0 - column_index - row_index as f32,
                    },
                    texture_tile.clone(),
                );
            }
        }

        // Spawn buttons
        struct Button {
            translation: Vec3,
            text: &'static str,
        }
        let button_base = (
            DespawnOnExit(InGame::Root),
            ButtonSprite,
            Pickable::default(),
        );
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
        let button_pos_start_right = Vec3::new(
            (tile_size.x - tile_thickness_offset.x) * PositionGenerator::<Turtle>::COLUMNS as f32
                / 2.0
                + tile_size.x / 2.0,
            -(tile_size.y - tile_thickness_offset.y) * PositionGenerator::<Turtle>::ROWS as f32
                / 2.0
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
                ))
                .observe(button_over)
                .observe(button_press)
                .observe(button_release)
                .observe(button_out);
        }

        // Spawn background
        commands.spawn((
            DespawnOnExit(InGame::Root),
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
    mut query: Query<&mut Sprite, (Without<tile::Marker>, With<ButtonSprite>)>,
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
    mut query: Query<&mut Sprite, (Without<tile::Marker>, With<ButtonSprite>)>,
    children: Query<&Children, (Without<tile::Marker>, With<ButtonSprite>)>,
    text2ds: Query<&Text2d, (Without<tile::Marker>, With<ButtonSprite>)>,
    mut msg_shuffle: MessageWriter<msg::Shuffle>,
    mut msg_help: MessageWriter<msg::Help>,
    mut msg_undo: MessageWriter<msg::Undo>,
) {
    let Ok(mut sprite) = query.get_mut(click.entity) else {
        panic!();
    };

    let Some(texture_atlas) = sprite.texture_atlas.as_mut() else {
        panic!();
    };

    texture_atlas.index = 2;

    let Ok(children) = children.get(click.entity) else {
        panic!();
    };

    let mut text2d = None;
    for &child in children {
        if let Ok(text) = text2ds.get(child) {
            text2d = Some(text);
            break;
        }
    }

    match text2d.unwrap().as_str() {
        "[S]huffle" => {
            msg_shuffle.write(msg::Shuffle);
        },
        "[H]elp" => {
            msg_help.write(msg::Help);
        },
        "  [U]ndo\n(limit: 32)" => {
            msg_undo.write(msg::Undo);
        },
        _ => panic!(),
    }
}

fn button_release(
    click: On<Pointer<Release>>,
    mut query: Query<&mut Sprite, (Without<tile::Marker>, With<ButtonSprite>)>,
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
    mut query: Query<&mut Sprite, (Without<tile::Marker>, With<ButtonSprite>)>,
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
    mut msg_onclick: MessageReader<OnClick>,
    mut commands: Commands,
    children: Query<&Children>,
    variants: Query<&tile::Variant>,
    mut tile_query: Query<
        (
            Entity,
            &mut tile::Position,
            &mut tile::Size,
            &mut Sprite,
            &mut Transform,
        ),
        (Without<tile::Inactive>, With<tile::Marker>),
    >,
    mut prev_tile: ResMut<PreviouslySelectedTile>,
    mut history: ResMut<History>,
) {
    let Some(origin) = msg_onclick.read().next() else {
        return;
    };
    let origin = **origin;

    // Sanity check.
    if let Err(err) = commands.get_entity(origin) {
        warn!("Entity does not exist. Error: {:?}", err);
        return;
    }

    // Fetch the entities [tile::Variant] (nested inside a child entity).
    let children = match children.get(origin) {
        Ok(children) => children,
        Err(err) => {
            warn!(
                "Could not fetch children of origin entity. Error: {:?}",
                err
            );
            return;
        },
    };

    let mut variant = None;
    for &child in children {
        if let Ok(variant_) = variants.get(child) {
            variant = Some(variant_);
            break;
        }
    }
    let Some(variant) = variant else {
        warn!("Could not fetch variant of (origins) child entity.",);
        return;
    };

    // Update appearance of selected tile.
    let mut current_tile = match tile_query.get_mut(origin) {
        Ok(entity) => entity,
        Err(err) => {
            warn!(
                "Could not fetch tile query entity based on origin entity. Error: {:?}",
                err
            );
            return;
        },
    };
    current_tile.3.color = Color::hsl(0.0, 0.0, 1.5);

    // Restore appearance of previous tile.
    if let Some(prev_tile) = &mut prev_tile.0 {
        if prev_tile.0 != origin {
            // Restore previous tile
            let mut e = tile_query.get_mut(prev_tile.0).unwrap();
            e.3.color = Color::hsl(0.0, 0.0, 1.0);
        }
    }

    // Set previous tile if currently non-existant.
    let Some((prev_entity, prev_variant)) = &mut prev_tile.0 else {
        // (We needed the variant of the entity before we can do this, hence why it is not the first step).
        prev_tile.0 = Some((origin, variant.clone()));
        return;
    };

    info!(
        "Prev: {:?} | Now: {:?}",
        prev_variant.clone(),
        variant.clone()
    );

    if *prev_entity != origin && *prev_variant == *variant {
        info!("It's a match!");

        if rule_check(
            prev_entity,
            prev_variant,
            &origin,
            variant,
            &tile_query
                .transmute_lens_filtered::<(Entity, &tile::Position, &tile::Size), With<tile::Marker>>()
                .query(),
        ) {
            commands.entity(*prev_entity).insert(tile::Inactive);
            commands.entity(origin).insert(tile::Inactive);

            let e1z = tile_query.get_mut(*prev_entity).unwrap().4.translation.z;
            let e2z= tile_query.get_mut(origin).unwrap().4.translation.z;

            history.push_back(Undo::Pair((*prev_entity, e1z), (origin, e2z)));
            if history.len() > History::SIZE {
                history.pop_front();
            }

            let mut e1 = tile_query.get_mut(*prev_entity).unwrap();
            e1.4.translation.z = -1000.0;
            let mut e2 = tile_query.get_mut(origin).unwrap();
            e2.4.translation.z = -1000.0;

            prev_tile.0 = None;
        } else {
            info!("Failed rule check!");
            prev_tile.0 = Some((origin, variant.clone()));
        }
    } else {
        prev_tile.0 = Some((origin, variant.clone()));
    }
}

fn rule_check(
    prev_entity: &Entity,
    prev_variant: &tile::Variant,
    this_entity: &Entity,
    this_variant: &tile::Variant,
    tile_query: &Query<(Entity, &tile::Position, &tile::Size), With<tile::Marker>>,
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
        tile_query: &Query<(Entity, &tile::Position, &tile::Size), With<tile::Marker>>,
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
        tile_query: &Query<(Entity, &tile::Position, &tile::Size), With<tile::Marker>>,
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

fn shuffle_button(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut msg: MessageWriter<msg::Shuffle>,
) {
    if keyboard.just_pressed(KeyCode::KeyS) {
        msg.write(msg::Shuffle);
    }
}

fn run_shuffle(mut msg: MessageReader<msg::Shuffle>) -> bool {
    let run = msg.read().len() > 0;
    msg.clear();
    run
}

fn shuffle(
    mut query: Query<
        (Entity, &mut tile::Variant, &mut Text2d),
        (Without<tile::Inactive>, With<tile::Marker>),
    >,
    mut history: ResMut<History>,
) {
    let mut rng = rand::rng();
    let query_len = query.iter().len();
    let mut variant_text = Vec::with_capacity(query_len);

    for (_, variant, text2d) in &query {
        variant_text.push((variant.clone(), text2d.clone()));
    }

    // Save for history
    let mut history_shuffle_vec = vec![];
    for (entity, variant, text2d) in query.iter() {
        history_shuffle_vec.push((entity, variant.clone(), Some(text2d.clone())));
    }

    history.push_back(Undo::Shuffle(history_shuffle_vec));
    if history.len() > History::SIZE {
        history.pop_front();
    }

    // Apply shuffle
    variant_text.shuffle(&mut rng);
    for (index, (_, mut variant, mut text2d)) in query.iter_mut().enumerate() {
        *variant = variant_text[index].0.clone();
        *text2d = variant_text[index].1.clone();
    }
}

fn help_button(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut msg: MessageWriter<msg::Help>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        msg.write(msg::Help);
    }
}

fn run_help(mut msg: MessageReader<msg::Help>) -> bool {
    let run = msg.read().len() > 0;
    msg.clear();
    run
}

fn help(// tile_query: Query<(Entity, &tile::Position, &tile::Size), With<tile::Marker>>,
    // children: Query<&Children, With<tile::Variant>>,
) {
    // for (entity, position, size) in tile_query {
    //     for (entity_, position_, size_) in tile_query {
    //         // rule_check()
    //         todo!();
    //     }
    // }

    info!("help pressed!");
}

fn undo_button(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut msg: MessageWriter<msg::Undo>,
) {
    if keyboard.just_pressed(KeyCode::KeyU) {
        msg.write(msg::Undo);
    }
}

fn run_undo(mut msg: MessageReader<msg::Undo>) -> bool {
    let run = msg.read().len() > 0;
    msg.clear();
    run
}

fn undo(
    mut commands: Commands,
    mut history: ResMut<History>,
    mut query_pair: Query<(&mut Transform, &mut Sprite), With<tile::Inactive>>,
) {
    if let Some(history_item) = history.pop_back() {
        match history_item {
            Undo::Pair((entity1, z1), (entity2, z2)) => {
                commands.entity(entity1).remove::<tile::Inactive>();
                commands.entity(entity2).remove::<tile::Inactive>();

                query_pair.get_mut(entity1).unwrap().0.translation.z = z1;
                query_pair.get_mut(entity2).unwrap().0.translation.z = z2;

                query_pair.get_mut(entity1).unwrap().1.color = Color::hsl(0.0, 0.0, 1.0);
                query_pair.get_mut(entity2).unwrap().1.color = Color::hsl(0.0, 0.0, 1.0);
            },
            Undo::Shuffle(items) => {
                for (entity, variant, text2d) in items {
                    commands.entity(entity).insert(variant);
                    commands.entity(entity).insert(text2d.unwrap());
                }
            },
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

mod model {
    pub mod bevy {
        pub mod tile {
            use bevy::prelude::*;
        }
    }

    pub mod grid {
        use bevy::math::{UVec2, Vec2};
        use std::{
            cell::{Ref, RefCell, RefMut},
            ops::{Index, IndexMut},
            rc::Rc,
        };

        pub type LayerOffset = Vec2;

        pub trait OccupantTrait {}
        impl<T> OccupantTrait for T {}

        #[derive(PartialEq, Debug)]
        pub struct OccupantWrapper<Occupant: OccupantTrait> {
            pub origin: UVec2,
            pub size: UVec2,
            pub occupant: Option<Occupant>,
        }

        pub struct Cell<Occupant: OccupantTrait> {
            occupant_wrapper: Option<Rc<RefCell<OccupantWrapper<Occupant>>>>,
        }

        pub type Layer<Occupant, const ROWS: usize, const COLUMNS: usize> =
            [[Cell<Occupant>; COLUMNS]; ROWS];

        /// A [Grid] where single entities can occupy more than one index.
        pub struct Grid<
            Occupant: OccupantTrait,
            const LAYERS: usize,
            const ROWS: usize,
            const COLUMNS: usize,
        > {
            occupied: [(LayerOffset, Layer<Occupant, ROWS, COLUMNS>); LAYERS],
        }

        impl<Occupant: OccupantTrait, const LAYERS: usize, const ROWS: usize, const COLUMNS: usize>
            std::fmt::Debug for Grid<Occupant, LAYERS, ROWS, COLUMNS>
        {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                writeln!(f, "GRID:{LAYERS}x{ROWS}x{COLUMNS}")?;

                for layer in 0..LAYERS {
                    writeln!(f, "Layer {}", &layer)?;

                    for row in (0..ROWS).rev() {
                        for column in 0..COLUMNS {
                            if let Some(_) = self.occupied[layer].1[row][column].occupant_wrapper {
                                write!(f, "[x]")?;
                            } else {
                                write!(f, "[ ]")?;
                            }
                        }
                        writeln!(f, "")?;
                    }
                }

                Ok(())
            }
        }

        impl<Occupant: OccupantTrait, const LAYERS: usize, const ROWS: usize, const COLUMNS: usize>
            Grid<Occupant, LAYERS, ROWS, COLUMNS>
        {
            pub fn new(layer_offsets: Option<[LayerOffset; LAYERS]>) -> Self {
                let layer_offsets = match layer_offsets {
                    Some(layer_offsets) => layer_offsets,
                    None => [LayerOffset::default(); LAYERS],
                };

                let occupied: [(LayerOffset, Layer<Occupant, ROWS, COLUMNS>); LAYERS] =
                    std::array::from_fn(|index| {
                        let layer = std::array::from_fn(|_| {
                            std::array::from_fn(|_| Cell {
                                occupant_wrapper: None,
                            })
                        });
                        (layer_offsets[index], layer)
                    });

                Self { occupied }
            }

            pub fn get_dimensions() -> (usize, usize, usize) {
                (LAYERS, ROWS, COLUMNS)
            }

            pub fn get_layer_offset(
                &self,
                layer: usize,
            ) -> LayerOffset {
                self[layer].0
            }

            pub fn get_list_of_occupants(
                &self,
                layer: usize,
                row: usize,
                column: usize,
                size: UVec2,
            ) -> Vec<Rc<RefCell<OccupantWrapper<Occupant>>>> {
                let mut list = vec![];

                let row_end = row + size.y as usize;
                let column_end = column + size.x as usize;

                for row in row..=row_end {
                    for column in column..=column_end {
                        if let Some(occupant_wrapper) =
                            &self.occupied[layer].1[row][column].occupant_wrapper
                        {
                            if !list
                                .iter()
                                .any(|item: &Rc<RefCell<OccupantWrapper<Occupant>>>| {
                                    Rc::ptr_eq(item, occupant_wrapper)
                                })
                            {
                                list.push(Rc::clone(occupant_wrapper));
                            }
                        }
                    }
                }

                list
            }

            /// Set the value of a [Cell].
            /// If outside bounds -> [Result::Err].
            /// If a single or multiple tiles already exist in the bounds of the new tile -> [Result::Ok] with [Some] containing a list of the removed/replaced tiles.
            /// Else [Result::Ok] with [None].
            pub fn set(
                &mut self,
                layer: usize,
                row: usize,
                column: usize,
                occupant: Occupant,
                size: UVec2,
            ) -> Result<Option<Vec<Rc<RefCell<OccupantWrapper<Occupant>>>>>, ()> {
                for row in row..row + size.y as usize {
                    for column in column..column + size.x as usize {
                        if !Self::is_within_bounds(layer, row, column) {
                            return Err(());
                        }
                    }
                }

                let current_occupants = self.get_list_of_occupants(layer, row, column, size);
                for occupant in &current_occupants {
                    self.remove_cells(layer, Rc::clone(occupant));
                }
                let removed_occupants = current_occupants;

                let occupant_wrapper = Rc::new(RefCell::new(OccupantWrapper {
                    origin: UVec2 {
                        x: column as u32,
                        y: row as u32,
                    },
                    size,
                    occupant: Some(occupant),
                }));
                self.set_cells(layer, occupant_wrapper);

                if removed_occupants.len() > 0 {
                    return Ok(Some(removed_occupants));
                }

                Ok(None)
            }

            pub fn get(
                &self,
                layer: usize,
                row: usize,
                column: usize,
            ) -> Option<Ref<'_, Occupant>> {
                if !Self::is_within_bounds(layer, row, column) {
                    return None;
                }

                if let Some(occupant_wrapper) = &self[layer].1[row][column].occupant_wrapper {
                    let occupant_wrapper = occupant_wrapper.borrow();
                    return Some(Ref::map(occupant_wrapper, |occupant_wrapper| {
                        occupant_wrapper.occupant.as_ref().unwrap()
                    }));
                }

                None
            }

            pub fn get_mut(
                &self,
                layer: usize,
                row: usize,
                column: usize,
            ) -> Option<RefMut<'_, Occupant>> {
                if !Self::is_within_bounds(layer, row, column) {
                    return None;
                }

                if let Some(occupant_wrapper) = &self[layer].1[row][column].occupant_wrapper {
                    let occupant_wrapper = occupant_wrapper.borrow_mut();
                    return Some(RefMut::map(occupant_wrapper, |occupant_wrapper| {
                        occupant_wrapper.occupant.as_mut().unwrap()
                    }));
                }

                None
            }

            pub fn remove(
                &mut self,
                layer: usize,
                row: usize,
                column: usize,
            ) -> Option<Occupant> {
                if !Self::is_within_bounds(layer, row, column) {
                    return None;
                }

                let Some(occupant_wrapper) = &self[layer].1[row][column].occupant_wrapper else {
                    return None;
                };

                let occupant = occupant_wrapper.borrow_mut().occupant.take();
                self.remove_cells(layer, Rc::clone(occupant_wrapper));
                occupant
            }

            pub fn is_within_bounds(
                layer: usize,
                row: usize,
                column: usize,
            ) -> bool {
                layer < LAYERS && row < ROWS && column < COLUMNS
            }

            fn set_cells(
                &mut self,
                layer: usize,
                occupant_wrapper: Rc<RefCell<OccupantWrapper<Occupant>>>,
            ) {
                let occupant_wrapper_ = occupant_wrapper.borrow();
                let row = occupant_wrapper_.origin.y as usize;
                let column = occupant_wrapper_.origin.x as usize;
                let row_end = row + occupant_wrapper_.size.y as usize;
                let column_end = column + occupant_wrapper_.size.x as usize;

                for row in row..=row_end {
                    for column in column..=column_end {
                        if !Self::is_within_bounds(layer, row, column) {
                            panic!();
                        }
                        self.occupied[layer].1[row][column].occupant_wrapper =
                            Some(Rc::clone(&occupant_wrapper));
                    }
                }
            }

            fn remove_cells(
                &mut self,
                layer: usize,
                occupant_wrapper: Rc<RefCell<OccupantWrapper<Occupant>>>,
            ) {
                let occupant_wrapper = occupant_wrapper.borrow();
                let row = occupant_wrapper.origin.y as usize;
                let column = occupant_wrapper.origin.x as usize;
                let row_end = row + occupant_wrapper.size.y as usize;
                let column_end = column + occupant_wrapper.size.x as usize;

                for row in row..=row_end {
                    for column in column..=column_end {
                        if !Self::is_within_bounds(layer, row, column) {
                            panic!();
                        }
                        self.occupied[layer].1[row][column].occupant_wrapper = None;
                    }
                }
            }
        }

        impl<Occupant: OccupantTrait, const LAYERS: usize, const ROWS: usize, const COLUMNS: usize>
            Index<usize> for Grid<Occupant, LAYERS, ROWS, COLUMNS>
        {
            type Output = (LayerOffset, Layer<Occupant, ROWS, COLUMNS>);

            fn index(
                &self,
                layer: usize,
            ) -> &Self::Output {
                &self.occupied[layer]
            }
        }

        impl<Occupant: OccupantTrait, const LAYERS: usize, const ROWS: usize, const COLUMNS: usize>
            IndexMut<usize> for Grid<Occupant, LAYERS, ROWS, COLUMNS>
        {
            fn index_mut(
                &mut self,
                layer: usize,
            ) -> &mut Self::Output {
                &mut self.occupied[layer]
            }
        }

        pub struct GridIteratorRef<
            'a,
            Occupant: OccupantTrait,
            const LAYERS: usize,
            const ROWS: usize,
            const COLUMNS: usize,
        > {
            grid: &'a Grid<Occupant, LAYERS, ROWS, COLUMNS>,
            layer: usize,
            row: usize,
            column: usize,
        }

        impl<
            'a,
            Occupant: OccupantTrait,
            const LAYERS: usize,
            const ROWS: usize,
            const COLUMNS: usize,
        > Iterator for GridIteratorRef<'a, Occupant, LAYERS, ROWS, COLUMNS>
        {
            type Item = Ref<'a, Occupant>;

            fn next(&mut self) -> Option<Self::Item> {
                self.column += 1;

                if self.column >= COLUMNS {
                    self.column = 0;
                    self.row += 1;
                }

                if self.row >= ROWS {
                    self.row = 0;
                    self.layer += 1;
                }

                self.grid.get(self.layer, self.row, self.column)
            }
        }

        impl<
            'a,
            Occupant: OccupantTrait,
            const LAYERS: usize,
            const ROWS: usize,
            const COLUMNS: usize,
        > IntoIterator for &'a Grid<Occupant, LAYERS, ROWS, COLUMNS>
        {
            type Item = Ref<'a, Occupant>;

            type IntoIter = GridIteratorRef<'a, Occupant, LAYERS, ROWS, COLUMNS>;

            fn into_iter(self) -> Self::IntoIter {
                Self::IntoIter {
                    grid: self,
                    layer: 0,
                    row: 0,
                    column: 0,
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            mod set {
                use super::*;

                #[test]
                fn set_single() {
                    let mut grid = Grid::<usize, 1, 2, 2>::new(None);

                    // Set
                    let row = 0;
                    let col = 0;
                    let size = UVec2::new(0, 0);
                    assert_eq!(grid.set(0, row, col, 1, size), Ok(None));

                    let occupant_wrapper = grid.occupied[0].1[row][col]
                        .occupant_wrapper
                        .as_ref()
                        .unwrap()
                        .borrow();
                    assert_eq!(occupant_wrapper.origin, UVec2::new(col as u32, row as u32));
                    assert_eq!(occupant_wrapper.size, size);
                    assert_eq!(occupant_wrapper.occupant, Some(1));

                    println!("{:?}", grid);
                }

                #[test]
                fn set_sized() {
                    let mut grid = Grid::<bool, 2, 3, 4>::new(None);

                    // Set
                    let row = 1;
                    let col = 2;
                    let size = UVec2::new(1, 1);
                    assert_eq!(grid.set(1, row, col, true, size), Ok(None));

                    let to_be_checked = [
                        UVec2::new(2, 1),
                        UVec2::new(2, 2),
                        UVec2::new(3, 1),
                        UVec2::new(3, 2),
                    ];

                    for pos in to_be_checked {
                        let occupant_wrapper = grid.occupied[1].1[pos.y as usize][pos.x as usize]
                            .occupant_wrapper
                            .as_ref()
                            .unwrap()
                            .borrow();
                        assert_eq!(occupant_wrapper.origin, UVec2::new(col as u32, row as u32));
                        assert_eq!(occupant_wrapper.size, size);
                        assert_eq!(occupant_wrapper.occupant, Some(true));
                    }

                    println!("{:?}", grid);
                }

                #[test]
                fn set_and_replace() {
                    let mut grid = Grid::<usize, 1, 16, 16>::new(None);

                    // Set
                    let to_set = [
                        (0, UVec2::new(0, 0), UVec2::new(6, 6)),
                        (1, UVec2::new(7, 7), UVec2::new(7, 7)),
                        (2, UVec2::new(7, 15), UVec2::new(8, 0)),
                        (3, UVec2::new(6, 7), UVec2::new(0, 8)),
                        (4, UVec2::new(7, 6), UVec2::new(8, 0)),
                    ];

                    for (index, rowcol, size) in &to_set {
                        let row = rowcol.y as usize;
                        let col = rowcol.x as usize;
                        assert_eq!(grid.set(0, row, col, *index, *size), Ok(None));
                        println!("{:?}", grid);
                    }

                    let to_replace_with = [
                        (UVec2::new(1, 1), UVec2::new(3, 3)),
                        (UVec2::new(8, 6), UVec2::new(4, 6)),
                    ];

                    let expected_removed_occupants = [to_set[0], to_set[1], to_set[4]];

                    for (index, (rowcol, size)) in to_replace_with.iter().enumerate() {
                        let row = rowcol.y as usize;
                        let col = rowcol.x as usize;
                        let set_result = grid.set(0, row, col, 10, *size);
                        let set_removed_cells = set_result.unwrap().unwrap();

                        // This is ugly T.T
                        if index == 0 {
                            for cell in set_removed_cells {
                                assert_eq!(
                                    cell.borrow().occupant,
                                    Some(expected_removed_occupants[0].0)
                                );
                                assert_eq!(cell.borrow().origin, expected_removed_occupants[0].1);
                                assert_eq!(cell.borrow().size, expected_removed_occupants[0].2);
                            }
                        } else {
                            for (index, cell) in set_removed_cells.iter().enumerate() {
                                if index == 0 {
                                    assert_eq!(
                                        cell.borrow().occupant,
                                        Some(expected_removed_occupants[2].0)
                                    );
                                    assert_eq!(
                                        cell.borrow().origin,
                                        expected_removed_occupants[2].1
                                    );
                                    assert_eq!(cell.borrow().size, expected_removed_occupants[2].2);
                                } else {
                                    assert_eq!(
                                        cell.borrow().occupant,
                                        Some(expected_removed_occupants[1].0)
                                    );
                                    assert_eq!(
                                        cell.borrow().origin,
                                        expected_removed_occupants[1].1
                                    );
                                    assert_eq!(cell.borrow().size, expected_removed_occupants[1].2);
                                }
                            }
                        }

                        println!("{:?}", grid);
                    }
                }
            }

            mod get {
                use super::*;

                #[test]
                fn test() {
                    let mut grid = Grid::<bool, 1, 2, 2>::new(None);
                    assert!(grid.set(0, 0, 0, true, UVec2::splat(0)).unwrap().is_none());

                    // Get
                    assert_eq!(*grid.get(0, 0, 0).unwrap(), true);
                    assert!(grid.get(0, 0, 1).is_none());

                    // Get Mut
                    *grid.get_mut(0, 0, 0).unwrap() = false;
                    assert_eq!(*grid.get(0, 0, 0).unwrap(), false);
                }

                #[test]
                fn bounds_check0() {
                    let grid = Grid::<bool, 1, 2, 2>::new(None);
                    assert!(grid.get(2, 0, 0).is_none());
                }
            }

            mod remove {
                use super::*;

                #[test]
                fn test() {
                    let mut grid = Grid::<usize, 1, 3, 3>::new(None);
                    assert_eq!(grid.set(0, 0, 0, 42, UVec2::splat(1)), Ok(None));
                    assert_eq!(grid.set(0, 2, 2, 1, UVec2::splat(0)), Ok(None));
                    println!("{:?}", grid);
                    assert_eq!(grid.remove(0, 0, 0), Some(42));
                    println!("{:?}", grid);
                }
            }

            mod layer {
                use super::*;

                #[test]
                fn test() {
                    const LAYERS: usize = 3;
                    const ROWS: usize = 1;
                    const COLUMNS: usize = 1;

                    let offsets = [
                        LayerOffset { x: 0.0, y: 0.0 },
                        LayerOffset { x: 1.0, y: 1.0 },
                        LayerOffset { x: 1.5, y: 1.5 },
                    ];

                    let grid = Grid::<(), LAYERS, ROWS, COLUMNS>::new(Some([
                        offsets[0], offsets[1], offsets[2],
                    ]));

                    for i in 0..LAYERS {
                        assert_eq!(grid.get_layer_offset(i), offsets[i]);
                    }
                }
            }
        }
    }
}

mod view {
    use super::model::grid::*;
    use bevy::prelude::*;

    pub trait ModelToViewData<'a, ViewData> {
        type Context;

        fn convert(
            &self,
            context: Option<&'a Self::Context>,
        ) -> ViewData;
    }

    pub struct TileContext {
        size: Vec2,
    }

    type TilePositions<const LAYERS: usize, const ROWS: usize, const COLUMNS: usize> =
        [[[Option<Vec2>; COLUMNS]; ROWS]; LAYERS];

    impl<'a, Occupant: OccupantTrait, const LAYERS: usize, const ROWS: usize, const COLUMNS: usize>
        ModelToViewData<'a, TilePositions<LAYERS, ROWS, COLUMNS>>
        for Grid<Occupant, LAYERS, ROWS, COLUMNS>
    {
        type Context = TileContext;

        fn convert(
            &self,
            context: Option<&'a Self::Context>,
        ) -> TilePositions<LAYERS, ROWS, COLUMNS> {
            let Some(context) = context else {
                panic!("This conversion demands a Self::Context!")
            };

            let mut tile_positions = [[[None; COLUMNS]; ROWS]; LAYERS];

            for layer in 0..LAYERS {
                for row in 0..ROWS {
                    for column in 0..COLUMNS {
                        if let Some(_) = self.get(layer, row, column) {
                            tile_positions[layer][row][column] = Some(Vec2 {
                                x: context.size.x * column as f32 + self.get_layer_offset(layer).x,
                                y: context.size.y * row as f32 + self.get_layer_offset(layer).y,
                            });
                        }
                    }
                }
            }

            tile_positions
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test() {
            const LAYERS: usize = 4;
            const ROWS: usize = 1;
            const COLUMNS: usize = 2;

            let offsets = [
                LayerOffset { x: 0.0, y: 0.0 },
                LayerOffset { x: 1.5, y: 1.5 },
                LayerOffset { x: 2.0, y: 2.0 },
                LayerOffset { x: 2.5, y: 2.5 },
            ];

            let mut grid = Grid::<(), LAYERS, ROWS, COLUMNS>::new(Some([
                offsets[0], offsets[1], offsets[2], offsets[3],
            ]));

            for layer in 0..LAYERS {
                for row in 0..ROWS {
                    for column in 0..COLUMNS {
                        if layer == 2 {
                            // Skip third layer! Let it be None.
                            continue;
                        }
                        assert!(
                            grid.set(layer, row, column, (), UVec2::splat(0))
                                .unwrap()
                                .is_none()
                        );
                    }
                }
            }

            let context = TileContext {
                size: Vec2 { x: 2.0, y: 3.0 },
            };

            let tile_positions: TilePositions<LAYERS, ROWS, COLUMNS> = grid.convert(Some(&context));

            for layer in 0..LAYERS {
                for row in 0..ROWS {
                    for column in 0..COLUMNS {
                        match layer {
                            0 | 1 | 3 => {
                                let Some(position) = tile_positions[layer][row][column] else {
                                    assert!(false, "Position was empty");
                                    continue;
                                };

                                assert_eq!(
                                    position.x,
                                    column as f32 * context.size.x + offsets[layer].x,
                                    "{:?}",
                                    layer
                                );
                                assert_eq!(
                                    position.y,
                                    row as f32 * context.size.y + offsets[layer].y,
                                    "{:?}",
                                    layer
                                );
                            },
                            2 => assert_eq!(tile_positions[layer][row][column], None),
                            4.. => unreachable!(),
                        }
                    }
                }
            }
        }
    }
}

mod logic {
    pub mod grid_factory {
        use super::super::model::grid::Grid;
        use bevy::prelude::*;

        /// Predefined [Grid]s, automatically populated.
        pub trait GridFactoryTrait<
            Occupant,
            const LAYERS: usize,
            const ROWS: usize,
            const COLUMNS: usize,
        >
        {
            type Grid;
            fn new() -> Self;

            /// Consumes [self] to transfer ownership of [Self::Grid] to the caller.
            fn get(self) -> Self::Grid;
        }

        #[derive(Debug, PartialEq)]
        pub enum Occupant {
            Blocked,
            Occupied((usize, Option<Entity>)),
        }

        /// Finds and returns a random valid "reverse free" position in the given layer based on current state of a [Grid].
        ///
        /// "Reverse free" (for given layer) as in any of these being true in order of priority for any given cell:
        /// - If row is occupied already, the returned position must be placed to either free side of it.
        /// - The whole row is empty
        fn reverse_free_position_in_layer<
            const LAYERS: usize,
            const ROWS: usize,
            const COLUMNS: usize,
        >(
            grid: &Grid<Occupant, LAYERS, ROWS, COLUMNS>,
            layer: usize,
        ) -> UVec3 {
            todo!()
        }

        /// Finds and returns a random valid "reverse free" position based on current state of a [Grid].
        ///
        /// "Reverse free" as in any of these being true in order of priority for any given cell:
        /// - Not blocked by other cells in the above layers.
        /// - If row is occupied already, the returned position must be placed to either free side of it.
        /// - The whole row is empty
        fn reverse_free_position<const LAYERS: usize, const ROWS: usize, const COLUMNS: usize>(
            grid: &Grid<Occupant, LAYERS, ROWS, COLUMNS>
        ) -> UVec3 {
            // Pick random row
            // let row =

            todo!()
        }

        pub mod turtle {
            use super::*;
            use rand::{Rng, rngs::ThreadRng};

            pub const LAYERS: usize = 5;
            pub const ROWS: usize = 8 * 2;
            pub const COLUMNS: usize = 15 * 2;
            pub const TILE_VARIANTS: usize = 36;
            // pub const TILE_PAIRS: usize = TILE_VARIANTS * 2; // TODO: Not needed?

            pub struct Turtle {
                grid: Option<Grid<Occupant, LAYERS, ROWS, COLUMNS>>,
                rng: ThreadRng,
                tile_pairs_to_be_placed: Vec<usize>,
            }

            impl Turtle {
                fn populate_grid(&mut self) -> Grid<Occupant, LAYERS, ROWS, COLUMNS> {
                    self.spawn_seed_tiles();
                    self.fill_remaining_cells();
                    self.grid.take().unwrap()
                }

                fn fill_grid_with_n_tile_pairs<
                    G: Fn(&Grid<Occupant, LAYERS, ROWS, COLUMNS>) -> UVec3,
                >(
                    &mut self,
                    tile_pair_count: usize,
                    position_generator: G,
                ) {
                    for _ in 0..tile_pair_count {
                        let tile_pair = self.tile_pairs_to_be_placed.swap_remove(
                            self.rng.random_range(0..self.tile_pairs_to_be_placed.len()),
                        );

                        // Spawn 2 tiles (hence tile PAIR)
                        for _ in 0..2 {
                            let pos = position_generator(self.grid.as_ref().unwrap());
                            assert_eq!(
                                self.grid.as_mut().unwrap().set(
                                    pos.z as usize,
                                    pos.y as usize,
                                    pos.x as usize,
                                    Occupant::Occupied((tile_pair, None)),
                                    UVec2::splat(1),
                                ),
                                Ok(None)
                            );
                        }
                    }
                }

                fn spawn_seed_tiles(&mut self) {
                    let tile_pair_count = self.rng.random_range(3..=3); // Just use 4 for now.
                    self.fill_grid_with_n_tile_pairs(
                        tile_pair_count,
                        |grid: &Grid<Occupant, LAYERS, ROWS, COLUMNS>| -> UVec3 {
                            reverse_free_position_in_layer(grid, 0)
                        },
                    );
                }

                fn fill_remaining_cells(&mut self) {
                    let tile_pair_count = self.tile_pairs_to_be_placed.len();
                    self.fill_grid_with_n_tile_pairs(
                        tile_pair_count,
                        |grid: &Grid<Occupant, LAYERS, ROWS, COLUMNS>| -> UVec3 {
                            reverse_free_position(grid)
                        },
                    );
                }
            }

            impl GridFactoryTrait<Occupant, LAYERS, ROWS, COLUMNS> for Turtle {
                type Grid = Grid<Occupant, LAYERS, ROWS, COLUMNS>;

                fn new() -> Self {
                    Self {
                        grid: Some(Grid::<Occupant, LAYERS, ROWS, COLUMNS>::new(None)),
                        rng: rand::rng(),
                        tile_pairs_to_be_placed: (0..TILE_VARIANTS)
                            .collect::<Vec<usize>>()
                            .repeat(1),
                    }
                }

                fn get(mut self) -> Self::Grid {
                    self.populate_grid()
                }
            }
        }
    }
}
