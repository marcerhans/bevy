use crate::plugin::scene::Startup;
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<Menu>()
            .add_systems(OnEnter(Startup::Running), on_enter)
            .add_plugins(root::Plugin);
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

#[derive(Component, Debug)]
enum MenuMarker {
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

    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            parent.spawn((Button, MenuMarker::Play, Text::new("Play"), font.clone()));
            parent.spawn((
                Button,
                MenuMarker::Settings,
                Text::new("Settings"),
                font.clone(),
            ));
            parent.spawn((Button, MenuMarker::Quit, Text::new("Quit"), font.clone()));
        });
}

pub mod root {
    use super::{Menu, MenuMarker};
    use bevy::prelude::*;

    pub struct Plugin;

    impl bevy::prelude::Plugin for Plugin {
        fn build(
            &self,
            app: &mut App,
        ) {
            app.add_systems(Update, update.run_if(in_state(Menu::Root)));
        }
    }

    fn update(
        query: Query<
            (&Interaction, &MenuMarker, &mut BackgroundColor),
            (Changed<Interaction>, With<Button>),
        >,
        mut event_writer: EventWriter<AppExit>,
        mut next_state: ResMut<NextState<crate::plugin::scene::Startup>>,
    ) {
        for (interaction, menu_marker, mut bg_color) in query {
            info!("{interaction:?} {menu_marker:?}");

            match interaction {
                Interaction::Pressed => {
                    *bg_color = Color::BLACK.into();
                    match menu_marker {
                        MenuMarker::Play => {
                            next_state.set(crate::plugin::scene::Startup::Greeter);
                        },
                        MenuMarker::Settings => {},
                        MenuMarker::Quit => {
                            event_writer.write(AppExit::Success);
                        },
                    }
                },
                Interaction::Hovered => {
                    bg_color.0.set_alpha(0.5);
                },
                Interaction::None => {
                    *bg_color = Color::srgba(0.0, 0.0, 0.0, 0.0).into();
                },
            }
        }
    }
}
