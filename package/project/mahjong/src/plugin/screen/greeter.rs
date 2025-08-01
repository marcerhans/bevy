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

mod prefab {
    use super::*;

    pub fn root() -> impl Bundle {
        (
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
            BackgroundColor(Color::BLACK),
        )
    }

    pub fn button(
        image: Handle<Image>,
        atlas: Handle<TextureAtlasLayout>,
        slicer_large: &TextureSlicer,
        slicer_small: &TextureSlicer,
        content: impl Bundle,
    ) -> impl Bundle {
        (
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            ImageNode {
                image: image.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: 1,
                    layout: atlas.clone(),
                }),
                image_mode: NodeImageMode::Sliced(slicer_large.clone()),
                ..default()
            },
            children![(
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                ImageNode {
                    image: image.clone(),
                    texture_atlas: Some(TextureAtlas {
                        index: 0,
                        layout: atlas.clone(),
                    }),
                    image_mode: NodeImageMode::Sliced(slicer_small.clone()),
                    ..default()
                },
                content,
                // children![content]
            )],
        )
    }
}

fn on_enter(
    mut commands: Commands,
    mut assets: ResMut<asset::Assets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tile_size = 128 * 3;
    let rows = 1;
    let cols = 2;
    let padding = 2;
    let image = assets.load::<Image>("atlas/384.png", "image::atlas::384");
    let atlas = assets.add(
        texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(tile_size),
            cols,
            rows,
            Some(UVec2::splat(padding)),
            None,
        )),
        "texture_atlas_layout::button",
    );
    let slicer_small = TextureSlicer {
        border: BorderRect::all(128 as f32),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Tile { stretch_value: 2.0 },
        max_corner_scale: 1.0,
    };
    let slicer_large = TextureSlicer {
        border: BorderRect::all(128 as f32),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Tile {
            stretch_value: 10.0,
        },
        max_corner_scale: 1.0,
    };

    use prefab::*;
    commands.spawn((
        root(),
        children![button(
            image,
            atlas,
            &slicer_large,
            &slicer_small,
            children![
                (Node::default(), Text::new("Mah Dong Inc. Presents:")),
                (Node::default(), Text::new("Mah Jong")),
            ],
        )],
    ));
}

fn update(
    timer: Res<Time>,
    colors: Query<&mut TextColor, With<Marker>>,
    mut screen: ResMut<NextState<Screen>>,
    mut greeter: ResMut<Greeter>,
) {
    // greeter.timer.tick(timer.delta());

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
