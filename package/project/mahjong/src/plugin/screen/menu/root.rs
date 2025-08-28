use bevy::{color::palettes::basic::*, prelude::*};
use bevy_enhanced_input::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_input_context::<Marker>()
            .add_systems(OnEnter(super::Menu::Root), on_enter)
            .add_systems(Update, button_system);
    }
}

#[derive(Component)]
struct Marker;

fn on_enter(mut commands: Commands) {
    use crate::plugin::shared::component::prefab::*;
    commands.spawn((
        Marker,
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
            ui::button(Text::new("hej"),),
            ui::button(Text::new("på")),
            ui::button(Text::new("dig")),
            ui::button(Text::new(":)")),
        ],
    ));
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            },
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            },
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            },
        }
    }
}
