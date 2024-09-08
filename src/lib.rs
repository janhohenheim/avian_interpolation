#![allow(clippy::too_many_arguments, clippy::type_complexity, unexpected_cfgs)]
//#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

#[cfg(all(feature = "2d", feature = "3d"))]
compile_error!("Cannot enable both 2d and 3d features at the same time.");
#[cfg(all(not(feature = "2d"), not(feature = "3d")))]
compile_error!("Cannot run without either 2d or 3d feature.");

use avian::prelude::*;
#[cfg(feature = "2d")]
use avian2d as avian;
#[cfg(feature = "3d")]
use avian3d as avian;
use bevy::{app::RunFixedMainLoop, prelude::*, time::run_fixed_main_schedule};

pub mod prelude {
    pub(crate) use crate::avian::{self, prelude::*};
    #[cfg(feature = "2d")]
    pub use crate::AvianInterpolation2dPlugin;
    #[cfg(feature = "3d")]
    pub use crate::AvianInterpolation3dPlugin;
    pub use crate::NonInterpolated;
    pub(crate) use crate::{FixedAvianInterpolationSystem, VariableAvianInterpolationSystem};
    pub(crate) use bevy::prelude::*;
}

mod interpolate;
mod lifecycle;
mod previous_transform;
mod transform_sync;

#[cfg(feature = "2d")]
#[derive(Default)]
#[non_exhaustive]
pub struct AvianInterpolation2dPlugin;

#[cfg(feature = "3d")]
#[derive(Default)]
#[non_exhaustive]
pub struct AvianInterpolation3dPlugin;

#[cfg(feature = "2d")]
type AvianInterpolationPlugin = AvianInterpolation2dPlugin;

#[cfg(feature = "3d")]
type AvianInterpolationPlugin = AvianInterpolation3dPlugin;

impl Plugin for AvianInterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<NonInterpolated>();
        app.add_plugins((
            previous_transform::plugin,
            interpolate::plugin,
            lifecycle::plugin,
            transform_sync::plugin,
        ));
        app.configure_sets(
            FixedPreUpdate,
            (
                FixedAvianInterpolationSystem::First,
                FixedAvianInterpolationSystem::CachePreviousPhysicsTransform,
                FixedAvianInterpolationSystem::Last,
            )
                .in_set(PhysicsStepSet::First)
                .chain(),
        );
        app.configure_sets(
            RunFixedMainLoop,
            (
                VariableAvianInterpolationSystem::First,
                VariableAvianInterpolationSystem::Interpolate,
                VariableAvianInterpolationSystem::Last,
            )
                .after(run_fixed_main_schedule)
                .chain(),
        );
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct NonInterpolated;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[non_exhaustive]
pub enum FixedAvianInterpolationSystem {
    First,
    CachePreviousPhysicsTransform,
    Last,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[non_exhaustive]
pub enum VariableAvianInterpolationSystem {
    First,
    Interpolate,
    Last,
}
