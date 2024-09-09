use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use avian_interpolation3d::prelude::*;
use bevy::{
    app::RunFixedMainLoop, color::palettes::tailwind, input::mouse::MouseMotion, prelude::*,
    time::run_fixed_main_schedule,
};

mod util;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            AvianInterpolationPlugin::default(),
            util::plugin(util::Example::RotateCamera),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            RunFixedMainLoop,
            rotate_camera.before(run_fixed_main_schedule),
        )
        .add_systems(FixedUpdate, follow_camera)
        .run();
}

#[derive(Component)]
struct FollowCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let terrain_material = materials.add(Color::WHITE);
    let prop_material = materials.add(Color::from(tailwind::EMERALD_300));

    commands.spawn((
        Name::new("Player Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            transform: Transform::from_xyz(3.0, 8.0, 3.0),
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 2_000_000.0,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
    ));

    let ground_mesh = meshes.add(Cuboid::new(15.0, 0.25, 15.0));
    commands.spawn((
        Name::new("Ground"),
        PbrBundle {
            mesh: ground_mesh.clone(),
            material: terrain_material.clone(),
            ..default()
        },
    ));

    // These are just here so we have something to look at.
    let terrain_transforms = [
        Transform::default(),
        Transform::from_xyz(7.5, 0.0, 0.0).with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
        Transform::from_xyz(-7.5, 0.0, 0.0).with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
        Transform::from_xyz(0.0, 0.0, 7.5).with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
        Transform::from_xyz(0.0, 0.0, -7.5).with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
    ];
    for (i, transform) in terrain_transforms.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Wall {}", i)),
            PbrBundle {
                mesh: ground_mesh.clone(),
                material: terrain_material.clone(),
                transform: *transform,
                ..default()
            },
        ));
    }

    let box_shape = Cuboid::from_size(Vec3::splat(0.5));
    commands.spawn((
        Name::new("Box"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(box_shape)),
            material: prop_material.clone(),
            ..default()
        },
        RigidBody::Static,
        Collider::from(box_shape),
        FollowCamera,
    ));
}

fn rotate_camera(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in &mut cameras {
        let dt = time.delta_seconds();
        // The factors are just arbitrary mouse sensitivity values.
        // It's often nicer to have a faster horizontal sensitivity than vertical.
        let mouse_sensitivity = Vec2::new(0.12, 0.10);

        for motion in mouse_motion.read() {
            let delta_yaw = -motion.delta.x * dt * mouse_sensitivity.x;
            let delta_pitch = -motion.delta.y * dt * mouse_sensitivity.y;

            const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
            let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
            let yaw = yaw + delta_yaw;
            let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    }
}

fn follow_camera(
    time: Res<Time>,
    mut follow_camera: Query<(&mut Position, &mut Rotation), With<FollowCamera>>,
    camera: Query<&Transform, With<Camera>>,
) {
    for camera in &camera {
        for (mut position, mut rotation) in &mut follow_camera {
            // Moving things to follow the camera will still need some additional interpolation to look smooth,
            // as the camera will update more frequently than the physics engine.
            // This means that the object will *always* lag slightly behind the camera.
            // To make this less jarring, we can run some nice additional nonlinear interpolation.
            let dt = time.delta_seconds();
            let decay_rate = f32::ln(1000.0);
            let alpha = 1.0 - f32::exp(-decay_rate * dt);

            let direction = camera.forward();
            let distance = 2.0;
            let target_pos = camera.translation + direction * distance;
            position.0 = position.0.lerp(target_pos, alpha);

            let target_rotation = camera.rotation;
            rotation.0 = rotation.0.slerp(target_rotation, alpha);
        }
    }
}
