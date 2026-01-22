use crate::plugin::{default::DefaultWinitSettings, scene::main_menu::MainMenu};
use bevy::{
    input::keyboard::KeyCode,
    prelude::*,
    sprite::{Anchor, Text2dShadow},
    winit::{UpdateMode, WinitSettings},
};
use platform::{Platform, PlatformPlugin, PlatformTrait};
use rand::{
    SeedableRng,
    rngs::StdRng,
    seq::{IteratorRandom, SliceRandom},
};
use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    time::Duration,
};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins(PlatformPlugin)
            .add_sub_state::<InGame>()
            .add_message::<HelpMsg>()
            .add_message::<BoardUpdated>()
            .insert_resource(Timer(bevy::time::Timer::new(
                Duration::from_millis(10),
                TimerMode::Repeating,
            )))
            .insert_resource(Seed::default())
            .insert_resource(TilePositionVariantPairs::default())
            .insert_resource(SelectedTile::default())
            .insert_resource(History::default())
            .insert_resource(HelpEnabled::default())
            .add_systems(OnEnter(InGame::Root), startup)
            .add_systems(
                OnEnter(InGame::Init),
                (
                    update_winit_settings,
                    spawn_background,
                    (bind_tiles_to_positions, spawn_tiles).chain(),
                    spawn_buttons,
                    spawn_info,
                ),
            )
            .add_systems(
                Update,
                resize.run_if(
                    in_state(InGame::Running)
                        .or(in_state(InGame::Init))
                        .or(in_state(InGame::Victory).or(in_state(InGame::Defeat))),
                ),
            )
            .add_systems(
                Update,
                (
                    progressively_show_tiles.run_if(in_state(InGame::Init)),
                    update_move_count.run_if(in_state(InGame::Running)),
                ),
            )
            .add_systems(
                Update,
                (
                    undo_keyboard,
                    redo_keyboard,
                    help_keyboard,
                    help_toggle,
                    help,
                )
                    .run_if(in_state(InGame::Running)),
            )
            .add_systems(Update, poll_new_seed.run_if(in_state(InGame::Running)))
            .add_systems(Update, spawn_finished.run_if(in_state(InGame::Victory)))
            .add_systems(Update, spawn_defeat.run_if(in_state(InGame::Defeat)));
    }
}

mod platform {
    use bevy::prelude::*;
    pub use implementation::Platform;

    pub struct PlatformPlugin;

    impl bevy::prelude::Plugin for PlatformPlugin {
        fn build(
            &self,
            app: &mut App,
        ) {
            app.add_message::<SeedChanged>()
                .add_plugins(implementation::PlatformPlugin);
        }
    }

    pub trait Observer<T> {
        fn observe(
            &mut self,
            object: T,
        );
    }

    #[derive(Message)]
    pub struct SeedChanged {
        pub new: u64,
    }

    pub trait PlatformTrait: Resource + Default {
        const DEFAULT_MSG: &'static str = "Not implemented for this platform!";

        type ObserverItem;

        fn rng_seed_observe(
            &mut self,
            _observer: &mut dyn Observer<Self::ObserverItem>,
        ) {
            debug!("rng_seed_observe {}", Self::DEFAULT_MSG);
        }

        fn rng_seed_get(&self) -> Option<u64> {
            debug!("rng_seed_get {}", Self::DEFAULT_MSG);
            None
        }

        fn rng_seed_set(
            &self,
            seed: u64,
        ) {
            debug!("rng_seed_set {}", Self::DEFAULT_MSG);
        }
    }

    /// NATIVE
    #[cfg(not(target_arch = "wasm32"))]
    mod implementation {
        use super::*;

        pub struct PlatformPlugin;

        impl bevy::prelude::Plugin for PlatformPlugin {
            fn build(
                &self,
                app: &mut App,
            ) {
                let mut platform = Platform::default();
                app.insert_resource(platform);
            }
        }

        #[derive(Resource, Default)]
        pub struct Platform;

        impl PlatformTrait for Platform {
            type ObserverItem = ();
        }

        impl Platform {}
    }

    /// WASM
    #[cfg(target_arch = "wasm32")]
    mod implementation {
        use super::*;
        use std::cell::RefCell;
        use std::rc::Rc;
        use wasm_bindgen::JsCast;
        use wasm_bindgen::prelude::*;
        use web_sys::HashChangeEvent;

        pub struct PlatformPlugin;

        impl bevy::prelude::Plugin for PlatformPlugin {
            fn build(
                &self,
                app: &mut App,
            ) {
                let mut platform = Platform::default();
                let mut observer = HashObserver::default();
                platform.rng_seed_observe(&mut observer);

                app.insert_resource(platform)
                    .insert_non_send_resource(observer)
                    .add_systems(PreUpdate, poll_hash_changes);
            }
        }

        #[derive(Resource, Default)]
        pub struct Platform;

        // Resource: NonSend
        #[derive(Default)]
        pub struct HashObserver {
            pub closure: Option<Closure<dyn FnMut(HashChangeEvent)>>,
            pub pending: Option<Rc<RefCell<Option<(String, String)>>>>,
        }

        impl
            Observer<(
                Closure<dyn FnMut(HashChangeEvent)>,
                Rc<RefCell<Option<(String, String)>>>,
            )> for HashObserver
        {
            fn observe(
                &mut self,
                object: (
                    Closure<dyn FnMut(HashChangeEvent)>,
                    Rc<RefCell<Option<(String, String)>>>,
                ),
            ) {
                self.closure = Some(object.0);
                self.pending = Some(object.1);
            }
        }

        impl Drop for HashObserver {
            fn drop(&mut self) {
                if self.closure.is_some() {
                    let window = web_sys::window().unwrap();
                    window
                        .remove_event_listener_with_callback(
                            "hashchange",
                            self.closure.take().unwrap().as_ref().unchecked_ref(),
                        )
                        .unwrap();
                }
            }
        }

        impl PlatformTrait for Platform {
            type ObserverItem = (
                Closure<dyn FnMut(HashChangeEvent)>,
                Rc<RefCell<Option<(String, String)>>>,
            );

