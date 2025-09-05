pub mod state {
    trait Base: Eq + PartialEq + Clone + std::fmt::Debug + Default {}
    pub trait States: bevy::prelude::States + Base {}
    pub trait SubStates: bevy::prelude::SubStates + Base {}
}
