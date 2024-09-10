use crate::{
    prelude::*,
    previous_transform::{PreviousPosition, PreviousRotation},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(insert_previous_position)
        .observe(insert_previous_rotation)
        .observe(remove_previous_transform)
        .observe(disable_interpolation)
        .observe(re_enable_interpolation);
}

fn insert_previous_position(
    trigger: Trigger<OnAdd, Position>,
    mut commands: Commands,
    q_position: Query<(&Position, Has<InterpolationMode>), Without<DisableTransformChanges>>,
) {
    let entity = trigger.entity();
    let Ok((position, has_interpolation_mode)) = q_position.get(entity) else {
        return;
    };
    commands
        .entity(entity)
        .insert((PreviousPosition::from(*position),));
    if !has_interpolation_mode {
        // We assume that having `Rotation` or `Collider` without `Position` would be malformed, so we insert this here.
        commands.entity(entity).insert(InterpolationMode::default());
    }
}

fn insert_previous_rotation(
    trigger: Trigger<OnAdd, Rotation>,
    mut commands: Commands,
    q_rotation: Query<&Rotation, Without<DisableTransformChanges>>,
) {
    let entity = trigger.entity();
    let Ok(rotation) = q_rotation.get(entity) else {
        return;
    };
    commands
        .entity(entity)
        .insert(PreviousRotation::from(*rotation));
}

fn disable_interpolation(trigger: Trigger<OnAdd, DisableTransformChanges>, mut commands: Commands) {
    let entity = trigger.entity();
    commands.entity(entity).add(remove_interpolation_components);
}

fn re_enable_interpolation(
    trigger: Trigger<OnRemove, DisableTransformChanges>,
    mut commands: Commands,
    q_physicsal_transform: Query<(&Position, &Rotation, Has<InterpolationMode>)>,
) {
    let entity = trigger.entity();
    let Ok((position, rotation, has_interpolation_mode)) = q_physicsal_transform.get(entity) else {
        return;
    };
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
    // We assume that having `Rotation` or `Collider` without `Position` would be malformed, so we only do this check for `Position`.
    commands.entity(entity).add(remove_interpolation_components);
}

fn remove_interpolation_components(entity: Entity, world: &mut World) {
    world
        .entity_mut(entity)
        .remove::<(PreviousPosition, PreviousRotation, InterpolationMode)>();
}
