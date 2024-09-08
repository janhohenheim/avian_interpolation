use bevy::app::RunFixedMainLoop;

use crate::prelude::*;
use crate::previous_transform::{PreviousPosition, PreviousRotation, PreviousScale};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        RunFixedMainLoop,
        interpolate_transform.in_set(VariableAvianInterpolationSystem::Interpolate),
    );
}

fn interpolate_transform(
    fixed_time: Res<Time<Fixed>>,
    mut q_interpolant: Query<
        (
            &mut Transform,
            &Position,
            &Rotation,
            Option<&Collider>,
            &PreviousPosition,
            &PreviousRotation,
            Option<&PreviousScale>,
        ),
        (Without<NonInterpolated>,),
    >,
) {
    // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
    let alpha = fixed_time.overstep_fraction();

    for (
        mut transform,
        position,
        rotation,
        collider,
        previous_position,
        previous_rotation,
        previous_scale,
    ) in &mut q_interpolant
    {}
}
