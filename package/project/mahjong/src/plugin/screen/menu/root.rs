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

#[derive(Component)]
struct Marker;

fn on_enter(
    mut commands: Commands,
) {
    use crate::plugin::shared::component::prefab::*;
    commands.spawn((
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
                    border: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        BorderColor(Color::srgb(0.5, 0.5, 0.0)),
        children![
            ui::button(Text::new("hej")),
            ui::button(Text::new("på")),
            ui::button(Text::new("dig")),
            ui::button(Text::new(":)")),
        ],
    ));
}
