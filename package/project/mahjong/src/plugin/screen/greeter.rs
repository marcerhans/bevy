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

fn bundle_button() -> impl Bundle {
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
        BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
    )
        // .(|parent| {
        //     parent
        //         .spawn((
        //             Node {
        //                 padding: UiRect::all(Val::Px(4.0)),
        //                 ..default()
        //             },
        //             ImageNode {
        //                 image: image.clone(),
        //                 texture_atlas: Some(TextureAtlas {
        //                     index: 1,
        //                     layout: atlas.clone(),
        //                 }),
        //                 image_mode: NodeImageMode::Sliced(slicer_large.clone()),
        //                 ..default()
        //             },
        //         ))
        //         .with_children(|parent| {
        //             parent.spawn((
        //                 Node {
        //                     padding: UiRect::all(Val::Px(10.)),
        //                     width: Val::Px(1000.),
        //                     ..default()
        //                 },
        //                 ImageNode {
        //                     image: image.clone(),
        //                     texture_atlas: Some(TextureAtlas {
        //                         index: 0,
        //                         layout: atlas.clone(),
        //                     }),
        //                     image_mode: NodeImageMode::Sliced(slicer_small.clone()),
        //                     ..default()
        //                 },
        //                 children![Text::new("hej")],
        //             ));
        //         });
        // })
}

fn on_enter(
    mut commands: Commands,
    mut assets: ResMut<asset::Assets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let text_color = TextColor(Color::srgba(0.9, 0.8, 0.0, 1.0));

    let tile_size = 32;
    let rows = 1;
    let cols = 2;
    let padding = 2;
    let image = assets.load::<Image>("ui/atlas.png", "image::ui::atlas");
    let atlas = assets.add(
        texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(tile_size * 3 - padding),
            cols,
            rows,
            Some(UVec2::splat(2)),
            None,
        )),
        "texture_atlas_layout::button",
    );
    let slicer_small = TextureSlicer {
        border: BorderRect::all(tile_size as f32),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };
    let slicer_large = TextureSlicer {
        border: BorderRect::all(tile_size as f32),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 10.0,
    };

    commands.spawn(bundle_button());

    // commands.spawn((
    //     Marker,
    //     StateScoped(Screen::Greeter),
    //     Node {
    //         height: Val::Percent(100.0),
    //         width: Val::Percent(100.0),
    //         justify_content: JustifyContent::Center,
    //         align_items: AlignItems::Center,
    //         flex_direction: FlexDirection::Column,
    //         ..default()
    //     },
    //     // BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
    //     BackgroundColor(Color::WHITE),
    //     children![
    //         (
    //             Marker,
    //             ImageNode::from_atlas_image(
    //                 image,
    //                 TextureAtlas {
    //                     index: 0,
    //                     layout: atlas.clone(),
    //                 }
    //             )
    //             .with_mode(NodeImageMode::Sliced(slicer.clone())),
    //             children![Node {
    //                 width: Val::Px(1000.),
    //                 height: Val::Px(100.),
    //                 ..default()
    //             }]
    //         ),
    //         (
    //             Marker,
    //             Text::new("Mah Dong Interactive Presents:"),
    //             TextFont {
    //                 font_size: 32.0,
    //                 ..default()
    //             },
    //             text_color,
    //         ),
    //         (
    //             Marker,
    //             Text::new("Mah Jong"),
    //             TextFont {
    //                 font_size: 64.0,
    //                 ..default()
    //             },
    //             text_color,
    //         ),
    //     ],
    // ));
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
