use bevy::app::RunFixedMainLoop;

use crate::prelude::*;
use crate::previous_transform::{PreviousPosition, PreviousRotation};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        RunFixedMainLoop,
        interpolate_rigid_bodies.in_set(AvianInterpolationVariableSystem::Interpolate),
    );
}

fn interpolate_rigid_bodies(
    fixed_time: Res<Time<Fixed>>,
    mut q_interpolant: Query<(
        &mut Transform,
        Option<&Parent>,
        &Position,
        &Rotation,
        &PreviousPosition,
        &PreviousRotation,
        Option<&InterpolateTransformFields>,
    )>,
    q_global_transform: Query<&GlobalTransform>,
) {
    // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
    let alpha = fixed_time.overstep_fraction();

    for (
        mut transform,
        maybe_parent,
        position,
        rotation,
        previous_position,
        previous_rotation,
        maybe_interpolate_transform_fields,
    ) in &mut q_interpolant
    {
        let interpolate_transform_fields = maybe_interpolate_transform_fields
            .copied()
            .unwrap_or_default();
        let translation = match interpolate_transform_fields.translation {
            InterpolationMode::Linear => Some(previous_position.lerp(position.0, alpha)),
            InterpolationMode::Last => Some(position.0),
            InterpolationMode::None => None,
        };
        #[cfg(feature = "2d")]
        let translation = translation.map(|translation| translation.extend(0.));

        let rotation = {
            #[cfg(feature = "2d")]
            {
                Quat::from(*rotation)
            }
            #[cfg(feature = "3d")]
            {
                rotation.0
            }
        };

        let rotation = match interpolate_transform_fields.rotation {
            InterpolationMode::Linear => Some(previous_rotation.slerp(rotation, alpha)),
            InterpolationMode::Last => Some(rotation),
            InterpolationMode::None => None,
        };

        let maybe_parent_transform = maybe_parent
            .and_then(|parent| q_global_transform.get(parent.get()).ok())
            .map(|global_transform| global_transform.compute_transform());

        let new_translation = translation.map(|translation| {
            maybe_parent_transform
                .map(|parent_transform| translation - parent_transform.translation)
                .unwrap_or(translation)
        });

        let new_rotation = rotation.map(|rotation| {
            maybe_parent_transform
                .map(|parent_transform| rotation * parent_transform.rotation.inverse())
                .unwrap_or(rotation)
        });

        if let Some(translation) = new_translation {
            if transform.translation.distance_squared(translation) > 1e-6 {
                transform.translation = translation;
            }
        }
        if let Some(rotation) = new_rotation {
            if transform.rotation.dot(rotation) < 0.9999 {
                transform.rotation = rotation;
            }
        }
    }
}
