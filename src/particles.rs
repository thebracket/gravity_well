use crate::GameMode;
use bevy::prelude::*;

#[derive(Component)]
pub struct ParticleLifetime {
    pub elapsed: f32,
    pub max: f32,
}

impl ParticleLifetime {
    pub fn new(max: f32) -> Self {
        Self { elapsed: 0.0, max }
    }
}

fn particle_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ParticleLifetime)>,
) {
    let delta = time.delta().as_millis() as f32;
    query.iter_mut().for_each(|(entity, mut lifetime)| {
        lifetime.elapsed += delta;
        if lifetime.elapsed > lifetime.max {
            commands.entity(entity).despawn();
        }
    });
}

#[derive(Component)]
pub struct ParticleColorLerp {
    pub start: Color,
    pub end: Color,
}

fn particle_color_lerp(
    mut query: Query<(
        &ParticleLifetime,
        &ParticleColorLerp,
        &mut TextureAtlasSprite,
    )>,
) {
    query.iter_mut().for_each(|(life, lerp, mut sprite)| {
        let life_complete = life.elapsed / life.max;
        let start: Vec4 = lerp.start.into();
        let end: Vec4 = lerp.end.into();
        let color_vec = start.lerp(end, life_complete);
        sprite.color = color_vec.into();
    });
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameMode::Playing)
                .with_system(particle_lifetimes)
                .with_system(particle_color_lerp),
        );
    }
}
