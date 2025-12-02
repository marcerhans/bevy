use bevy::prelude::*;

#[derive(Message, Deref, DerefMut)]
struct OnClick(Entity);

#[derive(Resource, Deref, DerefMut)]
struct PreviouslySelectedTile(pub Option<Entity>);

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
        .insert_resource(PreviouslySelectedTile(None))
        .add_systems(Update, stimulator)
        .add_systems(Update, (system_a, system_b))
        .run();
}

fn init(mut commands: Commands) {}

fn stimulator(
    mut not_first: Local<bool>,
    mut e1: Local<Option<Entity>>,
    mut e2: Local<Option<Entity>>,
    mut commands: Commands,
    mut msg_writer: MessageWriter<OnClick>,
) {
    if !*not_first {
        let mut e = commands.spawn((
            tile::Marker,
            tile::Position,
            tile::Inactive,
            tile::Variant::A,
        ));
        *e1 = Some(e.id());
        *e2 = Some(e.clone_and_spawn().id());
        *not_first = true;
    }

    msg_writer.write(OnClick(e1.unwrap()));
    msg_writer.write(OnClick(e1.unwrap()));
}

fn system_a(
    mut msg_onclick: MessageReader<OnClick>,
    mut commands: Commands,
    // variants: Query<&tile::Variant>,
    mut tile_query: Query<
        (Entity, &mut tile::Position, &mut Sprite, &mut Transform),
        (Without<tile::Inactive>, With<tile::Marker>),
    >,
    mut prev_entity: ResMut<PreviouslySelectedTile>,
) {
    let Some(origin) = msg_onclick.read().next() else {
        return;
    };

    let Some(prev_tile) = **prev_entity else {
        prev_entity.as_mut().0 = Some(origin.0);
        return;
    };

    let mut e = tile_query.get_mut(**origin).unwrap();
    commands.entity(prev_tile).insert(tile::Inactive);
    commands.entity(**origin).insert(tile::Inactive);
}

fn system_b(
    mut commands: Commands,
    mut query_pair: Query<(&mut Transform, &mut Sprite), With<tile::Inactive>>,
) {
}
