// use super::MainMenu;
// use bevy::prelude::*;

// pub struct Plugin;

// impl bevy::prelude::Plugin for Plugin {
//     fn build(
//         &self,
//         app: &mut App,
//     ) {
//         app.init_resource::<BevyIcon>()
//             .add_systems(Startup, on_startup)
//             .add_systems(OnEnter(MainMenu::About), on_enter)
//             .add_systems(Update, on_action.run_if(in_state(MainMenu::About)));
//     }
// }

// #[derive(Component)]
// struct Marker;

// #[derive(Resource, Default)]
// struct BevyIcon {
//     handle: Option<Handle<Image>>,
// }

// #[derive(Component, Debug)]
// enum Action {
//     Back,
// }

// fn on_startup(
//     asset_server: Res<AssetServer>,
//     mut image: ResMut<BevyIcon>,
// ) {
//     assert!(image.handle.is_none());
//     image.handle = Some(asset_server.load("misc/bevy_logo_fill.png"));
// }

// fn on_enter(
//     mut commands: Commands,
//     image: ResMut<BevyIcon>,
// ) {
//     let font = (
//         TextFont { ..default() },
//         TextColor(Color::srgb(0.9, 0.9, 0.9)),
//     );

//     commands.spawn((
//         Marker,
//         DespawnOnExit(MainMenu::About),
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
//         children![
//             (
//                 Node {
//                     height: Val::Percent(32.0),
//                     // aspect_ratio: Some(1.0),
//                     ..default()
//                 },
//                 ImageNode {
//                     image: image
//                         .handle
//                         .as_ref()
//                         .expect("Bevy icon not loaded!")
//                         .clone(),
//                     ..default()
//                 },
//             ),
//             (Text::new("Built with Bevy <3!"), font.clone()),
//             (
//                 Button,
//                 Node {
//                     padding: UiRect::all(Val::Px(8.0)),
//                     ..default()
//                 },
//                 Action::Back,
//                 BorderRadius::all(Val::Px(8.0)),
//                 children![(Text::new("Back"), font.clone()),]
//             ),
//         ],
//     ));
// }

// fn on_action(
//     query: Query<
//         (&Interaction, &Action, &mut BackgroundColor),
//         (Changed<Interaction>, With<Button>),
//     >,
//     mut next_state_sub: ResMut<NextState<MainMenu>>,
// ) {
//     for (interaction, action, mut bg_color) in query {
//         info!("{interaction:?} {action:?}");

//         match interaction {
//             Interaction::Pressed => {
//                 *bg_color = Color::BLACK.into();
//                 match action {
//                     Action::Back => next_state_sub.set(MainMenu::Root),
//                 }
//             },
//             Interaction::Hovered => {
//                 *bg_color = Color::srgba(0.0, 0.0, 0.0, 0.5).into();
//             },
//             Interaction::None => {
//                 *bg_color = Color::srgba(0.5, 0.5, 0.5, 0.5).into();
//             },
//         }
//     }
// }
