use crate::{centered_text_bundle, remove_matching_elements, AssetManager, GameMode};
use bevy::{ecs::event::Events, prelude::*};

#[derive(Component)]
pub struct GameOverElement;

pub struct GameOverResource {
    pub message: String,
}

pub fn setup_game_over(
    mut commands: Commands,
    assets: Res<AssetManager>,
    game_over: Option<Res<GameOverResource>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameOverElement);

    if let Some(game_over) = game_over {
        commands
            .spawn_bundle(centered_text_bundle(
                &game_over.message,
                assets.default_font.clone(),
                30.0,
                Color::WHITE,
                Vec3::new(0.0, 0.0, 2.0),
            ))
            .insert(GameOverElement {});
    }

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.menu_atlas.clone(),
            sprite: TextureAtlasSprite::new(1),
            ..Default::default()
        })
        .insert(GameOverElement);
}

pub fn run_game_over(
    keyboard: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameMode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if keyboard.pressed(KeyCode::Return) {
        app_state
            .set(GameMode::MainMenu)
            .expect("Failed to change mode");
    }
    if keyboard.pressed(KeyCode::Q) {
        app_exit_events.send(bevy::app::AppExit);
    }
}

pub fn exit_game_over(elements: Query<(Entity, &GameOverElement)>, mut commands: Commands) {
    remove_matching_elements(elements, &mut commands);
}
