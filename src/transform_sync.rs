use crate::prelude::*;
use avian::sync::SyncConfig;
use bevy::ecs::system::RunSystemOnce as _;

pub(super) fn plugin(app: &mut App) {
    app.world_mut().run_system_once(disable_transform_sync);
}

fn disable_transform_sync(sync_config: Option<ResMut<SyncConfig>>) {
    let Some(mut sync_config) = sync_config else {
        let interpolation_plugin = if cfg!(feature = "2d") {
            "AvianInterpolation2dPlugin"
        } else if cfg!(feature = "3d") {
            "AvianInterpolation3dPlugin"
        } else {
            unimplemented!("Neither 2d nor 3d feature is enabled. This is already a compile-time error, so this message should never be shown.");
        };

        panic!(
            "Failed to find `SyncConfig` in world. \
            This probably means that you forgot to add the avian physics plugins \
            to your app before adding the avian interpolation plugin.\n\
            Help: try the following:\n\
            `app.add_plugins((PhysicsPlugins::default(), {interpolation_plugin}))`"
        );
    };
    *sync_config = SyncConfig {
        position_to_transform: false,
        transform_to_position: false,
    };
}
