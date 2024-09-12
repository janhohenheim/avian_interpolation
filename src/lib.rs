#![allow(clippy::too_many_arguments, clippy::type_complexity, unexpected_cfgs)]
#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

#[cfg(all(feature = "2d", feature = "3d"))]
compile_error!("Cannot enable both 2d and 3d features at the same time.");
#[cfg(all(not(feature = "2d"), not(feature = "3d")))]
compile_error!("Cannot run without either 2d or 3d feature.");

// This allows me to reference `avian` components in the docs without the annoying `cfg_attr`.
#[allow(unused_imports)]
use avian::prelude::*;

#[cfg(feature = "2d")]
use avian2d as avian;
#[cfg(feature = "3d")]
use avian3d as avian;
use bevy::{app::RunFixedMainLoop, prelude::*, time::run_fixed_main_schedule};

/// Everything you need to interpolate transforms with Avian.
pub mod prelude {
    pub(crate) use crate::avian::{self, prelude::*};
    pub(crate) use crate::{AvianInterpolationFixedSystem, AvianInterpolationVariableSystem};
    pub use crate::{AvianInterpolationPlugin, DisableTransformChanges, InterpolationMode};
    pub(crate) use bevy::prelude::*;
}

mod interpolate;
mod lifecycle;
mod previous_transform;
mod transform_sync;

/// The plugin for [`Transform`] interpolation with Avian. Simply add it to your app after [`PhysicsPlugins`]:
///
/// ```rust,no_run
/// use bevy::prelude::*;
#[cfg_attr(feature = "2d", doc = "use avian2d::prelude::*;")]
#[cfg_attr(feature = "3d", doc = "use avian3d::prelude::*;")]
#[cfg_attr(feature = "2d", doc = "use avian_interpolation2d::prelude::*;")]
#[cfg_attr(feature = "3d", doc = "use avian_interpolation3d::prelude::*;")]
/// App::new()
///     .add_plugins((
///         DefaultPlugins,
///         PhysicsPlugins::default(),
///         AvianInterpolationPlugin::default(),
///     ));
/// ```
///
/// That's already it! Now, all your rigid bodies will be interpolated.
/// The interpolation source will be their [`Position`] and [`Rotation`].
#[derive(Default)]
#[non_exhaustive]
pub struct AvianInterpolationPlugin;

impl Plugin for AvianInterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<(InterpolationMode, DisableTransformChanges)>();
        app.add_plugins((
            previous_transform::plugin,
            interpolate::plugin,
            lifecycle::plugin,
            transform_sync::plugin,
        ));
        app.configure_sets(
            FixedPreUpdate,
            (
                AvianInterpolationFixedSystem::First,
                AvianInterpolationFixedSystem::CachePreviousPhysicsTransform,
                AvianInterpolationFixedSystem::Last,
            )
                .chain(),
        );
        app.configure_sets(
            RunFixedMainLoop,
            (
                AvianInterpolationVariableSystem::First,
                AvianInterpolationVariableSystem::Interpolate,
                AvianInterpolationVariableSystem::Last,
            )
                .after(run_fixed_main_schedule)
                .chain(),
        );
    }
}

/// The interpolation mode to use.
/// Change this value to set the interpolation mode for a rigid body.
///
/// This is added to rigid bodies for you, 
/// but you can also manually initialize it yourself to override the interpolation mode.
#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq, Component, Reflect)]
#[reflect(Component)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum InterpolationMode {
    /// Linear interpolation, i.e. the transform used is interpolated between the last two physics transforms.
    /// This is the default.
    #[default]
    Linear,
    /// No interpolation, i.e. the transform used is the last available physics transform.
    None,
}

/// Disables transform changes for a rigid body.
/// Add this to entities that you know will never move for a little performance boost.
/// Note that [`RigidBody::Static`] entities are never interpolated, so adding this to them is pointless.
/// You can also add it to an entity to implement a different kind of smoothing strategy manually, e.g. extrapolation.
///
/// Note that if the entity's physics transform is changed directly, the [`Transform`] will not be updated.
#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq, Component, Reflect)]
#[reflect(Component)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct DisableTransformChanges;

/// The system set for the fixed update loop.
/// This is scheduled in [`FixedPreUpdate`].
/// This means that it is run *before* the user code in [`FixedUpdate`] and the physics update in [`FixedPostUpdate`].
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[non_exhaustive]
pub enum AvianInterpolationFixedSystem {
    /// The first system in the set. This is empty by default.
    First,
    /// Cache the previous physics transform.
    CachePreviousPhysicsTransform,
    /// The last system in the set. This is empty by default.
    Last,
}

/// The system set for the variable update loop.
/// This is scheduled in [`RunFixedMainLoop`] and runs after [`run_fixed_main_schedule`].
/// This means that it is every frame, and if there were any fixed updates this frame,
/// it is run after the last fixed update. This also means that this is run after all physics updates.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[non_exhaustive]
pub enum AvianInterpolationVariableSystem {
    /// The first system in the set. This is empty by default.
    First,
    /// Interpolate the transforms.
    Interpolate,
    /// The last system in the set. This is empty by default.
    Last,
}
