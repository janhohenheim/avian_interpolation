use bevy::app::RunFixedMainLoop;

use crate::prelude::*;
use crate::previous_transform::{PreviousPosition, PreviousRotation};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        RunFixedMainLoop,
        interpolate_rigidbodies.in_set(AvianInterpolationVariableSystem::Interpolate),
    );
}

fn interpolate_rigidbodies(
    fixed_time: Res<Time<Fixed>>,
    mut q_interpolant: Query<
        (
            &mut Transform,
            Option<&Parent>,
            &Position,
            &Rotation,
            &PreviousPosition,
            &PreviousRotation,
            &InterpolationMode,
        ),
        Without<DisableTransformChanges>,
    >,
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
        interpolation_mode,
    ) in &mut q_interpolant
    {
        let translation = match interpolation_mode {
            InterpolationMode::Linear => previous_position.lerp(position.0, alpha),
            InterpolationMode::None => position.0,
        };
        #[cfg(feature = "2d")]
        let translation = translation.extend(0.);

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

        let rotation = match interpolation_mode {
            InterpolationMode::Linear => previous_rotation.slerp(rotation, alpha),
            InterpolationMode::None => rotation,
        };

        let global_transform = GlobalTransform::from(Transform {
            translation,
            rotation,
            ..default()
        });

        let new_transform = maybe_parent
            .and_then(|parent| q_global_transform.get(parent.get()).ok())
            .map(|parent_global_transform| global_transform.reparented_to(parent_global_transform))
            .unwrap_or_else(|| global_transform.compute_transform());
        if transform
            .translation
            .distance_squared(new_transform.translation)
            > 1e-6
        {
            transform.translation = new_transform.translation;
        }
        if transform.rotation.dot(new_transform.rotation) < 0.9999 {
            transform.rotation = new_transform.rotation;
        }
    }
}
