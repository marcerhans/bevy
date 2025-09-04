use bevy::prelude::*;

#[derive(Component)]
#[require(Name)]
struct Simple;

#[derive(Event)]
struct Despawned(Name);

impl Simple {
    fn on_despawn(trigger: Trigger<Despawned>) {
        info!("Despawned: {:?}", trigger.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<State>()
        .add_plugins(sub::Plugin)
        .add_systems(
            OnEnter(State::A),
            |mut commands: Commands, mut next_state: ResMut<NextState<State>>| {
                info!("Observing despawn events");
                commands.spawn(Observer::new(Simple::on_despawn));

                info!("Entered state: {:?}", State::A);
                info!("Switching to {:?}", State::B);
                let name = Name::new("A Component");
                commands.spawn((StateScoped(State::A), Simple, name.clone()));
                commands.trigger(Despawned(name));
                next_state.set(State::B);
            },
        )
        .add_systems(
            OnEnter(State::B),
            |mut commands: Commands, mut next_state: ResMut<NextState<State>>| {
                info!("Entered state: {:?}", State::B);
                info!("Switching to {:?}", State::C);
                let name = Name::new("B Component");
                commands.spawn((StateScoped(State::B), Simple, name.clone()));
                commands.trigger(Despawned(name));
                next_state.set(State::C);
            },
        )
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
#[states(scoped_entities)]
pub enum State {
    #[default]
    A,
    B,
    C,
}

mod sub {
    use bevy::prelude::*;
    use super::{State, Simple, Despawned};

    #[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
    #[source(State = State::B)]
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
            app.add_systems(
                OnEnter(SubState::AA),
                |mut commands: Commands, mut next_state: ResMut<NextState<SubState>>| {
                    info!("Entered state: {:?}", SubState::AA);
                    info!("Switching to {:?}", SubState::BB);
                    let name = Name::new("AA Component");
                    commands.spawn((StateScoped(State::A), Simple, name.clone()));
                    commands.trigger(Despawned(name));
                    next_state.set(SubState::BB);
                },
            );
        }
    }
}
