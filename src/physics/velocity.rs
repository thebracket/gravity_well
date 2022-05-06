use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let portion_of_frame = time.delta().as_millis() as f32 / 33.0;
    query.iter_mut().for_each(|(mut trans, velocity)| {
        let delta = velocity.0 * portion_of_frame;
        trans.translation.x += delta.x;
        trans.translation.y += delta.y;
        trans.translation.z += delta.z;
        // For some reason, Bevy 0.7 is giving an error when I add two Vec3s together.
    });
}

#[derive(Component)]
pub struct Attractor {
    pub max_velocity: f32,
}

pub fn velocity_attractor_2d(
    mut velocities: Query<(&Transform, &mut Velocity)>,
    attractors: Query<&Transform, With<Attractor>>,
    time: Res<Time>,
) {
    let portion_of_frame = time.delta().as_millis() as f32 / 33.0;
    for attractor in attractors.iter() {
        for (target, mut velocity) in velocities.iter_mut() {
            let distance = attractor.translation.distance_squared(target.translation);
            if distance > 0.0 {
                let direction = attractor.translation - target.translation;
                let normalized = direction.normalize();
                let scaled = (normalized / distance) * 2000.0 * portion_of_frame;
                velocity.0.x += scaled.x;
                velocity.0.y += scaled.y;
            }
        }
    }
}
