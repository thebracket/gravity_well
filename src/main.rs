use bevy::prelude::*;
use std::time::Duration;
mod components;
use components::{EmitTrail, PlayGameElement, Player, Salvage};
mod asset_manager;
pub use asset_manager::{AssetManager, BasicAssetPlugin};
mod menu_framework;
pub use menu_framework::{BasicGamePlugin, GameMode, GameOverResource};
mod particles;
pub use particles::{ParticleColorLerp, ParticleLifetime, ParticlePlugin};
mod physics;
pub use physics::{
    apply_velocity, find_one_collision, velocity_attractor_2d, Attractor, BoundingBox2D, Velocity,
};
mod random;
pub use random::*;
pub mod centered_text;
pub use centered_text::*;
mod remove_elements;
pub use remove_elements::remove_matching_elements;

pub struct SalvageTimer(Timer);

pub struct ParticleTimer(pub Timer);

pub struct Scores(Vec<u32>);

fn main() {
    let assets = BasicAssetPlugin::new().with_atlas("spritesheet.png", 24.0, 24.0, 5, 1);

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Gravity Well".to_string(),
            width: 1024.0,
            height: 768.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BasicGamePlugin)
        .add_plugin(assets)
        .add_plugin(ParticlePlugin)
        .insert_resource(RandomNumbers::new())
        .insert_resource(ClearColor(Color::BLACK))
        .add_system_set(SystemSet::on_enter(GameMode::Playing).with_system(setup_play_game))
        .add_system_set(
            SystemSet::on_update(GameMode::Playing)
                .with_system(player_control)
                .with_system(apply_velocity)
                .with_system(velocity_attractor_2d)
                .with_system(trails)
                .with_system(bounce)
                .with_system(black_hole)
                .with_system(spawn_salvage)
                .with_system(clamp_positions)
                .with_system(collect_salvage)
                .with_system(end_game),
        )
        .add_system_set(SystemSet::on_exit(GameMode::Playing).with_system(exit_play_game))
        .run();
}

/// Runs when "play game" becomes the active mode. Spawns all initial
/// entities required for game play.
fn setup_play_game(mut commands: Commands, assets: Res<AssetManager>) {
    // Spawn a 2D camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayGameElement);

    // Spawn player 0
    commands
        .spawn_bundle(bevy::prelude::SpriteSheetBundle {
            texture_atlas: assets.atlases[0].clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(300.0, 1.0, 0.0),
            ..Default::default()
        })
        .insert(PlayGameElement)
        .insert(Velocity(Vec3::new(0.0, 2.0, 0.0)))
        .insert(Player { id: 0 })
        .insert(EmitTrail)
        .insert(BoundingBox2D::new(24.0, 24.0));

    // Spawn player 1
    commands
        .spawn_bundle(bevy::prelude::SpriteSheetBundle {
            texture_atlas: assets.atlases[0].clone(),
            sprite: TextureAtlasSprite::new(1),
            transform: Transform::from_xyz(-300.0, 1.0, 0.0),
            ..Default::default()
        })
        .insert(PlayGameElement)
        .insert(Velocity(Vec3::new(0.0, -2.0, 0.0)))
        .insert(Player { id: 1 })
        .insert(EmitTrail)
        .insert(BoundingBox2D::new(24.0, 24.0));

    // Spawn the black hole in the middle
    commands
        .spawn_bundle(bevy::prelude::SpriteSheetBundle {
            texture_atlas: assets.atlases[0].clone(),
            sprite: TextureAtlasSprite::new(3),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Attractor { max_velocity: 3.0 })
        .insert(PlayGameElement)
        .insert(BoundingBox2D::new(24.0, 24.0));

    // Add the two timers the game uses
    commands.insert_resource(ParticleTimer(Timer::new(Duration::from_millis(10), true)));
    commands.insert_resource(SalvageTimer(Timer::new(Duration::from_millis(2000), true)));

    // Store the scores
    commands.insert_resource(Scores(vec![0, 0]));
}

/// Handle player keyboard input
fn player_control(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut Transform, &Player)>,
) {
    for (mut velocity, mut trans, player) in player_query.iter_mut() {
        let (left, right, thrust) = match player.id {
            0 => (KeyCode::Left, KeyCode::Right, KeyCode::Up),
            _ => (KeyCode::A, KeyCode::D, KeyCode::W),
        };

        if keyboard.pressed(left) {
            trans.rotate(Quat::from_rotation_z(f32::to_radians(2.0)));
        }
        if keyboard.pressed(right) {
            trans.rotate(Quat::from_rotation_z(f32::to_radians(-2.0)));
        }

        if keyboard.pressed(thrust) {
            velocity.0 += trans.local_y() / 10.0;
            velocity.0 = velocity.0.clamp_length_max(5.0);
        }
    }
}

