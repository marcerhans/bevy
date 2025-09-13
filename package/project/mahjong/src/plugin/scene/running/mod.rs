use crate::plugin::scene::Startup;
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<Menu>()
            .add_systems(OnEnter(Menu::Root), on_enter)
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
                *bg_color = Color::srgba(0.0, 0.0, 0.0, 0.5).into();
            },
            Interaction::None => {
                *bg_color = Color::srgba(0.5, 0.5, 0.5, 0.5).into();
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
            app.init_resource::<BevyIcon>()
                .add_systems(Startup, on_startup)
                .add_systems(OnEnter(Menu::About), on_enter)
                .add_systems(Update, on_action.run_if(in_state(Menu::About)));
        }
    }

    #[derive(Component)]
    struct Marker;

    #[derive(Resource, Default)]
    struct BevyIcon {
        handle: Option<Handle<Image>>,
    }

    #[derive(Component, Debug)]
    enum Action {
        Back,
    }

    fn on_startup(
        asset_server: Res<AssetServer>,
        mut image: ResMut<BevyIcon>,
    ) {
        assert!(image.handle.is_none());
        image.handle = Some(asset_server.load("misc/bevy_logo_fill.png"));
    }

    fn on_enter(
        mut commands: Commands,
        image: ResMut<BevyIcon>,
    ) {
        let font = (
            TextFont { ..default() },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        );

        commands.spawn((
            Marker,
            StateScoped(Menu::About),
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
            children![
                (
                    Node {
                        height: Val::Percent(32.0),
                        // aspect_ratio: Some(1.0),
                        ..default()
                    },
                    ImageNode {
                        image: image
                            .handle
                            .as_ref()
                            .expect("Bevy icon not loaded!")
                            .clone(),
                        ..default()
                    },
                ),
                (Text::new("Built with Bevy <3!"), font.clone()),
                (
                    Button,
                    Node {
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    Action::Back,
                    BorderRadius::all(Val::Px(8.0)),
                    children![(Text::new("Back"), font.clone()),]
                ),
            ],
        ));
    }

    fn on_action(
        query: Query<
            (&Interaction, &Action, &mut BackgroundColor),
            (Changed<Interaction>, With<Button>),
        >,
        mut next_state_sub: ResMut<NextState<Menu>>,
    ) {
        for (interaction, action, mut bg_color) in query {
            info!("{interaction:?} {action:?}");

            match interaction {
                Interaction::Pressed => {
                    *bg_color = Color::BLACK.into();
                    match action {
                        Action::Back => next_state_sub.set(Menu::Root),
                    }
                },
                Interaction::Hovered => {
                    *bg_color = Color::srgba(0.0, 0.0, 0.0, 0.5).into();
                },
                Interaction::None => {
                    *bg_color = Color::srgba(0.5, 0.5, 0.5, 0.5).into();
                },
            }
        }
    }
}
