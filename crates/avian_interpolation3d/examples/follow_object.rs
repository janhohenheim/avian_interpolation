use std::f32::consts::{FRAC_PI_2, TAU};

use avian3d::prelude::*;
use avian_interpolation3d::prelude::*;
use bevy::{color::palettes::tailwind, input::mouse::MouseMotion, prelude::*};

mod util;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            AvianInterpolation3dPlugin::default(),
            util::plugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_box)
        .add_systems(
            Update,
            (set_target_to_box, orbit_camera, follow_target).chain(),
        )
        .run();
}

#[derive(Component)]
struct Moving;

#[derive(Component, Default)]
struct OrbitCamera {
    target: Vec3,
    distance: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let terrain_material = materials.add(Color::WHITE);
    let prop_material = materials.add(Color::from(tailwind::EMERALD_300));
    let pillar_material = materials.add(Color::from(tailwind::RED_300));

    commands.spawn((
        Name::new("Player Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        OrbitCamera {
            target: Vec3::ZERO,
            distance: 2.0,
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

    // Take a look at this reference pillar while running the example to see the effect of the interpolation.
    let pillar_mesh = meshes.add(Cuboid::new(1.0, 5.0, 1.0));
    commands.spawn((
        Name::new("Pillar"),
        PbrBundle {
            mesh: pillar_mesh.clone(),
            material: pillar_material.clone(),
            transform: Transform::from_xyz(0.0, 2.5, -1.0),
            ..default()
        },
    ));

    let box_shape = Cuboid::from_size(Vec3::splat(0.5));
    commands.spawn((
        Name::new("Box"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(box_shape)),
            material: prop_material.clone(),
            transform: Transform::from_xyz(0.0, 3.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::from(box_shape),
        Moving,
    ));
}

fn move_box(time: Res<Time>, mut moving: Query<&mut Position, With<Moving>>) {
    let elapsed = time.elapsed_seconds();
    let max_offset = 1.3;
    let speed = 0.6;
    for mut position in &mut moving {
        let interpolant = elapsed * speed * TAU;
        let new_position = |a| a * max_offset;
        position.0.x = new_position(interpolant.sin());
    }
}

fn set_target_to_box(target: Query<&Transform, With<Moving>>, mut camera: Query<&mut OrbitCamera>) {
    for mut orbit_camera in &mut camera {
        for target_transform in &target {
            orbit_camera.target = target_transform.translation;
        }
    }
}

fn orbit_camera(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut cameras: Query<&mut Transform, With<OrbitCamera>>,
) {
    for mut transform in &mut cameras {
        let dt = time.delta_seconds();
        // The factors are just arbitrary mouse sensitivity values.
        // It's often nicer to have a faster horizontal sensitivity than vertical.
        let mouse_sensitivity = Vec2::new(0.12, 0.10);

        for motion in mouse_motion.read() {
            let delta_yaw = -motion.delta.x * dt * mouse_sensitivity.x;
            let delta_pitch = -motion.delta.y * dt * mouse_sensitivity.y;

            // Add yaw (global)
            transform.rotate_y(delta_yaw);

            // Add pitch (local)
            const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
            let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
            let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    }
}

fn follow_target(mut cameras: Query<(&mut Transform, &OrbitCamera)>) {
    for (mut transform, orbit) in &mut cameras {
        let target = orbit.target - transform.forward() * orbit.distance;
        transform.translation = target;
    }
}