            fn rng_seed_observe(
                &mut self,
                observer: &mut dyn Observer<Self::ObserverItem>,
            ) {
                let window = web_sys::window().expect("no window found");
                let pending = Rc::new(RefCell::new(None));

                let closure = Closure::wrap(Box::new({
                    let pending = Rc::clone(&pending);
                    move |event: HashChangeEvent| {
                        let url_old = event.old_url();
                        let url_new = event.new_url();
                        if let Some(hash_old) = url_old.split_once('#') {
                            let hash_old = hash_old.1;
                            let hash_new = url_new.split_once('#').unwrap().1;
                            *pending.borrow_mut() =
                                Some((hash_old.to_owned(), hash_new.to_owned()));
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                window
                    .add_event_listener_with_callback(
                        "hashchange",
                        closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();

                observer.observe((closure, pending));
            }

            fn rng_seed_get(&self) -> Option<u64> {
                Self::get_fragment()
            }

            fn rng_seed_set(
                &self,
                seed: u64,
            ) {
                Self::set_fragment(&seed.to_string())
            }
        }

        impl Platform {
            fn get_fragment() -> Option<u64> {
                let window = web_sys::window().expect("no global `window` exists");
                let location = window.location();
                location.hash().ok()?.trim_start_matches('#').parse().ok()
            }

            fn set_fragment(fragment_hash: &str) {
                let window = web_sys::window().expect("no global `window` exists");
                let location = window.location();

                location
                    .set_hash(format!("#{fragment_hash}").as_str())
                    .expect("failed to set hash");
            }
        }

        fn poll_hash_changes(
            observer: NonSend<HashObserver>,
            mut writer: MessageWriter<SeedChanged>,
        ) {
            if let Some(pending) = observer.pending.as_ref() {
                let new = pending.borrow_mut().take();
                if let Some((old, new)) = new {
                    if old != new {
                        debug!("New hash! ({new})");
                        let new = new.parse().unwrap();
                        writer.write(SeedChanged { new });
                    }
                }
            }
        }
    }
}

fn spawn<'a>(
    commands: &'a mut Commands,
    bundle: impl Bundle,
) -> EntityCommands<'a> {
    let mut ec = commands.spawn(bundle);
    ec.insert((DespawnOnExit(InGame::Running), Pickable::default()));
    ec
}

#[derive(SubStates, Default, Debug, Hash, Eq, PartialEq, Clone)]
#[source(MainMenu = MainMenu::Play)]
#[states(scoped_entities)]
enum InGame {
    #[default]
    Root,
    Init,
    Running,
    Victory,
    Defeat,
}

#[derive(Resource, Deref, DerefMut, Default)]
struct Seed(Option<u64>);

#[derive(Resource, Deref, DerefMut, Default)]
struct Timer(bevy::time::Timer);

#[derive(Resource, Deref, DerefMut, Default)]
struct TilePositionVariantPairs(Vec<(tile::Position, tile::Variant)>);

#[derive(Resource, Deref, DerefMut, Default)]
struct SelectedTile(Option<Entity>);

#[derive(Resource, Default, Deref, DerefMut, PartialEq, Eq)]
struct HelpEnabled(bool);

#[derive(Message)]
struct HelpMsg;

#[derive(Message)]
struct BoardUpdated;

#[derive(Clone)]
enum HistoryItem {
    ValidPair(Entity, Entity),
    Shuffle(Vec<(Entity, tile::Variant)>),
}

#[derive(Resource, Default)]
struct History {
    undo: VecDeque<HistoryItem>,
    redo: VecDeque<HistoryItem>,
}

impl History {
    const MAX: usize = 32;

    pub fn push_front(
        &mut self,
        item: HistoryItem,
    ) {
        if self.undo.len() >= Self::MAX {
            self.undo.pop_back();
        }
        self.undo.push_front(item);
        self.redo.clear();
    }

    pub fn pop_front(&mut self) -> Option<HistoryItem> {
        let item = self.undo.pop_front();

        if let Some(item) = item.clone() {
            self.redo.push_front(item);
        }

        item
    }

    pub fn push_front_redo(
        &mut self,
        item: HistoryItem,
    ) {
        self.undo.push_front(item);
    }

    pub fn pop_front_redo(&mut self) -> Option<HistoryItem> {
        self.redo.pop_front()
    }
}

mod marker {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Background;

    #[derive(Component)]
    pub struct Hidden;

    #[derive(Component)]
    pub struct Info;
}

mod tile {
    use bevy::prelude::*;
    use std::marker::PhantomData;

    pub mod asset {
        pub mod texture {
            pub const TILE: &'static str = "misc/rev2/lowres/Tile2.png";
            pub const ALLIANCE: &'static str = "misc/rev2/lowres/Alliance.png";
            pub const HORDE: &'static str = "misc/rev2/lowres/Horde.png";
            pub const FROSTMOURNE: &'static str = "misc/rev2/lowres/Frostmourne.png";
            pub const ASHBRINGER: &'static str = "misc/rev2/lowres/Ashbringer.png";

            pub const TILE_WIDTH: u32 = 962;
            pub const TILE_HEIGHT: u32 = 1238;
            pub const TILE_NO_BORDER_WIDTH: u32 = 872;
            pub const TILE_NO_BORDER_HEIGHT: u32 = 1149;
            pub const TILE_BORDER_LENGTH: u32 = 90;
        }
    }

    pub const DEFAULT_COLOR: Color = Color::Hsla(Hsla {
        hue: 60.0,
        saturation: 0.2,
        lightness: 1.25,
        alpha: 1.0,
    });

    #[derive(Bundle)]
    pub struct Tile {
        pub marker: Marker<0>,
        pub position: Position,
        pub variant: Variant,
    }

    /// "DEPTH" implies on which "Child" level the marker is at.
    #[derive(Component)]
    pub struct Marker<const DEPTH: u32>;

    #[derive(Component, Deref, DerefMut, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Position(pub UVec3);

    pub struct Turtle;

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

    impl PositionGenerator<Turtle> {
        pub const TILES: usize = 144;
        pub const TILE_VARIANT_GROUP_SIZE: usize = 4;
        pub const ROWS: usize = 8;
        pub const COLUMNS: usize = 15;
        pub const LAYERS: usize = 5;
        pub const TILE_GRID_SIZE: usize = 2;
    }

    impl Iterator for PositionGenerator<Turtle> {
        type Item = Position;

        fn next(&mut self) -> Option<Self::Item> {
            const TILES_INDEX_MAX: u32 = PositionGenerator::<Turtle>::TILES as u32 - 1;
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
                TILES_INDEX_MAX => {
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

    #[derive(Component, Deref, DerefMut, Clone, Copy, Eq, PartialEq, Debug)]
    pub struct Variant(pub u32);

    impl Variant {
        pub fn insert_sprite_as_child(
            asset_server: &Res<AssetServer>,
            entity_commands: &mut EntityCommands,
            variant: u32,
            max_size: &Vec2,
            offset: &Vec3,
        ) {
            let large = max_size * 0.8;
            let medium = large * 0.5;
            let medium2 = large * 0.7;
            let small = large * 0.40;

            let alliance: Handle<Image> = asset_server.load(asset::texture::ALLIANCE);
            let horde: Handle<Image> = asset_server.load(asset::texture::HORDE);
            let frostmourne: Handle<Image> = asset_server.load(asset::texture::FROSTMOURNE);
            let ashbringer: Handle<Image> = asset_server.load(asset::texture::ASHBRINGER);

            let common = (
                Transform::default().with_translation(Vec3::default().with_z(0.1) + offset),
                Visibility::Inherited,
            );

            fn template(
                x: f32,
                y: f32,
                z: f32,
                size: Vec2,
                image: Handle<Image>,
                shading: Option<Color>,
            ) -> impl Bundle {
                (
                    Transform {
                        translation: Vec3 { x, y, z },
                        ..default()
                    },
                    Sprite {
                        custom_size: Some(size),
                        color: shading.unwrap_or_default(),
                        ..Sprite::from_image(image)
                    },
                )
            }

            match variant {
                0 | 1 | 2 | 3 => {
                    let image = match variant {
                        0 => alliance,
                        1 => horde,
                        2 => frostmourne,
                        3 => ashbringer,
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![template(0.0, 0.0, 0.0, large.clone(), image.clone(), None),],
                    ));
                },
                4 | 5 | 6 | 7 => {
                    let (image, size, inverted) = match variant {
                        4 => (alliance, medium, 1.0),
                        5 => (horde, medium, 1.0),
                        6 => (frostmourne, medium2, 1.0),
                        7 => (ashbringer, medium2, -1.0),
                        _ => unreachable!(),
                    };

                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 8.0 * inverted,
                                max_size.y / 8.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 8.0 * inverted,
                                -max_size.y / 8.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                8 | 9 | 10 | 11 => {
                    let (image, size, inverted) = match variant {
                        8 => (alliance, small, 1.0),
                        9 => (horde, small, 1.0),
                        10 => (frostmourne, medium, 1.0),
                        11 => (ashbringer, medium, -1.0),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0 * inverted,
                                max_size.y / 5.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, 0.0, 0.1, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0 * inverted,
                                -max_size.y / 5.0,
                                0.3,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                12 | 13 | 14 | 15 => {
                    let (image, size) = match variant {
                        12 => (alliance, small),
                        13 => (horde, small),
                        14 => (frostmourne, medium),
                        15 => (ashbringer, medium),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 7.0,
                                -max_size.y / 7.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 7.0,
                                max_size.y / 7.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 7.0,
                                -max_size.y / 7.0,
                                0.2,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 7.0,
                                max_size.y / 7.0,
                                0.3,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                16 | 17 | 18 | 19 => {
                    let (image, color, size) = match variant {
                        16 => (alliance, Color::BLACK, small),
                        17 => (horde, Color::BLACK, small),
                        18 => (frostmourne, Color::BLACK, small),
                        19 => (ashbringer, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                max_size.y / 5.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, 0.0, 0.2, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.3,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 5.0,
                                max_size.y / 5.0,
                                0.4,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                20 | 21 | 22 | 23 => {
                    let (image, color, size) = match variant {
                        20 => (alliance, Color::BLACK, small),
                        21 => (horde, Color::BLACK, small),
                        22 => (frostmourne, Color::BLACK, small),
                        23 => (ashbringer, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 6.0,
                                -max_size.y / 5.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 6.0,
                                0.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 6.0,
                                max_size.y / 5.0,
                                0.2,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 6.0,
                                -max_size.y / 5.0,
                                0.3,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 6.0,
                                0.0,
                                0.4,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 6.0,
                                max_size.y / 5.0,
                                0.5,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                24 | 25 | 26 | 27 => {
                    let (image, color, size) = match variant {
                        24 => (alliance, Color::BLACK, small),
                        25 => (horde, Color::BLACK, small),
                        26 => (frostmourne, Color::BLACK, small),
                        27 => (ashbringer, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                0.0,
                                -max_size.y / 5.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.2,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                0.0,
                                0.3,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, 0.0, 0.4, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0,
                                0.0,
                                0.5,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                0.0,
                                max_size.y / 5.0,
                                0.6,
                                size.clone(),
                                image.clone(),
                                Some(color),
                            ),
                        ],
                    ));
                },
                28 | 29 | 30 | 31 => {
                    let (image, color, size) = match variant {
                        28 => (alliance, Color::BLACK, small),
                        29 => (horde, Color::BLACK, small),
                        30 => (frostmourne, Color::BLACK, small),
                        31 => (ashbringer, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0,
                                max_size.y / 5.0,
                                0.7,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                0.0,
                                max_size.y / 5.0,
                                0.6,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 5.0,
                                max_size.y / 5.0,
                                0.5,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                0.0,
                                0.4,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, 0.0, 0.3, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0,
                                0.0,
                                0.2,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                Some(color),
                            ),
                            template(
                                max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                Some(color),
                            ),
                        ],
                    ));
                },
                32 | 33 | 34 | 35 => {
                    let (image, color, size) = match variant {
                        32 => (alliance, Color::BLACK, small),
                        33 => (horde, Color::BLACK, small),
                        34 => (frostmourne, Color::BLACK, small),
                        35 => (ashbringer, Color::BLACK, small),
                        _ => unreachable!(),
                    };
                    entity_commands.with_child((
                        common,
                        children![
                            template(
                                -max_size.x / 5.0,
                                max_size.y / 5.0,
                                0.6,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                0.0,
                                max_size.y / 5.0,
                                0.7,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 5.0,
                                max_size.y / 5.0,
                                0.8,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                0.0,
                                0.3,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(0.0, 0.0, 0.4, size.clone(), image.clone(), None,),
                            template(
                                max_size.x / 5.0,
                                0.0,
                                0.5,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                -max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.0,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                0.0,
                                -max_size.y / 5.0,
                                0.1,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                            template(
                                max_size.x / 5.0,
                                -max_size.y / 5.0,
                                0.2,
                                size.clone(),
                                image.clone(),
                                None,
                            ),
                        ],
                    ));
                },
                _ => warn!("Unsupported variant!"),
            };
        }
    }
}

mod info {
    use bevy::prelude::*;

    pub mod asset {
        pub const INFO: &'static str = "misc/rev2/lowres/StoneSlab.png";
    }

    #[derive(Component)]
    pub struct ResizeData(pub Vec2, pub bool);

    #[derive(Component, Clone, PartialEq)]
    pub enum Marker {
        Moves,
    }

    impl Marker {
        pub fn as_string(&self) -> &'static str {
            use Marker::*;

            match self {
                Moves => "Moves:\n",
            }
        }
    }
}

mod button {
    use bevy::prelude::*;

    pub mod asset {
        pub const BUTTON: &'static str = "misc/rev2/lowres/button-atlas_1998x429.png";
    }

    #[derive(Component)]
    pub struct ResizeData(pub Vec2, pub bool);

    #[derive(Component, Clone, PartialEq)]
    pub enum Marker {
        Undo,
        Redo,
        Help,
        NewGame,
    }

    impl Marker {
        pub fn as_string(&self) -> &'static str {
            use Marker::*;

            match self {
                Undo => "[U]ndo",
                Redo => "[R]edo",
                Help => "[H]elp",
                NewGame => "NewGame",
            }
        }
    }
}

fn startup(
    mut next_state: ResMut<NextState<InGame>>,
    mut timer: ResMut<Timer>,
    mut seed: ResMut<Seed>,
    mut tile_pos_variant_pairs: ResMut<TilePositionVariantPairs>,
    mut selected_tile: ResMut<SelectedTile>,
    mut history: ResMut<History>,
    mut help_enabled: ResMut<HelpEnabled>,
) {
    *timer = Timer(bevy::time::Timer::new(
        Duration::from_millis(10),
        TimerMode::Repeating,
    ));
    *seed = Seed::default();
    *tile_pos_variant_pairs = TilePositionVariantPairs::default();
    *selected_tile = SelectedTile::default();
    *history = History::default();
    *help_enabled = HelpEnabled::default();
    next_state.set(InGame::Init);
}

fn update_winit_settings(mut winit_settings: ResMut<WinitSettings>) {
    winit_settings.focused_mode = UpdateMode::Continuous;
    winit_settings.unfocused_mode = UpdateMode::Continuous;
}

fn spawn_background(
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

fn bind_tiles_to_positions(
    mut tile_position_variant_pairs: ResMut<TilePositionVariantPairs>,
    platform: ResMut<Platform>,
) {
    let tile_grid_size = tile::PositionGenerator::<tile::Turtle>::TILE_GRID_SIZE as u32;
    let position_generator =
        tile::PositionGenerator::<tile::Turtle>::new(UVec2::splat(tile_grid_size));

    let positions: Vec<tile::Position> = position_generator.collect();
    let seed = platform.rng_seed_get();
    let (mut positions, seed) = generate_solvable_board(positions, seed);
    positions.reverse();
    platform.rng_seed_set(seed);
    tile_position_variant_pairs.0 = positions;
}

/// Returns a [Vec] with (position, variant) tuples along with the rng seed ([u64]) to create them.
fn generate_solvable_board(
    mut available_positions: Vec<tile::Position>,
    seed: Option<u64>,
) -> (Vec<(tile::Position, tile::Variant)>, u64) {
    if available_positions.len() % 2 != 0 {
        panic!();
    }

    let mut result: Vec<(tile::Position, tile::Variant)> = Vec::new();
    let mut occupied_positions: Vec<tile::Position> = Vec::with_capacity(available_positions.len());
    let original_positions = available_positions.clone();

    // Set rng seed
    let seed = seed.unwrap_or(rand::random());
    let mut rng = StdRng::seed_from_u64(seed);

    // Generate [tile::Variant] pairs
    let tile_variants: u32 = available_positions.len() as u32
        / tile::PositionGenerator::<tile::Turtle>::TILE_VARIANT_GROUP_SIZE as u32;
    let mut available_tile_variants: Vec<(tile::Variant, tile::Variant)> = Vec::new();

    for tile_variant in 0..tile_variants {
        available_tile_variants.push((tile::Variant(tile_variant), tile::Variant(tile_variant)));
        available_tile_variants.push((tile::Variant(tile_variant), tile::Variant(tile_variant)));
    }

    available_tile_variants.shuffle(&mut rng);

    /// Returns [Option::Some] if the given index is a candidate to place next iteration.
    fn valid_position_check<'a>(
        index: usize,
        available_positions: &Vec<tile::Position>,
        occupied_positions: &Vec<tile::Position>,
    ) -> Option<usize> {
        let pos = &available_positions[index];

        let mut overlapped_other_available_tile = false;
        for other in available_positions.iter().enumerate() {
            if other.0 == index {
                continue;
            }

            let is_on_same_layer = pos.z == other.1.z;
            let is_above_other_tile = pos.z > other.1.z;
            let is_overlapping_other_tile =
                pos.x.abs_diff(other.1.x) < 2 && pos.y.abs_diff(other.1.y) < 2;

            if is_on_same_layer && is_overlapping_other_tile {
                panic!("Invalid/bad tile positioning!")
            }

            if is_above_other_tile && is_overlapping_other_tile {
                overlapped_other_available_tile = true;
                break;
            }
        }

        if overlapped_other_available_tile {
            debug!("INVALID: Obstructs other available tile position(s).");
            debug!("{pos:?}");
            return None;
        }

        let mut row_already_occupied = false;

        for other in occupied_positions.iter().enumerate() {
            let is_on_same_layer = pos.z == other.1.z;
            let is_on_same_row = pos.y.abs_diff(other.1.y) < 2;

            if is_on_same_layer && is_on_same_row {
                row_already_occupied = true;
                break;
            }
        }

        if !row_already_occupied {
            debug!("VALID: Row is not occupied by any other tile! Any position (column) is valid!");
            debug!("{pos:?}");
            return Some(index);
        }

        let mut is_next_to_occupied_tile = false;

        for other in occupied_positions.iter().enumerate() {
            let is_on_same_layer = pos.z == other.1.z;
            let is_on_same_row = pos.y.abs_diff(other.1.y) < 2;
            let is_next_to_other_tile = pos.x.abs_diff(other.1.x) == 2;

            if is_on_same_layer && is_on_same_row && is_next_to_other_tile {
                is_next_to_occupied_tile = true;
                break;
            }
        }

        if is_next_to_occupied_tile {
            debug!("VALID: Tile (position) is next to an already occupied position");
            debug!("{pos:?}");
            return Some(index);
        }

        debug!("INVALID: Row is occupied, but tile is not next to it.");
        debug!("{pos:?}");
        None
    }

    for (v0, v1) in available_tile_variants {
        debug!("\n\nNew pair placement!");

        let v = [v0, v1];

        // Find valid positions
        let valid_positions = available_positions
            .iter()
            .enumerate()
            .filter_map(|(index, _pos)| {
                valid_position_check(index, &available_positions, &occupied_positions)
            });

        let mut valid: Vec<usize> = valid_positions.collect();
        valid.shuffle(&mut rng);

        let mut chosen_pair = None;

        // Find a pair that remains valid after first placement
        'outer: for &i in &valid {
            for &j in &valid {
                if i == j {
                    continue;
                }

                // Simulate placing i
                let mut available_tmp = available_positions.clone();
                let mut occupied_tmp = occupied_positions.clone();

                let pos_i = available_tmp.swap_remove(i);
                occupied_tmp.push(pos_i);

                // Recompute j index if needed
                let j2 = if j > i { j - 1 } else { j };

                if valid_position_check(j2, &available_tmp, &occupied_tmp).is_some() {
                    chosen_pair = Some((i, j));
                    break 'outer;
                }
            }
        }

        if chosen_pair.is_none() {
            return generate_solvable_board(original_positions, Some(seed + 1));
        }

        let mut chosen_pair = vec![chosen_pair.unwrap().0, chosen_pair.unwrap().1];

        if chosen_pair[1] == available_positions.len() - 1 {
            // Since we are using swap remove, we have to adjust the second of the two indexes in this particular case.
            chosen_pair[1] = chosen_pair[0];
        }

        for i in 0..2 {
            result.push((available_positions[chosen_pair[i]], v[i]));
            occupied_positions.push(available_positions.swap_remove(chosen_pair[i]));
        }
    }

    if available_positions.len() > 0 {
        panic!()
    }

    return (result, seed);
}

fn tile_pressed(
    on_press: On<Pointer<Press>>,
    mut commands: Commands,
    mut tiles: Query<
        (
            Entity,
            &tile::Variant,
            &tile::Position,
            &mut Sprite,
            &mut Visibility,
        ),
        (With<tile::Marker<0>>, Without<marker::Hidden>),
    >,
    mut selected_tile: ResMut<SelectedTile>,
    mut history: ResMut<History>,
    mut board_updated: MessageWriter<BoardUpdated>,
    mut next_state: ResMut<NextState<InGame>>,
) {
    let (pressed_entity, _, _, _, _) = tiles.iter().find(|tile| tile.0 == on_press.entity).unwrap();

    let Some(selected_entity) = selected_tile.0.take() else {
        let (_, _, _, mut pressed_sprite, _) = tiles.get_mut(pressed_entity).unwrap();
        pressed_sprite.color = Color::hsl(0.5, 1.0, 1.5);
        selected_tile.0 = Some(pressed_entity);
        return;
    };

    if selected_entity == pressed_entity {
        let (_, _, _, mut pressed_sprite, _) = tiles.get_mut(pressed_entity).unwrap();
        pressed_sprite.color = tile::DEFAULT_COLOR;
        return;
    }

    let (
        _selected_entity,
        _selected_variant,
        _selected_position,
        mut selected_sprite,
        _selected_visibility,
    ) = tiles.get_mut(selected_entity).unwrap();
    selected_sprite.color = tile::DEFAULT_COLOR;

    let [
        (pressed_entity, pressed_variant, pressed_position, pressed_sprite, pressed_visibility),
        (
            selected_entity,
            selected_variant,
            selected_position,
            _selected_sprite,
            _selected_visibility,
        ),
    ] = tiles.get_many([pressed_entity, selected_entity]).unwrap();

    if valid_removal(
        pressed_entity,
        selected_entity,
        pressed_variant,
        selected_variant,
        pressed_position,
        selected_position,
        &tiles,
    ) == false
    {
        let (_, _, _, mut pressed_sprite, _) = tiles.get_mut(pressed_entity).unwrap();
        pressed_sprite.color = Color::hsl(0.5, 1.0, 1.5);
        selected_tile.0 = Some(pressed_entity);
        return;
    }

    let [
        (
            pressed_entity,
            pressed_variant,
            pressed_position,
            mut pressed_sprite,
            mut pressed_visibility,
        ),
        (
            selected_entity,
            selected_variant,
            selected_position,
            mut selected_sprite,
            mut selected_visibility,
        ),
    ] = tiles
        .get_many_mut([pressed_entity, selected_entity])
        .unwrap();

    history.push_front(HistoryItem::ValidPair(pressed_entity, selected_entity));
    commands.entity(pressed_entity).insert(marker::Hidden);
    commands.entity(selected_entity).insert(marker::Hidden);
    *pressed_visibility = Visibility::Hidden;
    *selected_visibility = Visibility::Hidden;

    board_updated.write(BoardUpdated);

    if tiles.iter().len() == 2 {
        next_state.set(InGame::Victory);
    }
}

fn valid_removal(
    pressed_entity: Entity,
    selected_entity: Entity,
    pressed_variant: &tile::Variant,
    selected_variant: &tile::Variant,
    pressed_position: &tile::Position,
    selected_position: &tile::Position,
    tiles: &Query<
        (
            Entity,
            &tile::Variant,
            &tile::Position,
            &mut Sprite,
            &mut Visibility,
        ),
        (With<tile::Marker<0>>, Without<marker::Hidden>),
    >,
) -> bool {
    fn matching_variants(
        pressed_entity: Entity,
        selected_entity: Entity,
        pressed_variant: &tile::Variant,
        selected_variant: &tile::Variant,
    ) -> bool {
        let m = pressed_entity != selected_entity && pressed_variant == selected_variant;

        if !m {
            info!("Tiles are not matching!");
        }

        m
    }

    fn free_horizontally(
        entity: Entity,
        position: &tile::Position,
        tiles: &Query<
            (
                Entity,
                &tile::Variant,
                &tile::Position,
                &mut Sprite,
                &mut Visibility,
            ),
            (With<tile::Marker<0>>, Without<marker::Hidden>),
        >,
    ) -> bool {
        const TGS: u32 = tile::PositionGenerator::<tile::Turtle>::TILE_GRID_SIZE as u32;

        let mut blocked_left = false;
        let mut blocked_right = false;

        for (other, _, other_pos, _, _) in tiles.iter() {
            if other == entity {
                continue;
            }
            if position.z != other_pos.z {
                continue;
            }

            let overlapping_row = position.y + TGS > other_pos.y && position.y < other_pos.y + TGS;

            if !overlapping_row {
                continue;
            }

            if position.x == other_pos.x + TGS {
                blocked_left = true;
            }

            if position.x + TGS == other_pos.x {
                blocked_right = true;
            }
        }

        !(blocked_left && blocked_right)
    }

    fn free_above(
        entity: Entity,
        position: &tile::Position,
        tiles: &Query<
            (
                Entity,
                &tile::Variant,
                &tile::Position,
                &mut Sprite,
                &mut Visibility,
            ),
            (With<tile::Marker<0>>, Without<marker::Hidden>),
        >,
    ) -> bool {
        const TGS: u32 = tile::PositionGenerator::<tile::Turtle>::TILE_GRID_SIZE as u32;

        !tiles.iter().any(|(other, _, other_pos, _, _)| {
            other != entity
                && other_pos.z > position.z
                && position.y + TGS > other_pos.y
                && position.y < other_pos.y + TGS
                && position.x + TGS > other_pos.x
                && position.x < other_pos.x + TGS
        })
    }

    matching_variants(
        pressed_entity,
        selected_entity,
        pressed_variant,
        selected_variant,
    ) && free_horizontally(pressed_entity, pressed_position, tiles)
        && free_horizontally(selected_entity, selected_position, tiles)
        && free_above(pressed_entity, pressed_position, tiles)
        && free_above(selected_entity, selected_position, tiles)
}

fn spawn_buttons(
    mut commands: Commands,
    projection: Query<&Projection, With<Camera>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let texture_handle: Handle<Image> = asset_server.load(button::asset::BUTTON);
    let texture_atlas =
        TextureAtlasLayout::from_grid(UVec2::new(666 / 3, 429 / 3), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let button_size = Vec2::new(
        (projection.area.height() / tile::PositionGenerator::<tile::Turtle>::ROWS as f32) / 0.7,
        projection.area.height() / tile::PositionGenerator::<tile::Turtle>::ROWS as f32,
    );
    let font = (
        TextFont {
            font_size: button_size.y / 5.0,
            ..default()
        },
        Text2dShadow {
            offset: Vec2 { x: 3.0, y: -3.0 },
            color: Color::srgba(0.0, 0.0, 0.0, 0.95),
        },
        TextColor(Color::srgb_u8(239, 191, 4)),
    );

    struct Button {
        marker: button::Marker,
        flip_x: bool,
        offset: Vec3,
    }

    let buttons = [
        Button {
            marker: button::Marker::Undo,
            flip_x: false,
            offset: Vec3::default(),
        },
        Button {
            marker: button::Marker::Redo,
            flip_x: false,
            offset: Vec3 {
                y: button_size.y,
                ..default()
            },
        },
        Button {
            marker: button::Marker::NewGame,
            flip_x: true,
            offset: Vec3::default(),
        },
        Button {
            marker: button::Marker::Help,
            flip_x: true,
            offset: Vec3 {
                y: button_size.y,
                ..default()
            },
        },
    ];

    for button in buttons {
        let mut ec = spawn(
            &mut commands,
            (
                button.marker.clone(),
                button::ResizeData(button.offset.truncate(), button.flip_x),
                Sprite {
                    custom_size: Some(button_size),
                    ..Sprite::from_atlas_image(
                        texture_handle.clone(),
                        TextureAtlas {
                            layout: texture_atlas_handle.clone(),
                            index: 0,
                        },
                    )
                },
                Transform {
                    translation: Vec3 {
                        x: (-projection.area.width() / 2.0)
                            * if button.flip_x { -1.0 } else { 1.0 },
                        y: -projection.area.height() / 2.0,
                        ..default()
                    } + button.offset,
                    ..default()
                },
                if button.flip_x {
                    Anchor::BOTTOM_RIGHT
                } else {
                    Anchor::BOTTOM_LEFT
                },
                children![(
                    button.marker.clone(),
                    Text2d(button.marker.as_string().to_owned()),
                    font.clone(),
                    Transform {
                        translation: button_size.extend(0.0) / 2.0
                            * if button.flip_x {
                                Vec3 {
                                    x: -1.0,
                                    y: 1.0,
                                    z: 1.0,
                                }
                            } else {
                                Vec3 {
                                    x: 1.0,
                                    y: 1.0,
                                    z: 1.0,
                                }
                            },
                        ..default()
                    },
                )],
            ),
        );

        ec.observe(mouse_over)
            .observe(mouse_out)
            .observe(mouse_press)
            .observe(mouse_release);

        match button.marker {
            button::Marker::Undo => {
                ec.observe(undo_mouse);
            },
            button::Marker::Redo => {
                ec.observe(redo_mouse);
            },
            button::Marker::Help => {
                ec.observe(help_mouse);
            },
            button::Marker::NewGame => {
                ec.observe(new_game_mouse);
            },
        };
    }
}

fn spawn_info(
    mut commands: Commands,
    projection: Query<&Projection, With<Camera>>,
    asset_server: Res<AssetServer>,
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let texture_handle: Handle<Image> = asset_server.load(info::asset::INFO);

    let info_size = Vec2::new(
        (projection.area.height() / tile::PositionGenerator::<tile::Turtle>::ROWS as f32) / 0.7,
        projection.area.height() / tile::PositionGenerator::<tile::Turtle>::ROWS as f32,
    );
    let font = (
        TextFont {
            font_size: info_size.y / 5.0,
            ..default()
        },
        Text2dShadow {
            offset: Vec2 { x: 3.0, y: -3.0 },
            color: Color::srgba(0.0, 0.0, 0.0, 0.95),
        },
        TextColor(Color::srgb_u8(239, 191, 4)),
    );

    struct Info {
        marker: info::Marker,
        flip_x: bool,
        offset: Vec3,
    }

    let infos = [Info {
        marker: info::Marker::Moves,
        flip_x: false,
        offset: Vec3 {
            y: info_size.y * 2.0,
            ..default()
        },
    }];

    for info in infos {
        spawn(
            &mut commands,
            (
                info.marker.clone(),
                info::ResizeData(info.offset.truncate(), info.flip_x),
                Sprite {
                    custom_size: Some(info_size),
                    color: Color::hsl(0.0, 0.0, 0.7),
                    ..Sprite::from_image(texture_handle.clone())
                },
                Transform {
                    translation: Vec3 {
                        x: (-projection.area.width() / 2.0) * if info.flip_x { -1.0 } else { 1.0 },
                        y: -projection.area.height() / 2.0,
                        ..default()
                    } + info.offset,
                    ..default()
                },
                if info.flip_x {
                    Anchor::BOTTOM_RIGHT
                } else {
                    Anchor::BOTTOM_LEFT
                },
                children![(
                    info.marker.clone(),
                    Text2d(info.marker.as_string().to_owned()),
                    font.clone(),
                    Transform {
                        translation: info_size.extend(0.0) / 2.0
                            * if info.flip_x {
                                Vec3 {
                                    x: -1.0,
                                    y: 1.0,
                                    z: 1.0,
                                }
                            } else {
                                Vec3 {
                                    x: 1.0,
                                    y: 1.0,
                                    z: 1.0,
                                }
                            },
                        ..default()
                    },
                )],
            ),
        );
    }
}

fn resize(
    mut transform: Query<
        (&mut Transform, &mut Sprite),
        (
            With<marker::Background>,
            Without<button::Marker>,
            Without<info::Marker>,
        ),
    >,
    buttons: Query<
        (&mut Transform, &button::ResizeData),
        (
            With<button::Marker>,
            Without<marker::Background>,
            Without<info::Marker>,
        ),
    >,
    infos: Query<
        (&mut Transform, &info::ResizeData),
        (
            With<info::Marker>,
            Without<marker::Background>,
            Without<button::Marker>,
        ),
    >,
    projection: Query<&Projection, With<Camera>>,
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let Some((_, mut sprite)) = transform.iter_mut().next() else {
        return;
    };

    if sprite.custom_size.unwrap().x != projection.area.width()
        || sprite.custom_size.unwrap().y != projection.area.height()
    {
        sprite.custom_size = Some(Vec2 {
            x: projection.area.width(),
            y: projection.area.height(),
        });
    }

    for (mut button_transform, button_resize_data) in buttons {
        button_transform.translation = Vec3 {
            x: (-projection.area.width() / 2.0) * if button_resize_data.1 { -1.0 } else { 1.0 },
            y: -projection.area.height() / 2.0,
            ..default()
        } + button_resize_data.0.extend(0.0);
    }

    for (mut info_transform, info_resize_data) in infos {
        info_transform.translation = Vec3 {
            x: (-projection.area.width() / 2.0) * if info_resize_data.1 { -1.0 } else { 1.0 },
            y: -projection.area.height() / 2.0,
            ..default()
        } + info_resize_data.0.extend(0.0);
    }
}

fn spawn_tiles(
    mut commands: Commands,
    projection: Query<&Projection, With<Camera>>,
    asset_server: Res<AssetServer>,
    tile_position_variant_pairs: ResMut<TilePositionVariantPairs>,
    mut board_updated: MessageWriter<BoardUpdated>,
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let tile_texture: Handle<Image> = asset_server.load(tile::asset::texture::TILE);
    let tile_size = Vec2::new(
        (projection.area.height() / tile::PositionGenerator::<tile::Turtle>::ROWS as f32) * 0.85,
        projection.area.height() / tile::PositionGenerator::<tile::Turtle>::ROWS as f32,
    );
    let tile_grid_size = tile::PositionGenerator::<tile::Turtle>::TILE_GRID_SIZE as u32;
    let tile_size_full = Vec2::new(
        (tile_size.x / tile::asset::texture::TILE_NO_BORDER_WIDTH as f32)
            * tile::asset::texture::TILE_WIDTH as f32,
        (tile_size.y / tile::asset::texture::TILE_NO_BORDER_HEIGHT as f32)
            * tile::asset::texture::TILE_HEIGHT as f32,
    );
    let tile_size_ratio = tile_size.y / tile::asset::texture::TILE_NO_BORDER_HEIGHT as f32;
    let tile_border_length_scaled =
        tile::asset::texture::TILE_BORDER_LENGTH as f32 * tile_size_ratio;
    let tile_pos_offset = Vec3::new(
        -(tile_size.x * tile::PositionGenerator::<tile::Turtle>::COLUMNS as f32 / 2.0)
            + tile_size.x * 1.0
            - tile_border_length_scaled / 2.0,
        -projection.area.height() / 2.0 + tile_size_full.y * 0.5 - tile_border_length_scaled,
        0.0,
    );

    let default_depth = Vec3::default().with_z(100.0);
    let column_depth_offset_factor = Vec3::default().with_z(-0.1);
    let row_depth_offset_factor =
        column_depth_offset_factor * tile::PositionGenerator::<tile::Turtle>::COLUMNS as f32;
    let layer_depth_offset_factor = Vec3::default().with_z(10.0);
    let layer_offset_factor = Vec3 {
        x: tile_border_length_scaled,
        y: tile_border_length_scaled,
        ..default()
    };

    for (pos, variant) in tile_position_variant_pairs.iter() {
        let special = match pos.x / tile_grid_size {
            0 => Vec3::default().with_z(
                -column_depth_offset_factor.z
                    * tile::PositionGenerator::<tile::Turtle>::COLUMNS as f32,
            ),
            13 | 14 => Vec3::default().with_z(
                column_depth_offset_factor.z
                    * (tile::PositionGenerator::<tile::Turtle>::COLUMNS as f32),
            ),
            _ => Vec3::default(),
        };

        let mut entity_commands = spawn(
            &mut commands,
            (
                Visibility::Hidden,
                marker::Hidden,
                tile::Tile {
                    marker: tile::Marker::<0>,
                    position: *pos,
                    variant: *variant,
                },
                Sprite {
                    custom_size: Some(tile_size_full),
                    color: tile::DEFAULT_COLOR,
                    ..Sprite::from_image(tile_texture.clone())
                },
                Transform {
                    translation: (((pos.as_vec3() / tile_grid_size as f32)
                        * tile_size.extend(1.0))
                        + tile_pos_offset)
                        + default_depth
                        + (layer_offset_factor * pos.z as f32)
                        + (column_depth_offset_factor * pos.x as f32)
                        + (row_depth_offset_factor * pos.y as f32)
                        + (layer_depth_offset_factor * pos.z as f32)
                        + special,
                    ..default()
                },
            ),
        );

        entity_commands.observe(tile_pressed);

        if pos.z != 0 {
            entity_commands.with_child((
                Sprite {
                    custom_size: Some(tile_size_full),
                    color: Color::hsla(0.0, 0.0, 0.0, 0.75),
                    ..Sprite::from_image(tile_texture.clone())
                },
                Transform {
                    scale: Vec3 {
                        x: 1.2,
                        y: 1.03,
                        ..Vec3::splat(1.0)
                    },
                    translation: Vec3 {
                        x: -tile_size_full.x / 2.0,
                        y: -tile_size_full.y / 2.0,
                        z: column_depth_offset_factor.z * pos.x as f32,
                        ..default()
                    },
                    ..default()
                },
                Anchor::BOTTOM_LEFT,
            ));
        }

        let offset = layer_offset_factor / 2.0;
        tile::Variant::insert_sprite_as_child(
            &asset_server,
            &mut entity_commands,
            variant.0,
            &tile_size,
            &offset,
        );
    }

    board_updated.write(BoardUpdated);
}

fn progressively_show_tiles(
    mut commands: Commands,
    mut tiles: Query<(Entity, &mut Visibility), (With<tile::Marker<0>>, With<marker::Hidden>)>,
    default_winit_settings: ResMut<DefaultWinitSettings>,
    mut winit_settings: ResMut<WinitSettings>,
    mut next_state: ResMut<NextState<InGame>>,
    mut board_updated: MessageWriter<BoardUpdated>,
) {
    if tiles.iter().len() == 0 {
        *winit_settings = default_winit_settings.0.clone();
        board_updated.write(BoardUpdated);
        next_state.set(InGame::Running);
    }

    for (index, (entity, mut visibility)) in tiles.iter_mut().enumerate() {
        commands.entity(entity).remove::<marker::Hidden>();
        *visibility = Visibility::Inherited;

        if index != 0 && index % 8 == 0 {
            break;
        }
    }
}

fn mouse_activity(
    entity: Entity,
    buttons: &mut Query<(Entity, &mut Sprite)>,
    new_index: usize,
) {
    let (_entity, mut sprite) = buttons
        .iter_mut()
        .find(|(entity_, _)| *entity_ == entity)
        .unwrap();
    sprite.texture_atlas.as_mut().unwrap().index = new_index;
}

fn mouse_over(
    on_over: On<Pointer<Over>>,
    mut buttons: Query<(Entity, &mut Sprite)>,
) {
    mouse_activity(on_over.entity, &mut buttons, 1);
}

fn mouse_out(
    on_out: On<Pointer<Out>>,
    mut buttons: Query<(Entity, &mut Sprite)>,
) {
    mouse_activity(on_out.entity, &mut buttons, 0);
}

fn mouse_press(
    on_press: On<Pointer<Press>>,
    mut buttons: Query<(Entity, &mut Sprite)>,
) {
    mouse_activity(on_press.entity, &mut buttons, 2);
}

fn mouse_release(
    on_release: On<Pointer<Release>>,
    mut buttons: Query<(Entity, &mut Sprite)>,
) {
    mouse_activity(on_release.entity, &mut buttons, 1);
}

fn undo_mouse(
    _on_press: On<Pointer<Press>>,
    mut commands: Commands,
    mut history_valid_pair_tiles: Query<
        &mut Visibility,
        (With<tile::Marker<0>>, With<marker::Hidden>),
    >,
    mut history: ResMut<History>,
    mut board_updated: MessageWriter<BoardUpdated>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    undo(
        &mut commands,
        &mut history_valid_pair_tiles,
        &mut history,
        &mut selected_tile,
    );
    board_updated.write(BoardUpdated);
}

fn undo_keyboard(
    key: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut history_valid_pair_tiles: Query<
        &mut Visibility,
        (With<tile::Marker<0>>, With<marker::Hidden>),
    >,
    mut history: ResMut<History>,
    mut board_updated: MessageWriter<BoardUpdated>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    if key.just_pressed(KeyCode::KeyU) {
        undo(
            &mut commands,
            &mut history_valid_pair_tiles,
            &mut history,
            &mut selected_tile,
        );
        board_updated.write(BoardUpdated);
    }
}

fn undo(
    commands: &mut Commands,
    history_valid_pair_tiles: &mut Query<
        &mut Visibility,
        (With<tile::Marker<0>>, With<marker::Hidden>),
    >,
    history: &mut ResMut<History>,
    selected_tile: &mut ResMut<SelectedTile>,
) {
    if let Some(history_item) = history.pop_front() {
        ***selected_tile = None;

        match history_item {
            HistoryItem::ValidPair(entity0, entity1) => {
                let [mut a, mut b] = history_valid_pair_tiles
                    .get_many_mut([entity0, entity1])
                    .unwrap();
                commands.entity(entity0).remove::<marker::Hidden>();
                commands.entity(entity1).remove::<marker::Hidden>();
                *a = Visibility::Inherited;
                *b = Visibility::Inherited;
            },
            HistoryItem::Shuffle(items) => todo!(),
        }
    }
}

fn redo_mouse(
    _on_press: On<Pointer<Press>>,
    mut commands: Commands,
    mut history_valid_pair_tiles: Query<&mut Visibility, With<tile::Marker<0>>>,
    mut history: ResMut<History>,
    mut board_updated: MessageWriter<BoardUpdated>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    redo(
        &mut commands,
        &mut history_valid_pair_tiles,
        &mut history,
        &mut selected_tile,
    );
    board_updated.write(BoardUpdated);
}

fn redo_keyboard(
    key: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut history_valid_pair_tiles: Query<&mut Visibility, With<tile::Marker<0>>>,
    mut history: ResMut<History>,
    mut board_updated: MessageWriter<BoardUpdated>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    if key.just_pressed(KeyCode::KeyR) {
        redo(
            &mut commands,
            &mut history_valid_pair_tiles,
            &mut history,
            &mut selected_tile,
        );
        board_updated.write(BoardUpdated);
    }
}

fn redo(
    commands: &mut Commands,
    history_valid_pair_tiles: &mut Query<&mut Visibility, With<tile::Marker<0>>>,
    history: &mut ResMut<History>,
    selected_tile: &mut ResMut<SelectedTile>,
) {
    if let Some(history_item) = history.pop_front_redo() {
        ***selected_tile = None;

        match history_item {
            HistoryItem::ValidPair(entity0, entity1) => {
                let [mut a, mut b] = history_valid_pair_tiles
                    .get_many_mut([entity0, entity1])
                    .unwrap();
                history.push_front_redo(HistoryItem::ValidPair(entity0, entity1));
                commands.entity(entity0).insert(marker::Hidden);
                commands.entity(entity1).insert(marker::Hidden);
                *a = Visibility::Hidden;
                *b = Visibility::Hidden;
            },
            HistoryItem::Shuffle(items) => todo!(),
        }
    }
}

fn help_mouse(
    _on_press: On<Pointer<Press>>,
    mut help_msg: MessageWriter<HelpMsg>,
) {
    help_msg.write(HelpMsg);
}

fn help_keyboard(
    key: Res<ButtonInput<KeyCode>>,
    mut help_msg: MessageWriter<HelpMsg>,
) {
    if key.just_pressed(KeyCode::KeyH) {
        help_msg.write(HelpMsg);
    }
}

fn help_toggle(
    mut help_msg: MessageReader<HelpMsg>,
    mut buttons: Query<(Entity, &button::Marker, &mut Sprite)>,
    mut help_enabled: ResMut<HelpEnabled>,
) {
    if help_msg.is_empty() {
        return;
    }
    help_msg.clear();

    **help_enabled = !(**help_enabled);

    let (_, _, mut button_sprite) = buttons
        .iter_mut()
        .find(|(_entity, marker, _sprite)| **marker == button::Marker::Help)
        .unwrap();

    match **help_enabled {
        true => button_sprite.color = Color::hsl(120.0, 1.0, 0.5),
        false => button_sprite.color = Color::default(),
    }
}

fn help(
    mut tiles: Query<
        (
            Entity,
            &tile::Variant,
            &tile::Position,
            &mut Sprite,
            &mut Visibility,
        ),
        (With<tile::Marker<0>>, Without<marker::Hidden>),
    >,
    selected_tile: Res<SelectedTile>,
    help_enabled: Res<HelpEnabled>,
    mut prev_selection: Local<Option<Entity>>,
) {
    match selected_tile.0 {
        Some(selected_tile) => {
            if !**help_enabled {
                *prev_selection = None;

                let (
                    selected_entity,
                    selected_variant,
                    _selected_position,
                    _selected_sprite,
                    _selected_visiblity,
                ) = tiles.get(selected_tile).unwrap();

                let mut entities_to_reset = vec![];

                for (entity, variant, _position, _sprite, _visibility) in &tiles {
                    if selected_entity != entity && *selected_variant == *variant {
                        entities_to_reset.push(entity);
                    }
                }

                for entity in entities_to_reset {
                    let (_entity, _variant, _position, mut sprite, _visiblity) =
                        tiles.get_mut(entity).unwrap();
                    sprite.color = tile::DEFAULT_COLOR;
                }
                return;
            }

            if let Some(prev_selection) = *prev_selection {
                if prev_selection == selected_tile {
                    return;
                }
            }

            *prev_selection = Some(selected_tile);

            let (
                selected_entity,
                selected_variant,
                _selected_position,
                _selected_sprite,
                _selected_visiblity,
            ) = tiles.get(selected_tile).unwrap();

            let mut entities_to_update = vec![];
            let mut entities_to_reset = vec![];

            for (entity, variant, _position, _sprite, _visibility) in &tiles {
                if selected_entity != entity && *selected_variant == *variant {
                    entities_to_update.push(entity);
                } else if selected_entity != entity {
                    entities_to_reset.push(entity);
                }
            }

            for entity in entities_to_update {
                let (_entity, _variant, _position, mut sprite, _visiblity) =
                    tiles.get_mut(entity).unwrap();
                sprite.color = Color::hsl(120.0, 1.0, 0.5);
            }

            for entity in entities_to_reset {
                let (_entity, _variant, _position, mut sprite, _visiblity) =
                    tiles.get_mut(entity).unwrap();
                sprite.color = tile::DEFAULT_COLOR;
            }
        },
        None => {
            *prev_selection = None;
            for (_entity, _variant, _position, mut sprite, _visibility) in &mut tiles {
                sprite.color = tile::DEFAULT_COLOR;
            }
        },
    }
}

fn update_move_count(
    info_texts: Query<(&mut Text2d, &info::Marker)>,
    positions: Query<
        (&tile::Position, &tile::Variant),
        (With<tile::Marker<0>>, Without<marker::Hidden>),
    >,
    mut board_updated: MessageReader<BoardUpdated>,
    mut next_state: ResMut<NextState<InGame>>,
) {
    if board_updated.read().count() == 0 {
        return;
    }

    let len = positions.iter().len();

    if len == 0 || len % 2 != 0 {
        return;
    }

    let mut free_variants: HashMap<u32, u32> = HashMap::new();

    'outer: for (index0, (p0, v0)) in positions.iter().enumerate() {
        let mut is_free_horizontally_counter: u32 = 0;

        for (index1, (p1, v1)) in positions.iter().enumerate() {
            if index0 == index1 {
                continue;
            }

            let overlap = p0.x.abs_diff(p1.x) < 2 && p0.y.abs_diff(p1.y) < 2;

            if overlap {
                let is_free_vertically = p0.z > p1.z;
                if !is_free_vertically {
                    continue 'outer;
                }
            }

            let same_layer = p0.z == p1.z;
            if !same_layer {
                continue;
            }

            let is_same_row = p0.y.abs_diff(p1.y) < 2;
            let is_next_to = is_same_row && p0.x.abs_diff(p1.x) == 2;

            if is_next_to {
                is_free_horizontally_counter += 1;

                if is_free_horizontally_counter >= 2 {
                    continue 'outer;
                }
            }
        }

        if let Some(count) = free_variants.get_mut(&(v0.0)) {
            *count += 1;
        } else {
            free_variants.insert(v0.0, 1);
        }
    }

    let moves = free_variants
        .iter()
        .fold(0, |acc, (_variant, count)| acc + (*count / 2));

    for (mut info_text, info_marker) in info_texts {
        if matches!(info_marker, info::Marker::Moves) {
            info_text.0 = format!("Moves:\n{moves}").to_string();
        }
    }

    if moves < 1 {
        next_state.set(InGame::Defeat);
    }
}

fn new_game_mouse(
    _on_press: On<Pointer<Press>>,
    state: Res<State<InGame>>,
    mut next_state: ResMut<NextState<InGame>>,
    platform: ResMut<Platform>,
) {
    if matches!(state.get(), InGame::Running) {
        info!("New Game!");
        platform.rng_seed_set(rand::random::<u64>());
        next_state.set(InGame::Root);
    }
}

fn poll_new_seed(
    mut msg: MessageReader<platform::SeedChanged>,
    mut next_state: ResMut<NextState<InGame>>,
) {
    if let Some(_msg) = msg.read().last() {
        info!("New Game!");
        next_state.set(InGame::Root);
    }
}

fn spawn_finished(
    mut commands: Commands,
    projection: Query<&Projection, With<Camera>>,
    asset_server: Res<AssetServer>,
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let handle: Handle<Image> = asset_server.load("misc/rev2/original/Victory.png");

    spawn(
        &mut commands,
        (
            marker::Background,
            Sprite {
                custom_size: Some(Vec2::new(projection.area.width(), projection.area.height())),
                ..Sprite::from_image(handle)
            },
            Transform { ..default() },
        ),
    );
}

fn spawn_defeat(
    mut commands: Commands,
    projection: Query<&Projection, With<Camera>>,
    asset_server: Res<AssetServer>,
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let handle: Handle<Image> = asset_server.load("misc/rev2/original/Defeat.png");

    spawn(
        &mut commands,
        (
            marker::Background,
            Sprite {
                custom_size: Some(Vec2::new(projection.area.width(), projection.area.height())),
                ..Sprite::from_image(handle)
            },
            Transform { ..default() },
        ),
    );
}