/// Anything with an `EmitTrail` component spawns particles periodically.
fn trails(
    mut commands: Commands,
    query: Query<(&Transform, Option<&Player>), With<EmitTrail>>,
    time: Res<Time>,
    mut timer: ResMut<ParticleTimer>,
    assets: Res<AssetManager>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        for (pos, player) in query.iter() {
            let (start, end) = if let Some(player) = player {
                match player.id {
                    0 => (Color::YELLOW, Color::BLACK),
                    _ => (Color::PURPLE, Color::BLACK),
                }
            } else {
                (Color::WHITE, Color::BLACK)
            };

            commands
                .spawn_bundle(bevy::prelude::SpriteSheetBundle {
                    texture_atlas: assets.atlases[0].clone(),
                    sprite: TextureAtlasSprite::new(2),
                    transform: Transform::from_xyz(
                        pos.translation.x,
                        pos.translation.y,
                        pos.translation.z,
                    ),
                    ..Default::default()
                })
                .insert(PlayGameElement)
                .insert(ParticleLifetime::new(2000.0))
                .insert(ParticleColorLerp { start, end })
                .insert(Velocity(Vec3::ZERO));
        }
    }
}

/// Did the players hit one another? If so, we'll make them bounce away and spawn a particle
/// burst.
fn bounce(
    mut query: Query<(
        &mut BoundingBox2D,
        &mut Transform,
        &mut Velocity,
        &mut Player,
    )>,
    mut commands: Commands,
    assets: Res<AssetManager>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(box_a, pos_a, mut velocity_a, _p1), (box_b, pos_b, mut velocity_b, _p2)]) =
        combinations.fetch_next()
    {
        // mutably access components data
        if box_a.collides_with(&*pos_a, &box_b, &pos_b) {
            let bounce = (pos_a.translation - pos_b.translation).normalize();
            velocity_a.0 += bounce;
            velocity_b.0 -= bounce;
            spawn_particle_burst(
                &assets,
                &mut commands,
                pos_a.translation,
                Color::CYAN,
                Color::BLUE,
                1000.0,
            );
        }
    }
}

/// Did anything fall into the black hole? That ends the existence of an entity.
fn black_hole(
    mut commands: Commands,
    hole_query: Query<(&Transform, &BoundingBox2D), With<Attractor>>,
    other_query: Query<(Entity, &Transform, &BoundingBox2D), Without<Attractor>>,
) {
    let objects: Vec<(Entity, &Transform, &BoundingBox2D)> = other_query.iter().collect();
    for (hole_pos, hole_box) in hole_query.iter() {
        if let Some(destroyed) = find_one_collision(hole_pos, hole_box, &objects) {
            commands.entity(destroyed).despawn();
        }
    }
}

