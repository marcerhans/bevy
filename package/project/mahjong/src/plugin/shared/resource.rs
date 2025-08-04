use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(asset::atlas::X384::default());
    }
}

pub mod asset {
    use bevy::prelude::*;

    pub trait Load {
        fn load<T: Asset>(asset_server: &Res<AssetServer>) -> Handle<T>;
    }

    pub mod atlas {
        use bevy::prelude::*;

        #[derive(Resource, Default)]
        pub struct X384(pub Handle<Image>);

        impl super::Load for X384 {
            fn load<T: Asset>(asset_server: &Res<AssetServer>) -> Handle<T> {
                asset_server.load::<T>("atlas/x384.png")
            }
        }
    }

    #[deprecated]
    mod deprecated {
        use bevy::prelude::*;
        use std::{
            any::{Any, TypeId},
            collections::HashMap,
        };

        pub struct Plugin;

        impl bevy::prelude::Plugin for Plugin {
            fn build(
                &self,
                app: &mut App,
            ) {
                let world = app.world();
                let asset_server_clone = world
                    .get_resource::<AssetServer>()
                    .expect("Asset plugin has not been run yet.")
                    .clone();

                app.insert_resource(Assets::new(asset_server_clone))
                    .add_event::<LoadEvent>()
                    .add_systems(PreUpdate, check_loading_assets)
                    .add_systems(PostUpdate, clear_loaded_this_update);
            }
        }

        #[derive(Event)]
        pub enum LoadEvent {
            Everything,
            Something,
        }

        #[derive(Resource)]
        pub struct Assets {
            server: AssetServer,

            /// [Handle]s ([UntypedHandle]) which can be easily fetched. Itemized by category ([TypeId]) and a generic id.
            handles: HashMap<(TypeId, String), UntypedHandle>,

            /// Simple tracker for every currently loading asset.
            loading_assets: Vec<((TypeId, String), UntypedHandle)>,

            /// Assets which finished loading this update.
            loaded_this_update: Vec<((TypeId, String), UntypedHandle)>,
        }

        impl Assets {
            pub(super) fn new(asset_server_clone: AssetServer) -> Self {
                Self {
                    server: asset_server_clone,
                    handles: HashMap::default(),
                    loading_assets: Vec::default(),
                    loaded_this_update: Vec::default(),
                }
            }

            pub fn load<A: Asset>(
                &mut self,
                path: &'static str,
                id: Option<String>,
            ) -> Handle<A> {
                let id: String = match id {
                    Some(id) => id,
                    None => path.into(),
                };
                let type_id = TypeId::of::<A>();

                // Check if already loaded or registered
                if let Some(existing) = self.get::<A>((type_id, id.clone())) {
                    return existing;
                }

                let handle: Handle<A> = self.server.load::<A>(path);
                self.loading_assets
                    .push(((handle.type_id(), id.clone()), handle.clone().untyped()));
                self.add(handle.clone(), id);

                handle
            }

            pub fn add<A: Asset>(
                &mut self,
                handle: Handle<A>,
                id: impl Into<String>,
            ) -> Handle<A> {
                self.handles
                    .insert((handle.type_id(), id.into()), handle.clone().untyped());
                handle
            }

            pub fn get<A: Asset>(
                &self,
                (asset_type, id): (TypeId, impl Into<String>),
            ) -> Option<Handle<A>> {
                if let Some(handle) = self.handles.get(&(asset_type, id.into())) {
                    return Some(handle.clone().typed::<A>());
                }

                None
            }

            pub fn are_loaded(&self) -> bool {
                self.loading_assets.is_empty()
            }
        }

        pub(super) fn check_loading_assets(
            mut assets: ResMut<Assets>,
            mut ew: EventWriter<LoadEvent>,
        ) {
            let Assets {
                server,
                handles: _,
                loading_assets,
                loaded_this_update,
            } = &mut (*assets);

            loading_assets.retain(|((type_id, id), handle)| {
                let is_loaded = server
                    .get_load_state(handle.id())
                    .expect("Logic error somewhere...")
                    .is_loaded();

                if is_loaded {
                    loaded_this_update.push(((type_id.clone(), id.clone()), handle.clone()));
                }

                !is_loaded
            });

            if !loaded_this_update.is_empty() {
                match loading_assets.is_empty() {
                    true => {
                        ew.write(LoadEvent::Everything);
                    },
                    false => {
                        ew.write(LoadEvent::Something);
                    },
                };
            }
        }

        pub(super) fn clear_loaded_this_update(mut assets: ResMut<Assets>) {
            assets.loaded_this_update.clear();
        }
    }
}
