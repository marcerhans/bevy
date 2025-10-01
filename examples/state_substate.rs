use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<State>()
        .add_plugins(sub::Plugin)
        .add_observer(name_removed)
        .add_systems(Startup, |mut next_state: ResMut<NextState<State>>| {
            info!("Init");
            next_state.set(State::A);
        })
        .add_systems(
            OnEnter(State::A),
            |mut commands: Commands, mut next_state: ResMut<NextState<State>>| {
                info!("Entered state: {:?}", State::A);
                info!("Switching to {:?}", State::B);
                info!(
                    "Spawned: {:?}",
                    commands.spawn((DespawnOnExit(State::A), Name::new("A"))).id()
                );
                next_state.set(State::B);
            },
        )
        .add_systems(
            OnEnter(State::B),
            |mut commands: Commands, mut next_state: ResMut<NextState<State>>| {
                info!("Entered state: {:?}", State::B);
                info!("Switching to {:?}", State::C);
                info!(
                    "Spawned: {:?}",
                    commands.spawn((DespawnOnExit(State::B), Name::new("B"))).id()
                );
                next_state.set(State::C);
            },
        )
        .add_systems(OnEnter(State::C), |mut commands: Commands| {
            info!("Entered state: {:?}", State::C);
            info!(
                "Spawned: {:?}",
                commands.spawn((DespawnOnExit(State::C), Name::new("C"))).id()
            );
        })
        .run();
}

pub fn name_removed(
    trigger: Trigger<OnRemove, Name>,
    query: Query<&Name>,
) {
    let name = query.get(trigger.target()).unwrap();
    info!("Removed: {:?} ({:?})", name, trigger.target());
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
#[states(scoped_entities)]
pub enum State {
    #[default]
    Init,
    A,
    B,
    C,
}

mod sub {
    use super::State;
    use bevy::prelude::*;

    #[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
    #[source(State = State::C)]
    #[states(scoped_entities)]
    enum SubState {
        #[default]
        AA,
        BB,
        CC,
    }

    pub struct Plugin;

    impl bevy::prelude::Plugin for Plugin {
        fn build(
            &self,
            app: &mut App,
        ) {
            app.add_sub_state::<SubState>()
                .add_systems(
                    OnEnter(SubState::AA),
                    |mut commands: Commands, mut next_state: ResMut<NextState<SubState>>| {
                        info!("Entered state: {:?}", SubState::AA);
                        info!("Switching to {:?}", SubState::BB);
                        info!(
                            "Spawned: {:?}",
                            commands
                                .spawn((DespawnOnExit(SubState::AA), Name::new("AA")))
                                .id()
                        );
                        next_state.set(SubState::BB);
                    },
                )
                .add_systems(
                    OnEnter(SubState::BB),
                    |mut commands: Commands, mut next_state: ResMut<NextState<SubState>>| {
                        info!("Entered state: {:?}", SubState::BB);
                        info!("Switching to {:?}", SubState::CC);
                        info!(
                            "Spawned: {:?}",
                            commands
                                .spawn((DespawnOnExit(SubState::BB), Name::new("BB")))
                                .id()
                        );
                        next_state.set(SubState::CC);
                    },
                )
                .add_systems(
                    OnEnter(SubState::CC),
                    |mut commands: Commands, mut next_state: ResMut<NextState<State>>| {
                        info!("Entered state: {:?}", SubState::CC);
                        info!("Switching to super State {:?}", State::Init);
                        info!(
                            "Spawned: {:?}",
                            commands
                                .spawn((DespawnOnExit(SubState::CC), Name::new("CC")))
                                .id()
                        );
                        next_state.set(State::Init);
                    },
                );
        }
    }
}
