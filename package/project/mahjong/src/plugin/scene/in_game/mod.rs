use crate::plugin::scene::main_menu::MainMenu;
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<InGame>()
            .add_systems(
                OnEnter(InGame::Root),
                (spawn_background, spawn_tiles, spawn_buttons),
            )
            .add_systems(Update, resize_background.run_if(in_state(InGame::Root)));
    }
}

#[derive(SubStates, Default, Debug, Hash, Eq, PartialEq, Clone)]
#[source(MainMenu = MainMenu::Play)]
#[states(scoped_entities)]
pub enum InGame {
    #[default]
    Root,
}

mod marker {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Background;

    #[derive(Component)]
    pub struct Button;
}

mod tile {
    use bevy::prelude::*;

    pub mod asset {
        pub mod texture {
            pub const BORDER: &'static str = "misc/rev2/Tile4_700x1000.png";
            pub const ALLIANCE: &'static str = "misc/rev2/Alliance_1104x882.png";
            pub const HORDE: &'static str = "misc/rev2/Horde_740x1093.png";
            pub const BOTTOM_BORDER_PERCENTAGE_Y: f32 = 175.0 / 1000.0; // (Just the "thickness" of the tile, excluding the border)
            pub const LEFT_BORDER_PERCENTAGE_X: f32 = 124.0 / 700.0; // (Just the "thickness" of the tile, excluding the border)
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

    #[derive(Component, Deref, DerefMut)]
    pub struct Variant(u32);
}

pub fn spawn_background(
    mut commands: Commands,
    projection: Query<&Projection, With<Camera>>,
    asset_server: Res<AssetServer>,
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let handle: Handle<Image> = asset_server.load("misc/rev2/original/Arthas_LichKing_GPT2.png");

    commands.spawn((
        marker::Background,
        DespawnOnExit(InGame::Root),
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
    ));
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
}

pub fn spawn_buttons(
    mut commands: Commands,
    projection: Query<&Projection, With<Camera>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };
}

fn resize_background(
    mut transform: Query<(&mut Transform, &mut Sprite), With<marker::Background>>,
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
