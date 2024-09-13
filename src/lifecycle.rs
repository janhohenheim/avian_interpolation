use crate::{
    prelude::*,
    previous_transform::{PreviousPosition, PreviousRotation},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(insert_previous_position)
        .observe(remove_previous_transform);
}

fn insert_previous_position(
    trigger: Trigger<OnAdd, Position>,
    mut commands: Commands,
    q_position: Query<(&Position, &Rotation, &RigidBody)>,
) {
    let entity = trigger.entity();
    let Ok((position, rotation, rigid_body)) = q_position.get(entity) else {
        // This is a collider, not a rigid body.
        return;
    };
    if rigid_body.is_static() {
        return;
    }
    commands.entity(entity).insert((
        PreviousPosition::from(*position),
        PreviousRotation::from(*rotation),
    ));
}

fn remove_previous_transform(trigger: Trigger<OnRemove, Position>, mut commands: Commands) {
    let entity = trigger.entity();
    // We assume that having `Rotation` without `Position` would be malformed, so we only do this check for `Position`.
    commands
        .entity(entity)
        .remove::<(PreviousPosition, PreviousRotation)>();
}
