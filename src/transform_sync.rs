use crate::prelude::*;
use avian::sync::SyncConfig;
use bevy::ecs::system::RunSystemOnce as _;

pub(super) fn plugin(app: &mut App) {
    app.world_mut().run_system_once(disable_transform_sync);
    app.add_systems(
        PreUpdate,
        disable_transform_sync.run_if(resource_added::<SyncConfig>),
    );
}

fn disable_transform_sync(sync_config: Option<ResMut<SyncConfig>>) {
    let Some(mut sync_config) = sync_config else {
        // User either disabled `SyncPlugin` or configured `AvianInterpolationPlugin` before `PhysicsPlugins`.
        return;
    };
    *sync_config = SyncConfig {
        position_to_transform: false,
        transform_to_position: false,
    };
}
