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

    gizmo.rect_2d(
        Isometry2d::IDENTITY,
        Vec2::new(200.0, 200.0),
        Color::srgb(0.0, 0.0, 1.0),
    );

    commands
        .spawn((
            Gizmo {
                handle: gizmo_assets.add(gizmo),
                line_config: GizmoLineConfig {
                    width: 10.0,
                    ..default()
                },
                ..default()
            },
            Pickable::default(),
        ))
        .observe(|trigger: Trigger<Pointer<Click>>| {
            info!("Clicked!");
        });

    commands
        .spawn((
            Sprite::from_color(Color::WHITE, Vec2::splat(300.0)),
            Pickable::default(),
        ))
        .observe(|trigger: Trigger<Pointer<Click>>| {
            info!("Sprite click!");
        });
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
