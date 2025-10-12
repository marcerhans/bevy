use crate::plugin::scene::main_menu::MainMenu;
use bevy::prelude::*;
use generator::*;
use helpers::*;
use rand::seq::SliceRandom;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<InGame>()
            .insert_resource(PreviouslySelectedTile::default())
            .add_systems(OnEnter(InGame::Root), on_enter)
            .add_systems(Update, next_move.run_if(in_state(InGame::Root)));
    }
}

#[derive(Component, SubStates, Hash, Eq, PartialEq, Clone, Debug, Default)]
#[source(MainMenu = MainMenu::Play)]
#[states(scoped_entities)]
pub enum InGame {
    #[default]
    Root,
}

#[derive(Resource, Default)]
struct PreviouslySelectedTile(Option<Entity>);

#[derive(Component, Clone)]
struct Tile;

#[derive(Component)]
#[relationship_target(relationship = PairWithTile)]
struct TilePairs(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = TilePairs)]
struct PairWithTile(pub Entity);

#[derive(Component, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ID(usize);

fn on_enter(
    mut commands: Commands,
    window: Single<&Window>,
) {
    let height = window.height() / 8.0;
    let width = height * 0.7;

    let mut rng = rand::rng();
    let mut tile_pairs: Vec<usize> = (0..Generator::<Turtle>::TILES / 2).collect();
    tile_pairs.shuffle(&mut rng);

    let placer = Placer::new(Vec2::new(width, height), Generator::<Turtle>::new());
    let tile_positions: Vec<(usize, Vec3)> = placer.into_iter().enumerate().collect();

    let columns = 11.0;
    let rows = 8.0;
    let start_x = -width * columns / 2.0;
    let start_y = height * rows / 2.0;

    let tile_components = (
        Tile,
        DespawnOnExit(InGame::Root),
        // Sprite::from_color(Color::srgb_u8(255, 0, 0), Vec2::new(width, height)),
        Pickable::default(),
        TextFont::from_font_size(height / 5.0),
        TextColor::BLACK,
    );

    for (tile_pair, position_pair) in tile_pairs.iter().zip(tile_positions.windows(2).step_by(2)) {
        let tile_a = commands
            .spawn((
                tile_components.clone(),
                ID(*tile_pair),
                Text2d::new(tile_pair.to_string()),
                Transform {
                    translation: Vec3 {
                        x: start_x + position_pair[0].1.x,
                        y: start_y - position_pair[0].1.y,
                        z: position_pair[0].1.z,
                    },
                    ..default()
                },
                Sprite::from_color(
                    match position_pair[0].0 {
                        ..87 => Color::srgb_u8(255, 0, 0),
                        87..123 => Color::srgb_u8(0, 255, 0),
                        123..139 => Color::srgb_u8(0, 0, 255),
                        139..143 => Color::srgb_u8(255, 255, 0),
                        143.. => Color::srgb_u8(255, 0, 255),
                    },
                    Vec2::new(width, height),
                ),
            ))
            .observe(on_click)
            .id();

        commands
            .spawn((
                tile_components.clone(),
                ID(*tile_pair),
                Text2d::new(tile_pair.to_string()),
                Transform {
                    translation: Vec3 {
                        x: start_x + position_pair[1].1.x,
                        y: start_y - position_pair[1].1.y,
                        z: position_pair[1].1.z,
                    },
                    ..default()
                },
                Sprite::from_color(
                    match position_pair[1].0 {
                        ..87 => Color::srgb_u8(255, 0, 0),
                        87..123 => Color::srgb_u8(0, 255, 0),
                        123..139 => Color::srgb_u8(0, 0, 255),
                        139..143 => Color::srgb_u8(255, 255, 0),
                        143.. => Color::srgb_u8(255, 0, 255),
                    },
                    Vec2::new(width, height),
                ),
                PairWithTile(tile_a),
            ))
            .observe(on_click);

        // .observe(
        //     |drag: On<Pointer<Drag>>,
        //      window: Single<&Window>,
        //      mut transform: Query<&mut Transform>,
        //      camera: Single<(&Camera, &GlobalTransform, &Projection)>| {
        //         let window = *window;
        //         let (_camera, camera_transform, projection) = *camera;

        //         let Projection::Orthographic(ortho) = projection else {
        //             panic!();
        //         };

        //         let window_size = Vec2::new(window.width(), window.height());
        //         let ortho_size = ortho.area.size() * ortho.scale;
        //         let world_per_pixel;

        //         match ortho.scaling_mode {
        //             ScalingMode::WindowSize => {
        //                 // uniform scaling: world units per pixel is linear
        //                 world_per_pixel = ortho_size / window_size;
        //             },
        //             ScalingMode::FixedVertical { viewport_height: _ } => {
        //                 let scale = ortho_size.y / window_size.y;
        //                 world_per_pixel = Vec2::new(scale, scale); // horizontal scales proportionally
        //             },
        //             ScalingMode::FixedHorizontal { viewport_width: _ } => {
        //                 let scale = ortho_size.x / window_size.x;
        //                 world_per_pixel = Vec2::new(scale, scale); // vertical scales proportionally
        //             },
        //             _ => panic!(),
        //         }

        //         let mut world_delta = Vec2::new(drag.delta.x, -drag.delta.y) * world_per_pixel;

        //         // Apply camera rotation if needed
        //         world_delta =
        //             (camera_transform.rotation() * world_delta.extend(0.0)).truncate();

        //         // Move the entity
        //         let mut transform = transform.get_mut(drag.original_event_target()).unwrap();
        //         transform.translation.x += world_delta.x;
        //         transform.translation.y += world_delta.y;
        //     },
        // )
    }
}

