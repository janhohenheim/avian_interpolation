#![allow(clippy::too_many_arguments, clippy::type_complexity, unexpected_cfgs)]
#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

use bevy::prelude::*;

pub mod prelude {
    #[cfg(feature = "2d")]
    pub use crate::AvianInterpolation2dPlugin;
    #[cfg(feature = "3d")]
    pub use crate::AvianInterpolation3dPlugin;
}

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
    fn build(&self, _app: &mut App) {}
}
