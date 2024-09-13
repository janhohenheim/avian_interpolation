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
    pub use crate::{AvianInterpolationPlugin, InterpolateTransformFields, InterpolationMode};
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
///         // Disabling SyncPlugin is optional, but will get you a performance boost.
///         PhysicsPlugins::default().build().disable::<SyncPlugin>(),
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
        app.register_type::<InterpolateTransformFields>();
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

/// Controls which fields of the transform are interpolated. This component is absent by default,
/// in which case all fields are assumed to be [`InterpolationMode::Linear`].
/// You can insert this component into non-static rigid bodies to interpolate only certain fields.
///
/// Placing this on something else than a non-static rigid body will have no effect.
#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq, Component, Reflect)]
#[reflect(Component, Default, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct InterpolateTransformFields {
    /// Whether to interpolate [`Transform::translation`] based on [`Position`].
    pub translation: InterpolationMode,
    /// Whether to interpolate [`Transform::rotation`] based on [`Rotation`].
    pub rotation: InterpolationMode,
}

impl From<InterpolationMode> for InterpolateTransformFields {
    fn from(mode: InterpolationMode) -> Self {
        Self {
            translation: mode,
            rotation: mode,
        }
    }
}

/// The interpolation mode to use on a given transform field in [`InterpolateTransformFields`].
#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq, Reflect)]
#[reflect(Default, PartialEq)]
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
    /// No interpolation, the transform used is the last available physics transform.
    /// This behaves as if you did not activate the [`AvianInterpolationPlugin`] for this field.
    Last,
    /// No interpolation, don't change the transform at all. Use this if you want to control
    /// the [`Transform`] yourself to implement custom interpolation logic like extrapolation.
    None,
}

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