fn next_move(
    mut removed: RemovedComponents<Tile>,
    query: Query<(Entity, &ID, &Transform, &Sprite), With<Tile>>,
) {
    if removed.is_empty() {
        return;
    }

    removed.clear();

    // for (entity, id, transform, sprite) in query {
    //     for (entity, id, transform, sprite) in query {
    //         todo!("Loop through like this or just pair them when spawning them...");
    //     }
    // }
}

fn on_click(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    mut prev_res: ResMut<PreviouslySelectedTile>,
    query: Query<(Entity, &ID, &Transform, &Sprite), With<Tile>>,
) {
    let curr_entity = query.get(click.original_event_target()).unwrap();

    let Some(prev_entity) = prev_res.0 else {
        prev_res.0 = Some(curr_entity.0);
        return;
    };

    let Ok(prev_entity) = query.get(prev_entity) else {
        return;
    };

    info!("Clicked:\n{:?}", curr_entity);
    info!("Prevously clicked:\n{:?}", prev_entity);

    // Check rules:
    // 1. NOT Same id?
    // 2. Same entity?
    if *prev_entity.1 != *curr_entity.1 || prev_entity.0 == curr_entity.0 {
        info!("Same entity or non-matching ids");
        prev_res.0 = Some(curr_entity.0);
        return;
    }

    // 3. BOTH entities have free space to either left or right.
    // 4. BOTH entities are not blocked by any above
    let mut prev_left = false;
    let mut prev_right = false;
    let mut prev_obscured = false;
    let mut curr_left = false;
    let mut curr_right = false;
    let mut curr_obscured = false;
    let prev_size = prev_entity.3.custom_size.unwrap();
    let curr_size = curr_entity.3.custom_size.unwrap();
    let prev_pos = prev_entity.2.translation;
    let curr_pos = curr_entity.2.translation;

    for entity in query {
        let size = entity.3.custom_size.unwrap();
        let pos = entity.2.translation;

        if let Some(side) = which_side((prev_size, prev_pos), (size, pos)) {
            match side {
                LR::Left => prev_left = true,
                LR::Right => prev_right = true,
            }
        }

        if let Some(side) = which_side((curr_size, curr_pos), (size, pos)) {
            match side {
                LR::Left => curr_left = true,
                LR::Right => curr_right = true,
            }
        }

        prev_obscured |= overlapping((prev_size, prev_pos), (size, pos)) && prev_pos.z < pos.z;
        curr_obscured |= overlapping((curr_size, curr_pos), (size, pos)) && curr_pos.z < pos.z;
    }

    info!(
        "Prev left|right|obscured: {:?}|{:?}|{:?}",
        prev_left, prev_right, prev_obscured
    );
    info!(
        "Curr left|right|obscured: {:?}|{:?}|{:?}",
        curr_left, curr_right, curr_obscured
    );

    if ((prev_left && prev_right) || prev_obscured) || ((curr_left && curr_right) || curr_obscured)
    {
        info!("Failed pairing due to neighbouring rules.");
        prev_res.0 = Some(curr_entity.0);
        return;
    }

    commands.entity(prev_entity.0).despawn();
    commands.entity(curr_entity.0).despawn();
    prev_res.0 = None;
    return;
}

