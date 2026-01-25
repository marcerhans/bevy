use bevy::prelude::*;

#[derive(Message, Deref, DerefMut)]
struct OnClick(Entity);

#[derive(Resource, Deref, DerefMut)]
struct Entities(Option<(Entity, Entity)>);

mod tile {
    use super::*;

    #[derive(Component)]
    pub struct Marker;

    #[derive(Component)]
    pub struct Position;

    #[derive(Component)]
    pub struct Inactive;

    #[derive(Component)]
    pub enum Variant {
        A,
        B,
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<OnClick>()
        .insert_resource(Entities(None))
        .add_systems(Update, stimulator)
        .add_systems(Update, (system_a, system_b).chain().after(stimulator))
        .run();
}

fn stimulator(
    mut not_first: Local<bool>,
    mut e: ResMut<Entities>,
    mut commands: Commands,
    mut msg_writer: MessageWriter<OnClick>,
) {
    if !*not_first {
        let mut e_new = commands.spawn((tile::Marker, tile::Position, tile::Variant::A));

        e.0 = Some((e_new.id(), e_new.clone_and_spawn().id()));
        *not_first = true;
    }

    msg_writer.write(OnClick(e.unwrap().0));
    msg_writer.write(OnClick(e.unwrap().1));
}

fn system_a(
    mut commands: Commands,
    mut msg_onclick: MessageReader<OnClick>,
    mut tile_query: Query<
        (Entity, &mut tile::Position),
        (Without<tile::Inactive>, With<tile::Marker>),
    >,
) {
    let Some(origin) = msg_onclick.read().next() else {
        panic!();
    };
    let origin = **origin;

    let Some(prev_tile) = msg_onclick.read().next() else {
        panic!();
    };
    let prev_tile = **prev_tile;

    let _ = tile_query.get_mut(origin).unwrap();
    commands.entity(prev_tile).insert(tile::Inactive);
    commands.entity(origin).insert(tile::Inactive);
}

fn system_b(
    mut commands: Commands,
    e: Res<Entities>,
) {
    commands.entity(e.unwrap().0).remove::<tile::Inactive>();
    commands.entity(e.unwrap().1).remove::<tile::Inactive>();
}
