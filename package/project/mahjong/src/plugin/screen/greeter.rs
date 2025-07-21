use bevy::prelude::*;

use crate::plugin::{screen::Screen, shared::resource::asset};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(Greeter::default())
            .add_systems(OnEnter(Screen::Greeter), on_enter)
            .add_systems(Update, update);
    }
}

#[derive(Component)]
struct Marker;

#[derive(Resource)]
struct Greeter {
    timer: Timer,
}

impl Default for Greeter {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(4.0, TimerMode::Once),
        }
    }
}

fn on_enter(
    mut commands: Commands,
    mut assets: ResMut<asset::Assets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let text_color = TextColor(Color::srgba(0.9, 0.8, 0.0, 1.0));

    let image = assets.load::<Image>("ui/button.png", "image::button");
    let atlas = assets.add(
        texture_atlases.add(TextureAtlasLayout::from_grid(
            UVec2::splat(32),
            3,
            3,
            None,
            None,
        )),
        "atlas::button",
    );
    let slicer = TextureSlicer {
        border: BorderRect::all(32.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    commands.spawn((
        Marker,
        StateScoped(Screen::Greeter),
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Marker,
                ImageNode::from_atlas_image(image, TextureAtlas::from(atlas))
                    .with_mode(NodeImageMode::Sliced(slicer.clone())),
            ),
            (
                Marker,
                Text::new("Mah Dong Interactive Presents:"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                text_color,
            ),
            (
                Marker,
                Text::new("Mah Jong"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                text_color,
            ),
        ],
    ));
}

fn update(
    timer: Res<Time>,
    colors: Query<&mut TextColor, With<Marker>>,
    mut screen: ResMut<NextState<Screen>>,
    mut greeter: ResMut<Greeter>,
) {
    greeter.timer.tick(timer.delta());

    if greeter.timer.finished() {
        screen.set(Screen::Menu);
    } else {
        let timer = &greeter.timer;
        let div = 4.0;
        let extra = timer.duration().as_secs_f32() / div;
        let extra_time = 4.0;
        let gradient = (timer.fraction_remaining() + extra
            - (extra * (timer.elapsed_secs() / extra_time)).min(extra))
        .min(1.0);

        for mut color in colors {
            color.0.set_alpha(gradient.powi(5));
        }
    }
}
