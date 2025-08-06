use bevy::prelude::*;

use crate::plugin::{
    screen::Screen,
    shared::resource::asset::{self, Load},
};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(Greeter::default())
            .add_systems(Startup, on_startup)
            .add_systems(OnEnter(Screen::Greeter), on_enter)
            .add_systems(Update, update);
    }
}

#[derive(Component)]
struct Marker;

#[derive(Resource)]
struct Greeter {
    timer: Timer,
    texture_atlas: Handle<TextureAtlasLayout>,
}

impl Greeter {
    const TIMER_DURATION: f32 = 5.0;
}

impl Default for Greeter {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(Self::TIMER_DURATION, TimerMode::Once),
            texture_atlas: default(),
        }
    }
}

mod prefab {
    use super::*;

    pub fn root() -> impl Bundle {
        (
            Marker,
            StateScoped(Screen::Greeter),
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::BLACK),
        )
    }

    pub fn button(
        image: Handle<Image>,
        atlas: Handle<TextureAtlasLayout>,
        slicer_large: &TextureSlicer,
        slicer_small: &TextureSlicer,
        content: impl Bundle,
    ) -> impl Bundle {
        (
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            ImageNode {
                image: image.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: 0,
                    layout: atlas.clone(),
                }),
                image_mode: NodeImageMode::Sliced(slicer_large.clone()),
                ..default()
            },
            children![(
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                content,
            )],
        )
    }
}

fn on_startup(
    asset_server: Res<AssetServer>,
    mut asset_atlas: ResMut<Assets<TextureAtlasLayout>>,
    mut resource_image: ResMut<asset::atlas::X384>,
    mut resource_greeter: ResMut<Greeter>,
) {
    #[inline]
    fn load_assets(
        asset_server: &Res<AssetServer>,
        resource_image: &mut ResMut<asset::atlas::X384>,
    ) {
        resource_image.0 = asset::atlas::X384::load(&asset_server);
    }

    #[inline]
    fn compute_and_store_atlas_layout(
        resource_greeter: &mut ResMut<Greeter>,
        asset_atlas: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let tile_size = 256 * 3;
        let rows = 1;
        let cols = 1;
        let padding = 1;
        resource_greeter.texture_atlas = asset_atlas.add(TextureAtlasLayout::from_grid(
            UVec2::splat(tile_size),
            cols,
            rows,
            Some(UVec2::splat(padding)),
            None,
        ));
    }

    load_assets(&asset_server, &mut resource_image);
    compute_and_store_atlas_layout(&mut resource_greeter, &mut asset_atlas);
}

fn on_enter(
    mut commands: Commands,
    resource_image: ResMut<asset::atlas::X384>,
    resource_greeter: ResMut<Greeter>,
) {
    let slicer_small = TextureSlicer {
        border: BorderRect::all(256 as f32),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Tile { stretch_value: 2.0 },
        max_corner_scale: 1.0,
    };
    let slicer_large = TextureSlicer {
        border: BorderRect::all(256 as f32),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Tile {
            stretch_value: 10.0,
        },
        max_corner_scale: 1.0,
    };

    use prefab::*;
    commands.spawn((
        root(),
        children![button(
            resource_image.0.clone(),
            resource_greeter.texture_atlas.clone(),
            &slicer_large,
            &slicer_small,
            children![
                (
                    Marker,
                    Node::default(),
                    Text::new("Mah Dong Inc. Presents:")
                ),
                (Marker, Node::default(), Text::new("Mah Jong")),
            ],
        )],
    ));
}

fn update(
    timer: Res<Time>,
    mut text_color: Query<&mut TextColor>,
    mut image_node: Query<&mut ImageNode>,
    mut screen: ResMut<NextState<Screen>>,
    mut greeter: ResMut<Greeter>,
) {
    greeter.timer.tick(timer.delta());

    if greeter.timer.finished() {
        screen.set(Screen::Menu);
    } else {
        let ef = EaseFunction::CubicIn;
        let timer = &greeter.timer;
        let gradient = 1.0 - ef.sample_clamped(timer.fraction());

        for (mut text_color, mut image_node) in text_color.iter_mut().zip(image_node.reborrow()) {
            text_color.0.set_alpha(gradient);
            image_node.color.set_alpha(gradient);
        }
    }
}
