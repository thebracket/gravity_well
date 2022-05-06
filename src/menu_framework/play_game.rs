use bevy::prelude::*;

use crate::remove_matching_elements;

#[derive(Component)]
pub struct PlayGameElement;

pub fn exit_play_game(elements: Query<(Entity, &PlayGameElement)>, mut commands: Commands) {
    remove_matching_elements(elements, &mut commands);
}
