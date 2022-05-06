/// The menu framework is another item I reuse. It just displays graphics
/// from assets/menus.png to provide a loading, menu, and game over
/// screen foundation. If a game is good, I remove this and use
/// something tailored to the game.

mod game_over;
mod main_menu;
mod play_game;
use bevy::prelude::*;
pub use game_over::*;
pub use main_menu::*;
pub use play_game::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum GameMode {
    Loading,
    MainMenu,
    Playing,
    GameOver,
}

/// Creates a basic 1024x768 Bevy game structure, featuring a Loading
/// screen, a Main Menu, and a Game Over menu.
pub struct BasicGamePlugin;

impl Plugin for BasicGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameMode::Loading)
            // Main Menu Handler
            .add_system_set(SystemSet::on_enter(GameMode::MainMenu).with_system(setup_main_menu))
            .add_system_set(SystemSet::on_update(GameMode::MainMenu).with_system(run_main_menu))
            .add_system_set(SystemSet::on_exit(GameMode::MainMenu).with_system(exit_main_menu))
            // Play Game Handler
            .add_system_set(SystemSet::on_exit(GameMode::Playing).with_system(exit_play_game))
            // Game Over Handler
            .add_system_set(SystemSet::on_enter(GameMode::GameOver).with_system(setup_game_over))
            .add_system_set(SystemSet::on_update(GameMode::GameOver).with_system(run_game_over))
            .add_system_set(SystemSet::on_exit(GameMode::GameOver).with_system(exit_game_over));
    }
}
