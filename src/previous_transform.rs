use crate::prelude::*;
use avian::math::{Quaternion, Vector};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedPreUpdate,
        cache_previous_transform
            .in_set(AvianInterpolationFixedSystem::CachePreviousPhysicsTransform),
    );
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Component, Deref, DerefMut)]
pub(crate) struct PreviousPosition(pub Vector);

impl From<Position> for PreviousPosition {
    fn from(value: Position) -> Self {
        PreviousPosition(value.0)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Component, Deref, DerefMut)]
pub(crate) struct PreviousRotation(pub Quaternion);

impl From<Rotation> for PreviousRotation {
    fn from(value: Rotation) -> Self {
        PreviousRotation(value.into())
    }
}

fn cache_previous_transform(
    mut q_physics: Query<
        (
            Ref<Position>,
            Ref<Rotation>,
            &mut PreviousPosition,
            &mut PreviousRotation,
        ),
        Or<(Changed<Position>, Changed<Rotation>)>,
    >,
) {
    for (position, rotation, mut previous_position, mut previous_rotation) in &mut q_physics {
        if position.is_changed() {
            *previous_position = (*position).into();
        }
        if rotation.is_changed() {
            *previous_rotation = (*rotation).into();
        }
    }
}
