use bevy::prelude::*;

use crate::plugin::screen::Screen;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(Greeter::default())
            .add_systems(OnEnter(Screen::Greeter), on_enter)
            .add_systems(Update, update);
    }
}

#[derive(Component)]
struct Marker;

#[derive(Resource)]
struct Greeter {
    timer: Timer,
}

impl Default for Greeter {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(4.0, TimerMode::Once),
        }
    }
}

fn on_enter(mut commands: Commands) {
    let text_color = TextColor(Color::srgba(1.0, 0.0, 0.0, 1.0));

    commands.spawn((
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
        children![
            (
                Marker,
                Text::new("Mah Dong Interactive Presents:"),
                text_color,
            ),
            (
                Marker,
                Text::new("Mah Jong"),
                text_color,
            ),
        ],
    ));
}

fn update(
    timer: Res<Time>,
    mut screen: ResMut<NextState<Screen>>,
    mut greeter: ResMut<Greeter>,
    mut colors: Query<&mut TextColor, With<Marker>>,
) {
    warn!("hej");
    greeter.timer.tick(timer.delta());

    if greeter.timer.finished() {
        screen.set(Screen::Menu);
    } else {
        warn!("alpha");
        for mut color in colors {
            let alpha = color.0.alpha();
            color.0.set_alpha(0.0);
        }
    }
}