/// Periodically add salvage to the game
fn spawn_salvage(
    mut commands: Commands,
    assets: Res<AssetManager>,
    time: Res<Time>,
    mut timer: ResMut<SalvageTimer>,
    rng: ResMut<RandomNumbers>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let position = Vec3::new(
            rng.range(0, 1024) as f32 - 512.0,
            rng.range(0, 768) as f32 - 384.0,
            1.0,
        );
        let velocity = Vec3::new(
            (rng.range(0, 20) as f32 - 10.0) / 5.0,
            (rng.range(0, 20) as f32 - 10.0) / 5.0,
            0.0,
        );
        commands
            .spawn_bundle(bevy::prelude::SpriteSheetBundle {
                texture_atlas: assets.atlases[0].clone(),
                sprite: TextureAtlasSprite::new(4),
                transform: Transform::from_xyz(position.x, position.y, position.z),
                ..Default::default()
            })
            .insert(PlayGameElement)
            .insert(BoundingBox2D::new(24.0, 24.0))
            .insert(EmitTrail)
            .insert(Velocity(velocity))
            .insert(Salvage);
        spawn_particle_burst(
            &assets,
            &mut commands,
            position,
            Color::PINK,
            Color::BLACK,
            1000.0,
        );
    }
}

/// Ensure that nothing flies completely off the screen
fn clamp_positions(mut query: Query<&mut Transform, Or<(Changed<Transform>, Added<Transform>)>>) {
    let left = (0.0 - 1024.0) / 2.0;
    let right = 1024.0 / 2.0;
    let top = (0.0 - 768.0) / 2.0;
    let bottom = 768.0 / 2.0;
    for mut pos in query.iter_mut() {
        if pos.translation.x < left {
            pos.translation.x = left;
        }
        if pos.translation.x > right {
            pos.translation.x = right;
        }
        if pos.translation.y < top {
            pos.translation.y = top;
        }
        if pos.translation.y > bottom {
            pos.translation.y = bottom;
        }
    }
}

/// Spawn a circle of particles at a given point.
fn spawn_particle_burst(
    assets: &AssetManager,
    commands: &mut Commands,
    pos: Vec3,
    start: Color,
    end: Color,
    lifetime: f32,
) {
    for angle in 0..360 {
        let velocity =
            Quat::from_rotation_z((angle as f32).to_radians()) * Vec3::new(1.0, 0.0, 0.0);
        commands
            .spawn_bundle(bevy::prelude::SpriteSheetBundle {
                texture_atlas: assets.atlases[0].clone(),
                sprite: TextureAtlasSprite::new(2),
                transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                ..Default::default()
            })
            .insert(PlayGameElement)
            .insert(ParticleLifetime::new(lifetime))
            .insert(ParticleColorLerp { start, end })
            .insert(Velocity(velocity));
    }
}

/// Did either player hit some salvage? If so, give them some score, despawn the salvage and add a particle burst.
fn collect_salvage(
    mut commands: Commands,
    players: Query<(&Player, &Transform, &BoundingBox2D)>,
    salvage: Query<(Entity, &Transform, &BoundingBox2D), With<Salvage>>,
    mut scores: ResMut<Scores>,
    assets: Res<AssetManager>,
) {
    let salvage: Vec<(Entity, &Transform, &BoundingBox2D)> = salvage.iter().collect();
    for (player, pos, bounds) in players.iter() {
        if let Some(salvage) = find_one_collision(pos, bounds, &salvage) {
            commands.entity(salvage).despawn();
            scores.0[player.id] += 1;
            spawn_particle_burst(
                &assets,
                &mut commands,
                pos.translation,
                Color::GREEN,
                Color::YELLOW,
                2000.0,
            );
        }
    }
}

/// Did a player die? If so, end the game.
fn end_game(
    query: Query<&Player>,
    mut app_state: ResMut<State<GameMode>>,
    mut commands: Commands,
    scores: Res<Scores>,
) {
    let remaining_players = query.iter().count();
    if remaining_players < 2 {
        let mut message = String::new();
        for (id, score) in scores.0.iter().enumerate() {
            message += &format!("Player {} scored {} points.\n", id + 1, score);
        }
        commands.insert_resource(GameOverResource { message });
        app_state.set(GameMode::GameOver).unwrap();
    }
}

/// Clean up by removing all gameplay elements on game over.
fn exit_play_game(mut commands: Commands, elements: Query<(Entity, &PlayGameElement)>) {
    remove_matching_elements(elements, &mut commands);
}
