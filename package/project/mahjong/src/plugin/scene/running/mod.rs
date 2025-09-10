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
            .add_systems(Update, update.run_if(in_state(Menu::Root)))
            .add_systems(OnExit(Menu::Root), on_exit)
            .add_plugins(about::Plugin);
    }
}

#[derive(Component, SubStates, Hash, Eq, PartialEq, Clone, Debug, Default)]
#[source(Startup = Startup::Running)]
#[states(scoped_entities)]
pub enum Menu {
    #[default]
    Root,
    Play,
    Settings,
    About,
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
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        ))
        .with_children(|parent| {
            let common = (
                Button,
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                BorderRadius::all(Val::Px(8.0)),
                font.clone(),
            );

            parent.spawn((common.clone(), Menu::Play, children![Text::new("Play")]));
            parent.spawn((
                common.clone(),
                Menu::Settings,
                children![Text::new("Settings")],
            ));
            parent.spawn((common.clone(), Menu::About, children![Text::new("About")]));
            parent.spawn((common.clone(), Menu::Quit, children![Text::new("Quit")]));
        });
}

fn update(
    query: Query<(&Interaction, &Menu, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut event_writer: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<crate::plugin::scene::Startup>>,
    mut next_state_sub: ResMut<NextState<Menu>>,
) {
    for (interaction, menu_marker, mut bg_color) in query {
        info!("{interaction:?} {menu_marker:?}");

        match interaction {
            Interaction::Pressed => {
                *bg_color = Color::BLACK.into();
                match menu_marker {
                    Menu::Root => unreachable!(),
                    Menu::Play => {
                        next_state.set(crate::plugin::scene::Startup::Greeter);
                    },
                    Menu::Settings => {},
                    Menu::About => {
                        next_state_sub.set(Menu::About);
                    },
                    Menu::Quit => {
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

fn on_exit() {
    info!("hej");
}

pub mod about {
    use super::Menu;
    use bevy::prelude::*;

    pub struct Plugin;

    impl bevy::prelude::Plugin for Plugin {
        fn build(
            &self,
            app: &mut App,
        ) {
            app.add_systems(OnEnter(Menu::About), on_enter);
        }
    }

    fn on_enter(mut commands: Commands) {
        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            children![(
                Node {
                    width: Val::Px(42.0),
                    height: Val::Px(42.0),
                    ..default()
                },
                ImageNode::default(),
            )],
        ));
    }
}
