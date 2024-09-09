use bevy::app::RunFixedMainLoop;

use crate::prelude::*;
use crate::previous_transform::{PreviousPosition, PreviousRotation, PreviousScale};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        RunFixedMainLoop,
        interpolate_transform.in_set(AvianInterpolationVariableSystem::Interpolate),
    );
}

fn interpolate_transform(
    fixed_time: Res<Time<Fixed>>,
    mut q_interpolant: Query<
        (
            &mut Transform,
            Option<&Parent>,
            &Position,
            &Rotation,
            &PreviousPosition,
            &PreviousRotation,
            Option<(&Collider, &PreviousScale)>,
            &InterpolationMode,
        ),
        Without<DisableInterpolation>,
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

        let scale = if let Some((collider, previous_scale)) = maybe_scale {
            let scale = match interpolation_mode {
                InterpolationMode::Linear => previous_scale.lerp(collider.scale(), alpha),
                InterpolationMode::None => collider.scale(),
            };
            #[cfg(feature = "2d")]
            let scale = scale.extend(1.);
            scale
        } else {
            Vec3::splat(1.0)
        };
        let global_transform = GlobalTransform::from(Transform {
            translation,
            rotation,
            scale,
        });
        let new_transform = if let Some(parent) = maybe_parent {
            if let Ok(parent_global_transform) = q_global_transform.get(parent.get()) {
                global_transform.reparented_to(parent_global_transform)
            } else {
                global_transform.compute_transform()
            }
        } else {
            global_transform.compute_transform()
        };
        *transform = new_transform;
    }
}
