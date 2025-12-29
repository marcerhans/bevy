use crate::plugin::scene::main_menu::MainMenu;
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<InGame>();
        // .add_systems(OnEnter(InGame::Root), spawn_tiles);
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
        }
    }
}
