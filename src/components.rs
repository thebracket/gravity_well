use bevy::prelude::*;

#[derive(Component)]
pub struct PlayGameElement;

#[derive(Component)]
pub struct Player {
    pub id: usize,
}

#[derive(Component)]
pub struct EmitTrail;

#[derive(Component)]
pub struct Salvage;
