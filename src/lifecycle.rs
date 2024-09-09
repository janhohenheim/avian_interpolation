use crate::{
    prelude::*,
    previous_transform::{PreviousPosition, PreviousRotation, PreviousScale},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(insert_previous_position)
        .observe(insert_previous_rotation)
        .observe(insert_previous_scale)
        .observe(remove_previous_transform)
        .observe(remove_previous_scale)
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
    commands.entity(entity).insert((
        PreviousPosition::from(*position),
    ));
    if !has_interpolation_mode {
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

fn insert_previous_scale(
    trigger: Trigger<OnAdd, Collider>,
    mut commands: Commands,
    q_collider: Query<&Collider, Without<DisableTransformChanges>>,
) {
    let entity = trigger.entity();
    let Ok(collider) = q_collider.get(entity) else {
        return;
    };
    commands
        .entity(entity)
        .insert(PreviousScale::from(collider));
}

fn disable_interpolation(trigger: Trigger<OnAdd, DisableTransformChanges>, mut commands: Commands) {
    let entity = trigger.entity();
    commands.entity(entity).add(remove_interpolation_components);
}

fn re_enable_interpolation(
    trigger: Trigger<OnRemove, DisableTransformChanges>,
    mut commands: Commands,
    q_physicsal_transform: Query<(
        &Position,
        &Rotation,
        Option<&Collider>,
        Has<InterpolationMode>,
    )>,
) {
    let entity = trigger.entity();
    let Ok((position, rotation, maybe_collider, has_interpolation_mode)) =
        q_physicsal_transform.get(entity)
    else {
        return;
    };
    commands.entity(entity).insert((
        PreviousPosition::from(*position),
        PreviousRotation::from(*rotation),
    ));
    if !has_interpolation_mode {
        commands.entity(entity).insert(InterpolationMode::default());
    }
    if let Some(collider) = maybe_collider {
        commands
            .entity(entity)
            .insert(PreviousScale::from(collider));
    }
}

fn remove_previous_transform(trigger: Trigger<OnRemove, Position>, mut commands: Commands) {
    let entity = trigger.entity();
    // We assume that having `Rotation` or `Collider` without `Position` would be malformed, so we only do this check for `Position`.
    commands.entity(entity).add(remove_interpolation_components);
}

fn remove_previous_scale(trigger: Trigger<OnRemove, Collider>, mut commands: Commands) {
    let entity = trigger.entity();
    // This is handled because having a `Position` and `Rotation` but removing the `Collider` at runtime is valid.
    commands.entity(entity).remove::<PreviousScale>();
}

fn remove_interpolation_components(entity: Entity, world: &mut World) {
    world.entity_mut(entity).remove::<(
        PreviousPosition,
        PreviousRotation,
        PreviousScale,
        InterpolationMode,
    )>();
}
