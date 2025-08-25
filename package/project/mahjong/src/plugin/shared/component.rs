use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
    }
}

pub mod prefab {
    pub mod ui {
        use bevy::prelude::*;

        pub fn root() -> impl Bundle {
            (
                Node {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::BLACK),
            )
        }

        pub fn button(
            // image: Handle<Image>,
            // atlas: Handle<TextureAtlasLayout>,
            // slicer: &TextureSlicer,
            content: impl Bundle,
        ) -> impl Bundle {
            (
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::all(Val::Px(16.0)),
                    padding: UiRect::all(Val::Px(16.0)),
                    border: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                content,
            )
        }
    }
}
