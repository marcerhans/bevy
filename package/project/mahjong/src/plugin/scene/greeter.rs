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
            .add_systems(Update, on_update.run_if(in_state(Startup::Greeter)));
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
struct BackgroundSprite;

#[derive(Component)]
struct Marker;

fn on_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<TimerRes>,
    projection: Query<&Projection, With<Camera>>,
) {
    // Spawn title/greeter screen
    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let greeter_asset: Handle<Image> = asset_server.load("misc/rev2/original/Greeter.png");

    commands.spawn((
        DespawnOnExit(Startup::Greeter),
        Marker,
        BackgroundSprite,
        Sprite {
            custom_size: Some(Vec2::new(projection.area.width(), projection.area.height())),
            ..Sprite::from_image(greeter_asset)
        },
    ));

    // Initialize timer
    timer.inner.reset();
}

fn on_update(
    time: Res<Time>,
    mut timer: ResMut<TimerRes>,
    mut next_state: ResMut<NextState<Startup>>,
    mut transform: Query<&mut Sprite, With<BackgroundSprite>>,
    projection: Query<&Projection, With<Camera>>,
) {
    timer.inner.tick(time.delta());

    if timer.inner.just_finished() {
        next_state.set(Startup::MainMenu);
    }

    let Some(Projection::Orthographic(projection)) = projection.iter().next() else {
        panic!();
    };

    let Some(mut sprite) = transform.iter_mut().next() else {
        return;
    };

    sprite.custom_size = Some(Vec2 {
        x: projection.area.width(),
        y: projection.area.height(),
    });
}
