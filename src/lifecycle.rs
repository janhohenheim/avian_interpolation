use crate::{
    prelude::*,
    previous_transform::{PreviousPosition, PreviousRotation},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(insert_previous_position)
        .observe(remove_previous_transform)
        .observe(disable_interpolation)
        .observe(re_enable_interpolation);
}

fn insert_previous_position(
    trigger: Trigger<OnAdd, Position>,
    mut commands: Commands,
    q_position: Query<
        (&Position, &Rotation, &RigidBody, Has<InterpolationMode>),
        Without<DisableTransformChanges>,
    >,
) {
    let entity = trigger.entity();
    let Ok((position, rotation, rigid_body, has_interpolation_mode)) = q_position.get(entity)
    else {
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

    if !has_interpolation_mode {
        commands.entity(entity).insert(InterpolationMode::default());
    }
}

fn disable_interpolation(trigger: Trigger<OnAdd, DisableTransformChanges>, mut commands: Commands) {
    let entity = trigger.entity();
    commands.entity(entity).add(remove_interpolation_components);
}

fn re_enable_interpolation(
    trigger: Trigger<OnRemove, DisableTransformChanges>,
    mut commands: Commands,
    q_physicsal_transform: Query<(&Position, &Rotation, &RigidBody, Has<InterpolationMode>)>,
) {
    let entity = trigger.entity();
    let Ok((position, rotation, rigid_body, has_interpolation_mode)) =
        q_physicsal_transform.get(entity)
    else {
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
    if !has_interpolation_mode {
        commands.entity(entity).insert(InterpolationMode::default());
    }
}

fn remove_previous_transform(trigger: Trigger<OnRemove, Position>, mut commands: Commands) {
    let entity = trigger.entity();
    // We assume that having `Rotation` without `Position` would be malformed, so we only do this check for `Position`.
    commands.entity(entity).add(remove_interpolation_components);
}

fn remove_interpolation_components(entity: Entity, world: &mut World) {
    // We need to check if the entity still exists, as this may have been called on a despawning entity.
    if let Some(mut entity_mut) = world.get_entity_mut(entity) {
        entity_mut.remove::<(PreviousPosition, PreviousRotation, InterpolationMode)>();
    }
}
