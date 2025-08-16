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

fn on_enter(mut commands: Commands) {
    use crate::plugin::shared::component::prefab::*;
    commands.spawn((
        Marker,
        StateScoped(Screen::Greeter),
        ui::root(),
        children![ui::button(children![
            (
                Marker,
                Node::default(),
                Text::new("Mah Dong Inc. Presents:")
            ),
            (Marker, Node::default(), Text::new("Mah Jong")),
        ],)],
    ));
}

fn update(
    timer: Res<Time>,
    mut text_color: Query<&mut TextColor>,
    mut image_node: Query<&mut ImageNode>,
    mut screen: ResMut<NextState<Screen>>,
    mut greeter: ResMut<Greeter>,
) {
    // TMP
    screen.set(Screen::Menu);
    return;

    greeter.timer.tick(timer.delta());

    if greeter.timer.finished() {
        screen.set(Screen::Menu);
    } else {
        let ef = EaseFunction::CubicIn;
        let timer = &greeter.timer;
        let gradient = 1.0 - ef.sample_clamped(timer.fraction());

        for mut text_color in text_color {
            text_color.0.set_alpha(gradient);
        }

        for mut image_node in image_node {
            image_node.color.set_alpha(gradient);
        }
    }
}
