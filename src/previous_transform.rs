use std::borrow::Borrow;

use crate::prelude::*;
use avian::math::Vector;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PhysicsSchedule,
        cache_previous_transform
            .in_set(FixedAvianInterpolationSystem::CachePreviousPhysicsTransform),
    );
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Component, Deref, DerefMut)]
pub(crate) struct PreviousPosition(pub Position);

impl From<Position> for PreviousPosition {
    fn from(value: Position) -> Self {
        PreviousPosition(value)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Component, Deref, DerefMut)]
pub(crate) struct PreviousRotation(pub Rotation);

impl From<Rotation> for PreviousRotation {
    fn from(value: Rotation) -> Self {
        PreviousRotation(value)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Component, Deref, DerefMut)]
pub(crate) struct PreviousScale(pub Vector);

impl<T: Borrow<Collider>> From<T> for PreviousScale {
    fn from(value: T) -> Self {
        PreviousScale(value.borrow().scale())
    }
}

fn cache_previous_transform(
    mut q_physics: Query<
        (
            Ref<Position>,
            Ref<Rotation>,
            Option<Ref<Collider>>,
            &mut PreviousPosition,
            &mut PreviousRotation,
            Option<&mut PreviousScale>,
        ),
        (
            Without<NonInterpolated>,
            Or<(Changed<Position>, Changed<Rotation>, Changed<Collider>)>,
        ),
    >,
) {
    for (
        position,
        rotation,
        collider,
        mut previous_position,
        mut previous_rotation,
        mut previous_scale,
    ) in &mut q_physics
    {
        if position.is_changed() {
            *previous_position = position.clone().into();
        }
        if rotation.is_changed() {
            *previous_rotation = rotation.clone().into();
        }
        if let Some(mut previous_scale) = previous_scale {
            if let Some(collider) = collider {
                if collider.is_changed() {
                    *previous_scale = collider.as_ref().into();
                }
            }
        }
    }
}
