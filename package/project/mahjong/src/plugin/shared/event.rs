use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
    }
}

#[derive(Event)]
pub struct Clicked(pub Entity);