mod helpers {
    use bevy::prelude::{Vec2, Vec3};

    pub fn overlapping(
        (size_a, pos_a): (Vec2, Vec3),
        (size_b, pos_b): (Vec2, Vec3),
    ) -> bool {
        let x_overlap = (pos_a.x - pos_b.x).abs() < ((size_a.x + size_b.x) / 2.0);
        let y_overlap = (pos_a.y - pos_b.y).abs() < ((size_a.y + size_b.y) / 2.0);
        x_overlap && y_overlap
    }

    pub enum LR {
        Left,
        Right,
    }

    pub fn which_side(
        (size_a, pos_a): (Vec2, Vec3),
        (size_b, pos_b): (Vec2, Vec3),
    ) -> Option<LR> {
        if pos_a.z != pos_b.z {
            return None;
        }

        let y_overlap = (pos_a.y - pos_b.y).abs() < ((size_a.y + size_b.y) / 2.0);

        if !y_overlap {
            return None;
        }

        match pos_a.x > pos_b.x {
            true => Some(LR::Left),
            false => Some(LR::Right),
        }
    }
}

mod generator {
    use bevy::prelude::{Vec2, Vec3};
    use std::marker::PhantomData;

    pub struct Turtle;

    pub struct Generator<T>(PhantomData<T>);

    impl<T> Generator<T> {
        pub fn new() -> Self {
            Self(PhantomData::<T>)
        }
    }

    impl Generator<Turtle> {
        pub const TILES: usize = 144;
    }

    impl PositionGenerator for Generator<Turtle> {
        fn generate(
            &self,
            tile_size: Vec2,
            current: usize,
        ) -> Option<Vec3> {
            if current >= Self::TILES {
                return None;
            }

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
                    layer = 4;
                    return Some(Vec3::new(5.5, 3.5, layer as f32) * tile_size.extend(1.0));
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
                                return Some(
                                    Vec3::new(-1.0, 3.5, layer as f32) * tile_size.extend(1.0),
                                );
                            },
                            1 => {
                                return Some(
                                    Vec3::new(12.0, 3.5, layer as f32) * tile_size.extend(1.0),
                                );
                            },
                            2 => {
                                return Some(
                                    Vec3::new(13.0, 3.5, layer as f32) * tile_size.extend(1.0),
                                );
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

            Some(Vec3::new(column as f32, row as f32, layer as f32) * tile_size.extend(1.0))
        }
    }

    pub trait PositionGenerator {
        fn generate(
            &self,
            tile_size: Vec2,
            current: usize,
        ) -> Option<Vec3>;
    }

    pub struct Placer<G: PositionGenerator> {
        tile_size: Vec2,
        generator: G,
    }

    impl<G: PositionGenerator> Placer<G> {
        pub fn new(
            tile_size: Vec2,
            generator: G,
        ) -> Self {
            Self {
                tile_size,
                generator,
            }
        }
    }

    pub struct PlacerIterator<'a, G: PositionGenerator> {
        placer: &'a Placer<G>,
        counter: usize,
    }

    type PlaceIteratorItem = Vec3;

    impl<'a, G: PositionGenerator> Iterator for PlacerIterator<'a, G> {
        type Item = PlaceIteratorItem;

        fn next(&mut self) -> Option<Self::Item> {
            self.counter += 1;
            self.placer
                .generator
                .generate(self.placer.tile_size, self.counter - 1)
        }
    }

    impl<'a, G: PositionGenerator> IntoIterator for &'a Placer<G> {
        type Item = PlaceIteratorItem;
        type IntoIter = PlacerIterator<'a, G>;

        fn into_iter(self) -> Self::IntoIter {
            PlacerIterator {
                placer: self,
                counter: 0,
            }
        }
    }
}
