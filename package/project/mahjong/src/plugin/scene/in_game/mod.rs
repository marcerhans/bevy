use std::f32::consts::FRAC_PI_4;

use crate::plugin::scene::main_menu::MainMenu;
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<InGame>()
            .add_systems(OnEnter(InGame::Root), on_enter)
            .add_systems(Update, update.run_if(in_state(InGame::Root)));
    }
}

#[derive(Component, SubStates, Hash, Eq, PartialEq, Clone, Debug, Default)]
#[source(MainMenu = MainMenu::Play)]
#[states(scoped_entities)]
pub enum InGame {
    #[default]
    Root,
}

#[derive(Component)]
struct Marker;

fn on_enter(
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    let mut gizmo = GizmoAsset::default();
    gizmo.arc_2d(
        Isometry2d::IDENTITY,
        FRAC_PI_4,
        1.,
        Color::srgb(1.0, 0.0, 0.0),
    );

    // Arcs have 32 line-segments by default.
    // You may want to increase this for larger arcs.
    gizmo
        .arc_2d(
            Isometry2d::IDENTITY,
            FRAC_PI_4,
            100.,
            Color::srgb(0.0, 1.0, 0.0),
        )
        .resolution(3);

    commands.spawn(Gizmo {
        handle: gizmo_assets.add(gizmo),
        line_config: GizmoLineConfig {
            width: 4.0,
            ..default()
        },
        ..default()
    });
    // let font = (
    //     TextFont { ..default() },
    //     TextColor(Color::srgb(0.9, 0.9, 0.9)),
    // );

    // commands
    //     .spawn((
    //         Marker,
    //         StateScoped(MainMenu::Root),
    //         Node {
    //             width: Val::Percent(100.0),
    //             height: Val::Percent(100.0),
    //             justify_content: JustifyContent::Center,
    //             align_items: AlignItems::Center,
    //             flex_direction: FlexDirection::Column,
    //             row_gap: Val::Px(8.0),
    //             ..default()
    //         },
    //         BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    //     ))
    //     .with_children(|parent| {
    //         let common = (
    //             Button,
    //             Node {
    //                 justify_content: JustifyContent::Center,
    //                 align_items: AlignItems::Center,
    //                 flex_direction: FlexDirection::Column,
    //                 padding: UiRect::all(Val::Px(8.0)),
    //                 ..default()
    //             },
    //             BorderRadius::all(Val::Px(8.0)),
    //             font.clone(),
    //         );

    //         parent.spawn((common.clone(), MainMenu::Play, children![Text::new("Play")]));
    //         parent.spawn((
    //             common.clone(),
    //             MainMenu::Settings,
    //             children![Text::new("Settings")],
    //         ));
    //         parent.spawn((
    //             common.clone(),
    //             MainMenu::About,
    //             children![Text::new("About")],
    //         ));
    //         parent.spawn((common.clone(), MainMenu::Quit, children![Text::new("Quit")]));
    //     });
}

fn update(
    query: Query<
        (&Interaction, &MainMenu, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut event_writer: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<crate::plugin::scene::Startup>>,
    mut next_state_sub: ResMut<NextState<MainMenu>>,
) {
    // for (interaction, menu_marker, mut bg_color) in query {
    //     info!("{interaction:?} {menu_marker:?}");

    //     match interaction {
    //         Interaction::Pressed => {
    //             *bg_color = Color::BLACK.into();
    //             match menu_marker {
    //                 MainMenu::Root => unreachable!(),
    //                 MainMenu::Play => {
    //                     next_state.set(crate::plugin::scene::Startup::Greeter);
    //                 },
    //                 MainMenu::Settings => {},
    //                 MainMenu::About => {
    //                     next_state_sub.set(MainMenu::About);
    //                 },
    //                 MainMenu::Quit => {
    //                     event_writer.write(AppExit::Success);
    //                 },
    //             }
    //         },
    //         Interaction::Hovered => {
    //             *bg_color = Color::srgba(0.0, 0.0, 0.0, 0.5).into();
    //         },
    //         Interaction::None => {
    //             *bg_color = Color::srgba(0.5, 0.5, 0.5, 0.5).into();
    //         },
    //     }
    // }
}
