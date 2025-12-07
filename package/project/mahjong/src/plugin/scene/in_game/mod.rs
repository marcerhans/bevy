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
                text: "Help (h)",
            },
            Button {
                translation: Vec3 {
                    x: button_pos_start.x + (button_size.x + button_margin.x) * 0.0,
                    y: button_pos_start.y + (button_size.y + button_margin.y) * 1.0,
                    ..button_pos_start
                },
                text: "Shuffle (s)",
            },
            Button {
                translation: button_pos_start_right,
                text: "  Undo (u)\n(limit: 32)",
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
        "Shuffle (s)" => {
            msg_shuffle.write(msg::Shuffle);
        },
        "Help (h)" => {
            msg_help.write(msg::Help);
        },
        "  Undo (u)\n(limit: 32)" => {
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
    pub mod grid {
        use std::ops::{Index, IndexMut};

        #[derive(Copy, Clone, Default)]
        pub struct LayerOffset {
            x: f32,
            y: f32,
        }

        pub trait OccupantTrait: Eq {}
        impl<T: Eq> OccupantTrait for T {}

        pub type Layer<Occupant, const ROWS: usize, const COLUMNS: usize> =
            [[Option<Occupant>; COLUMNS]; ROWS];

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
            Grid<Occupant, LAYERS, ROWS, COLUMNS>
        {
            pub fn new(layer_offsets: Option<[LayerOffset; LAYERS]>) -> Self {
                let layer_offsets = match layer_offsets {
                    Some(layer_offsets) => layer_offsets,
                    None => [LayerOffset::default(); LAYERS],
                };

                let occupied: [(LayerOffset, Layer<Occupant, ROWS, COLUMNS>); LAYERS] =
                    std::array::from_fn(|index| {
                        let layer =
                            std::array::from_fn(|_| std::array::from_fn(|_| None::<Occupant>));
                        (layer_offsets[index], layer)
                    });

                Self { occupied }
            }

            pub fn set(
                &mut self,
                layer: usize,
                row: usize,
                column: usize,
                occupant: Occupant,
            ) {
                let layer = &mut self[layer];
                let row = &mut (*layer).1[row];
                let old_occupant = &mut (*row)[column];
                *old_occupant = Some(occupant);
            }

            pub fn try_set(
                &mut self,
                layer: usize,
                row: usize,
                column: usize,
                occupant: Occupant,
            ) -> Option<()> {
                if !Self::is_within_bounds(layer, row, column) {
                    return None;
                }
                self.set(layer, row, column, occupant);
                Some(())
            }

            pub fn set_const<const LAYER: usize, const ROW: usize, const COLUMN: usize>(
                &mut self,
                occupant: Occupant,
            ) {
                self.set(LAYER, ROW, COLUMN, occupant);
            }

            pub fn try_set_const<const LAYER: usize, const ROW: usize, const COLUMN: usize>(
                &mut self,
                occupant: Occupant,
            ) -> Option<()> {
                if !Self::is_within_bounds_const::<LAYER, ROW, COLUMN>() {
                    return None;
                }
                self.set(LAYER, ROW, COLUMN, occupant);
                Some(())
            }

            pub fn get(
                &self,
                layer: usize,
                row: usize,
                column: usize,
            ) -> Option<&Occupant> {
                self[layer].1[row][column].as_ref()
            }

            pub fn try_get(
                &self,
                layer: usize,
                row: usize,
                column: usize,
            ) -> Option<&Occupant> {
                if !Self::is_within_bounds(layer, row, column) {
                    return None;
                }
                self[layer].1[row][column].as_ref()
            }

            pub fn get_mut(
                &mut self,
                layer: usize,
                row: usize,
                column: usize,
            ) -> Option<&mut Occupant> {
                self[layer].1[row][column].as_mut()
            }

            pub fn try_get_mut(
                &mut self,
                layer: usize,
                row: usize,
                column: usize,
            ) -> Option<&mut Occupant> {
                if !Self::is_within_bounds(layer, row, column) {
                    return None;
                }
                self[layer].1[row][column].as_mut()
            }

            pub fn get_const<const LAYER: usize, const ROW: usize, const COLUMN: usize>(
                &self
            ) -> Option<&Occupant> {
                self[LAYER].1[ROW][COLUMN].as_ref()
            }

            pub fn try_get_const<const LAYER: usize, const ROW: usize, const COLUMN: usize>(
                &self
            ) -> Option<&Occupant> {
                if !Self::is_within_bounds_const::<LAYER, ROW, COLUMN>() {
                    return None;
                }
                self[LAYER].1[ROW][COLUMN].as_ref()
            }

            pub fn get_mut_const<const LAYER: usize, const ROW: usize, const COLUMN: usize>(
                &mut self
            ) -> Option<&mut Occupant> {
                self[LAYER].1[ROW][COLUMN].as_mut()
            }

            pub fn try_get_mut_const<const LAYER: usize, const ROW: usize, const COLUMN: usize>(
                &mut self
            ) -> Option<&mut Occupant> {
                if !Self::is_within_bounds_const::<LAYER, ROW, COLUMN>() {
                    return None;
                }
                self[LAYER].1[ROW][COLUMN].as_mut()
            }

            fn is_within_bounds(
                layer: usize,
                row: usize,
                column: usize,
            ) -> bool {
                layer < LAYERS && row < ROWS && column < COLUMNS
            }

            const fn is_within_bounds_const<
                const LAYER: usize,
                const ROW: usize,
                const COLUMN: usize,
            >() -> bool {
                LAYER < LAYERS && ROW < ROWS && COLUMN < COLUMNS
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

        #[cfg(test)]
        mod tests {
            use super::*;
            use rand::Rng;

            mod set {
                use super::*;

                #[test]
                fn test() {
                    let mut grid = Grid::<bool, 1, 2, 2>::new(None);
                    let mut rng = rand::rng();

                    // Set
                    let row: u32 = rng.random_range(0..2);
                    let col: u32 = rng.random_range(0..2);
                    let row = row as usize;
                    let col = col as usize;
                    grid.set(0, row, col, true);

                    // Set const
                    grid.set_const::<0, 0, 0>(true);

                    // Try set
                    assert_eq!(grid.try_set(0, 0, 0, true), Some(()));
                    assert_eq!(grid.try_set(0, 1, 0, true), Some(()));
                    assert_eq!(grid.try_set(0, 0, 1, true), Some(()));
                    assert_eq!(grid.try_set(0, 1, 1, true), Some(()));
                    assert_eq!(grid.try_set(1, 0, 0, true), None);
                    assert_eq!(grid.try_set(0, 2, 0, true), None);
                    assert_eq!(grid.try_set(0, 0, 2, true), None);

                    // Try set const
                    assert_eq!(grid.try_set_const::<0, 0, 0>(true), Some(()));
                    assert_eq!(grid.try_set_const::<0, 1, 0>(true), Some(()));
                    assert_eq!(grid.try_set_const::<0, 0, 1>(true), Some(()));
                    assert_eq!(grid.try_set_const::<0, 1, 1>(true), Some(()));
                    assert_eq!(grid.try_set_const::<1, 0, 0>(true), None);
                    assert_eq!(grid.try_set_const::<0, 2, 0>(true), None);
                    assert_eq!(grid.try_set_const::<0, 0, 2>(true), None);
                }

                #[test]
                #[should_panic]
                fn should_panic0() {
                    let mut grid = Grid::<bool, 1, 2, 2>::new(None);
                    grid.set(1, 0, 0, true);
                }

                #[test]
                #[should_panic]
                fn should_panic1() {
                    let mut grid = Grid::<bool, 1, 2, 2>::new(None);
                    grid.set_const::<1, 0, 0>(true);
                }
            }

            mod get {
                use super::*;

                #[test]
                fn test() {
                    let mut grid = Grid::<bool, 1, 2, 2>::new(None);
                    grid.set_const::<0, 0, 0>(true);

                    // Get
                    assert_eq!(*grid.get(0, 0, 0).unwrap(), true);
                    assert_eq!(grid.get(0, 0, 1), None);

                    // Get const
                    assert_eq!(*grid.get_const::<0, 0, 0>().unwrap(), true);
                    assert_eq!(grid.get_const::<0, 0, 1>(), None);

                    // Try get
                    assert_eq!(*grid.try_get(0, 0, 0).unwrap(), true);
                    assert_eq!(grid.try_get(0, 1, 0), None);
                    assert_eq!(grid.try_get(0, 0, 1), None);
                    assert_eq!(grid.try_get(0, 1, 1), None);
                    assert_eq!(grid.try_get(1, 0, 0), None);
                    assert_eq!(grid.try_get(0, 2, 0), None);
                    assert_eq!(grid.try_get(0, 0, 2), None);

                    // Try get const
                    assert_eq!(*grid.try_get_const::<0, 0, 0>().unwrap(), true);
                    assert_eq!(grid.try_get_const::<0, 1, 0>(), None);
                    assert_eq!(grid.try_get_const::<0, 0, 1>(), None);
                    assert_eq!(grid.try_get_const::<0, 1, 1>(), None);
                    assert_eq!(grid.try_get_const::<1, 0, 0>(), None);
                    assert_eq!(grid.try_get_const::<0, 2, 0>(), None);
                    assert_eq!(grid.try_get_const::<0, 0, 2>(), None);
                }

                #[test]
                #[should_panic]
                fn should_panic0() {
                    let grid = Grid::<bool, 1, 2, 2>::new(None);
                    grid.get(1, 0, 0);
                }

                #[test]
                #[should_panic]
                fn should_panic1() {
                    let mut grid = Grid::<bool, 1, 2, 2>::new(None);
                    grid.get_mut(1, 0, 0);
                }

                #[test]
                #[should_panic]
                fn should_panic2() {
                    let grid = Grid::<bool, 1, 2, 2>::new(None);
                    grid.get_const::<1, 0, 0>();
                }

                #[test]
                #[should_panic]
                fn should_panic3() {
                    let mut grid = Grid::<bool, 1, 2, 2>::new(None);
                    grid.get_mut_const::<1, 0, 0>();
                }
            }
        }
    }
}

mod view {
    use super::model::grid::*;
    use bevy::prelude::*;

    pub trait ModelToViewData<View> {
        type Context;

        fn convert(
            &self,
            context: Option<Self::Context>,
        ) -> View;
    }

    pub struct TileContext {
        size: Vec2,
    }

    type TilePositions<const LAYERS: usize> = Vec<[Vec2; LAYERS]>;
    impl<const LAYERS: usize, const ROWS: usize, const COLUMNS: usize> ModelToViewData<TilePositions<LAYERS>>
        for Grid<Entity, LAYERS, ROWS, COLUMNS>
    {
        type Context = TileContext;

        fn convert(
            &self,
            context: Option<Self::Context>,
        ) -> TilePositions<LAYERS> {
            todo!()
        }
    }
}

mod controller {}
