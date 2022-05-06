use crate::{remove_matching_elements, AssetManager, GameMode};
use bevy::{ecs::event::Events, prelude::*};

#[derive(Component)]
pub struct MainMenuElement;

pub fn setup_main_menu(mut commands: Commands, assets: Res<AssetManager>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainMenuElement);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.menu_atlas.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert(MainMenuElement);
}

pub fn run_main_menu(
    keyboard: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameMode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if keyboard.pressed(KeyCode::P) {
        app_state
            .set(GameMode::Playing)
            .expect("Failed to change mode");
    }
    if keyboard.pressed(KeyCode::Q) {
        app_exit_events.send(bevy::app::AppExit);
    }
}

pub fn exit_main_menu(elements: Query<(Entity, &MainMenuElement)>, mut commands: Commands) {
    remove_matching_elements(elements, &mut commands);
}
