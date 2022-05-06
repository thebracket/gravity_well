use bevy::prelude::*;

#[derive(Component)]
pub struct BoundingBox2D {
    pub width: f32,
    pub height: f32,
}

impl BoundingBox2D {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn collides_with(
        &self,
        pos: &Transform,
        other: &BoundingBox2D,
        other_pos: &Transform,
    ) -> bool {
        let pos = pos.translation;
        let other_pos = other_pos.translation;
        pos.x + self.width > other_pos.x
            && pos.x < other_pos.x + other.width
            && pos.y + self.height > other_pos.y
            && pos.y < other_pos.y + other.height
    }
}

pub fn find_one_collision(
    single_position: &Transform,
    single_bounds: &BoundingBox2D,
    potential_targets: &[(Entity, &Transform, &BoundingBox2D)],
) -> Option<Entity> {
    for (entity, position, bounds) in potential_targets.iter() {
        if single_bounds.collides_with(single_position, bounds, position) {
            return Some(*entity);
        }
    }
    None
}
