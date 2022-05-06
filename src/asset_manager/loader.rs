use crate::{
    centered_text::centered_text_bundle, remove_matching_elements, AssetManager, BasicAssetPlugin,
    GameMode,
};
use bevy::prelude::*;

pub struct LoaderStatus {
    pub remaining_assets: Vec<HandleUntyped>,
}

#[derive(Component)]
pub struct LoaderElement;

pub fn setup_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_requests: Res<BasicAssetPlugin>,
) {
    // Load the required font
    let default_font = asset_server.load("FiraMono-Medium.ttf");

    // Initialize the loading list
    let mut status = LoaderStatus {
        remaining_assets: Vec::new(),
    };

    // Load the main menu and game over atlas
    let menu_texture_handle = asset_server.load("menus.png");
    status
        .remaining_assets
        .push(menu_texture_handle.clone_untyped());
    let menu_atlas = TextureAtlas::from_grid(menu_texture_handle, Vec2::new(1024.0, 768.0), 2, 1);
    let menu_atlas_handle = texture_atlases.add(menu_atlas);

    // Load the requested teture atlases
    let mut atlases = Vec::new();
    for atlas in asset_requests.atlases.iter() {
        let texture_handle = asset_server.load(&atlas.filename);
        status.remaining_assets.push(texture_handle.clone_untyped());
        let atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(atlas.sprite_width, atlas.sprite_height),
            atlas.columns,
            atlas.rows,
        );
        let atlas_handle = texture_atlases.add(atlas);
        atlases.push(atlas_handle);
    }

    // Create the basic loading UI
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(LoaderElement);

    commands
        .spawn_bundle(centered_text_bundle(
            "Loading, Please Wait...",
            default_font.clone(),
            30.0,
            Color::WHITE,
            Vec3::ZERO,
        ))
        .insert(LoaderElement);

    // Make the resource available
    commands.insert_resource(AssetManager {
        menu_atlas: menu_atlas_handle,
        default_font,
        atlases,
    });
    commands.insert_resource(status);
}

pub fn run_loading(
    mut app_state: ResMut<State<GameMode>>,
    mut loader: ResMut<LoaderStatus>,
    server: Res<AssetServer>,
) {
    // Remove any assets from the list that have loaded
    loader.remaining_assets.retain(|h| {
        let state = server.get_load_state(h.id);
        state != bevy::asset::LoadState::Loaded
    });

    if loader.remaining_assets.is_empty() {
        app_state
            .set(GameMode::MainMenu)
            .expect("Unable to change game mode.");
    }
}

pub fn exit_loading(mut commands: Commands, elements: Query<(Entity, &LoaderElement)>) {
    commands.remove_resource::<LoaderStatus>();
    commands.remove_resource::<BasicAssetPlugin>();
    remove_matching_elements(elements, &mut commands);
}
