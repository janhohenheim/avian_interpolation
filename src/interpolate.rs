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
            &PreviousPosition,
            &PreviousRotation,
            Option<(&Collider, &PreviousScale)>,
            &InterpolationMode,
        ),
        Without<DisableTransformChanges>,
    >,
) {
    // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
    let alpha = fixed_time.overstep_fraction();

    for (
        mut transform,
        position,
        rotation,
        previous_position,
        previous_rotation,
        maybe_scale,
        interpolation_mode,
    ) in &mut q_interpolant
    {
        let translation = match interpolation_mode {
            InterpolationMode::Linear => previous_position.lerp(position.0, alpha),
            InterpolationMode::None => position.0,
        };
        #[cfg(feature = "2d")]
        let translation = translation.extend(0.);
        transform.translation = translation;

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
            InterpolationMode::Linear => previous_rotation.lerp(rotation, alpha),
            InterpolationMode::None => rotation,
        };
        transform.rotation = rotation;

        if let Some((collider, previous_scale)) = maybe_scale {
            let scale = match interpolation_mode {
                InterpolationMode::Linear => previous_scale.lerp(collider.scale(), alpha),
                InterpolationMode::None => collider.scale(),
            };
            #[cfg(feature = "2d")]
            let scale = scale.extend(0.);
            transform.scale = scale;
        }
    }
}
