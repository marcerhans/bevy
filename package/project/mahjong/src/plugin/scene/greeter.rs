use std::time::Duration;

use crate::plugin::scene::Startup;
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.init_resource::<TimerRes>()
            .add_systems(OnEnter(Startup::Greeter), on_enter)
            .add_systems(Update, on_update);
    }
}

#[derive(Resource)]
struct TimerRes {
    inner: Timer,
}

impl Default for TimerRes {
    fn default() -> Self {
        Self {
            inner: Timer::new(Duration::from_secs(4), TimerMode::Once),
        }
    }
}

#[derive(Component)]
struct Marker;

fn on_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<TimerRes>,
    projection: Single<&Projection, With<Camera>>,
) {
    // Spawn title/greeter screen
    let Projection::Orthographic(projection) = *projection else {
        panic!();
    };

    let greeter_asset: Handle<Image> = asset_server.load("misc/rev2/original/Greeter.png");

    commands.spawn((
        DespawnOnExit(Startup::Greeter),
        Marker,
        Sprite {
            custom_size: Some(Vec2::new(projection.area.width(), projection.area.height())),
            ..Sprite::from_image(greeter_asset)
        },
    ));

    // Initialize timer
    timer.inner.reset();

    // let font = (
    //     TextFont { ..default() },
    //     TextColor(Color::srgb(0.9, 0.9, 0.9)),
    // );

    // commands.spawn((
    //     DespawnOnExit(Startup::Greeter),
    //     Marker,
    //     Node {
    //         width: Val::Percent(100.0),
    //         height: Val::Percent(100.0),
    //         justify_content: JustifyContent::Center,
    //         align_items: AlignItems::Center,
    //         flex_direction: FlexDirection::Column,
    //         ..default()
    //     },
    //     BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    //     children![
    //         (Text::new("Mah Dong Inc. Presents:"), font.clone()),
    //         (Text::new("Mah Jong"), font.clone())
    //     ],
    // ));
}

fn on_update(
    time: Res<Time>,
    mut timer: ResMut<TimerRes>,
    mut next_state: ResMut<NextState<Startup>>,
) {
    timer.inner.tick(time.delta());

    if timer.inner.just_finished() {
        next_state.set(Startup::MainMenu);
    }
}
