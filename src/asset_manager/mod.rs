/// The asset manager is a reusable helper I sometimes use to make
/// Bevy games quickly. It lets me specify my assets up-front,
/// provides a Res<Assets> that gives me numbered access to asset
/// handles (rather than passing around tons of handle-storing resources)
/// and integrates with the loader to avoid asset popping.

mod asset_management;
pub use asset_management::*;
mod loader;
use crate::GameMode;
use bevy::prelude::*;
pub use loader::*;

#[derive(Clone)]
struct AtlasInfo {
    filename: String,
    sprite_width: f32,
    sprite_height: f32,
    columns: usize,
    rows: usize,
}

#[derive(Clone)]
pub struct BasicAssetPlugin {
    atlases: Vec<AtlasInfo>,
}

impl Plugin for BasicAssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.clone())
            // Loading System Handler
            .add_system_set(SystemSet::on_enter(GameMode::Loading).with_system(setup_loading))
            .add_system_set(SystemSet::on_update(GameMode::Loading).with_system(run_loading))
            .add_system_set(SystemSet::on_exit(GameMode::Loading).with_system(exit_loading));
    }
}

impl BasicAssetPlugin {
    pub fn new() -> Self {
        Self {
            atlases: Vec::new(),
        }
    }

    pub fn with_atlas<S: ToString>(
        mut self,
        filename: S,
        sprite_width: f32,
        sprite_height: f32,
        columns: usize,
        rows: usize,
    ) -> Self {
        self.atlases.push(AtlasInfo {
            filename: filename.to_string(),
            sprite_width,
            sprite_height,
            columns,
            rows,
        });
        self
    }
}
