use crate::plugin::scene::Startup;
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<Menu>()
            .add_systems(OnEnter(Startup::Running), on_enter);
    }
}

#[derive(SubStates, Hash, Eq, PartialEq, Clone, Debug, Default)]
#[source(Startup = Startup::Running)]
#[states(scoped_entities)]
pub enum Menu {
    #[default]
    Root,
    Play,
    Settings,
    Quit,
}

#[derive(Component)]
struct Marker;

fn on_enter(mut commands: Commands) {
    let font = (
        TextFont { ..default() },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    );

    commands.spawn((
        Marker,
        StateScoped(Menu::Root),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        children![
            (Text::new("Play"), font.clone()),
            (Text::new("Settings"), font.clone()),
            (Text::new("Quit"), font.clone()),
        ],
    ));
}
