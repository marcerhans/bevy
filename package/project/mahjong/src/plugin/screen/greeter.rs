use bevy::prelude::*;

use crate::plugin::{screen::Screen, shared::resource};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems(OnEnter(Screen::Greeter), spawn_screen)
            .add_systems(Update, update.run_if(update_if));
    }
}

#[derive(Component)]
struct Greeter;

fn spawn_screen(mut commands: Commands) {
    commands
        .spawn((
            Greeter,
            StateScoped(Screen::Greeter),
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_child(Text::new("Loading..."));
}

fn update_if(er: EventReader<resource::asset::LoadEvent>) -> bool {
    !er.is_empty()
}

fn update(
    mut er: EventReader<resource::asset::LoadEvent>,
    mut screen: ResMut<NextState<Screen>>,
) {
    for event in er.read() {
        if matches!(event, resource::asset::LoadEvent::Everything) {
            debug!("Woho!");
            screen.set(Screen::Menu);
        }
    }
}
