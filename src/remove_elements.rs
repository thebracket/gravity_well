use bevy::prelude::*;

/// Remove all entities with a given tag component.
/// Useful for cleaning up scenes on mode-transition.
pub fn remove_matching_elements<T: Component>(
    element_query: Query<(Entity, &T)>,
    commands: &mut Commands,
) {
    element_query.iter().for_each(|(entity, _)| {
        commands.entity(entity).despawn();
    });
}
