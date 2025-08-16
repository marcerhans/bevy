use bevy::prelude::*;

use crate::plugin::screen::Screen;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_sub_state::<Menu>().add_plugins(root::Plugin);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates, Component)]
#[source(Screen = Screen::Menu)]
#[states(scoped_entities)]
enum Menu {
    #[default]
    Root,
    Settings,
}

mod root {
    use bevy::prelude::*;
    pub struct Plugin;

    impl bevy::prelude::Plugin for Plugin {
        fn build(
            &self,
            app: &mut App,
        ) {
            app.add_systems(OnEnter(super::Menu::Root), on_enter);
        }
    }

    fn on_enter(
        mut commands: Commands,
        // mut assets: ResMut<resource::asset::Assets>,
    ) {
        // let image = assets.load::<Image>("riichi_mahjong_tiles/ExampleBlack.png", "what");

        // let mut container = commands.spawn((
        //     super::Menu::Root,
        //     StateScoped(super::Menu::Root),
        //     Node {
        //         height: Val::Percent(100.0),
        //         width: Val::Percent(100.0),
        //         justify_content: JustifyContent::Center,
        //         align_items: AlignItems::Center,
        //         ..default()
        //     },
        //     ImageNode {
        //         image,
        //         // image_mode: todo!(),
        //         ..default()
        //     },
        // ));

        // container.with_child((Text::new("hej"),));

        // use crate::plugin::shared::component::prefab::*;

        fn button(children: impl Bundle) -> impl Bundle {
            (
                Node {
                    padding: UiRect::all(Val::Px(16.0)),
                    border: UiRect::all(Val::Px(8.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                BorderColor(Color::srgb(0.0, 1.0, 0.0)),
                BorderRadius::all(Val::Px(16.0)),
                children![children],
            )
        }

        commands.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(16.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.5, 0.5, 1.0)),
            children![
                button(Text::new("Play")),
                button(Text::new("Settings")),
                button(Text::new("Quit")),
            ],
        ));
    }
}
