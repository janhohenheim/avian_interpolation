use crate::{
    prelude::*,
    previous_transform::{PreviousPosition, PreviousRotation, PreviousScale},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(insert_previous_position)
        .observe(insert_previous_rotation)
        .observe(insert_previous_scale)
        .observe(remove_previous_position)
        .observe(remove_previous_rotation)
        .observe(remove_previous_scale);
}

fn insert_previous_position(
    trigger: Trigger<OnAdd, Position>,
    mut commands: Commands,
    q_position: Query<&Position>,
) {
    let entity = trigger.entity();
    let Ok(position) = q_position.get(entity) else {
        return;
    };
    commands
        .entity(entity)
        .insert(PreviousPosition::from(*position));
}

fn insert_previous_rotation(
    trigger: Trigger<OnAdd, Rotation>,
    mut commands: Commands,
    q_rotation: Query<&Rotation>,
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
    q_collider: Query<&Collider>,
) {
    let entity = trigger.entity();
    let Ok(collider) = q_collider.get(entity) else {
        return;
    };
    commands
        .entity(entity)
        .insert(PreviousScale::from(collider));
}

fn remove_previous_position(trigger: Trigger<OnRemove, Position>, mut commands: Commands) {
    let entity = trigger.entity();
    commands.entity(entity).remove::<PreviousPosition>();
}

fn remove_previous_rotation(trigger: Trigger<OnRemove, Rotation>, mut commands: Commands) {
    let entity = trigger.entity();
    commands.entity(entity).remove::<PreviousRotation>();
}

fn remove_previous_scale(trigger: Trigger<OnRemove, Collider>, mut commands: Commands) {
    let entity = trigger.entity();
    commands.entity(entity).remove::<PreviousScale>();
}
