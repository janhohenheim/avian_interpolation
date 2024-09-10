use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use avian_interpolation3d::prelude::*;
use bevy::{
    app::RunFixedMainLoop, color::palettes::tailwind, input::mouse::MouseMotion, prelude::*,
    render::camera::Viewport, time::run_fixed_main_schedule, window::WindowResized,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: Vec2::new(1280.0 * 2.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            AvianInterpolationPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            RunFixedMainLoop,
            rotate_camera.before(run_fixed_main_schedule),
        )
        .add_systems(FixedUpdate, follow_camera)
        .add_systems(Update, set_camera_viewports)
        .run();
}

#[derive(Debug, Component, Deref, DerefMut)]
struct FollowCamera(Entity);

#[derive(Component)]
struct CameraPosition {
    pos: UVec2,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let terrain_material = materials.add(Color::WHITE);
    let on_material = materials.add(Color::from(tailwind::EMERALD_600));
    let off_material = materials.add(Color::from(tailwind::RED_600));

    let box_shape = Cuboid::from_size(Vec3::splat(0.5));

    for (index, (camera_name, camera_pos)) in [
        ("Interpolation ON", Vec3::new(-3.0, 1.0, 0.0)),
        ("Interpolation OFF", Vec3::new(3.0, 1.0, 0.0)),
    ]
    .iter()
    .enumerate()
    {
        let camera = commands
            .spawn((
                Camera3dBundle {
                    transform: Transform::from_translation(*camera_pos),
                    camera: Camera {
                        // Renders cameras with different priorities to prevent ambiguities
                        order: index as isize,
                        ..default()
                    },
                    ..default()
                },
                CameraPosition {
                    pos: UVec2::new((index % 2) as u32, (index / 2) as u32),
                },
            ))
            .id();

        // Set up UI
        commands
            .spawn((
                TargetCamera(camera),
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(*camera_name, TextStyle::default()).with_style(
                        Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(12.),
                            left: Val::Px(12.),
                            ..default()
                        },
                    ),
                );
            });

        let interpolation_mode = if index == 0 {
            InterpolationMode::Linear
        } else {
            InterpolationMode::None
        };
        let material = if index == 0 {
            on_material.clone()
        } else {
            off_material.clone()
        };
        commands.spawn((
            Name::new("Box"),
            PbrBundle {
                mesh: meshes.add(Mesh::from(box_shape)),
                material,
                ..default()
            },
            RigidBody::Static,
            Collider::from(box_shape),
            FollowCamera(camera),
            interpolation_mode,
        ));
    }

    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            transform: Transform::from_xyz(3.0, 8.0, 3.0),
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 10_000_000.0,
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
}

fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<(&CameraPosition, &mut Camera)>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        let mut size = window.physical_size();
        size.x /= 2;

        for (camera_position, mut camera) in &mut query {
            camera.viewport = Some(Viewport {
                physical_position: camera_position.pos * size,
                physical_size: size,
                ..default()
            });
        }
    }
}

fn rotate_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    for motion in mouse_motion.read() {
        for mut transform in &mut cameras {
            // The factors are just arbitrary mouse sensitivity values.
            // It's often nicer to have a faster horizontal sensitivity than vertical.
            let mouse_sensitivity = Vec2::new(0.003, 0.002);

            let delta_yaw = -motion.delta.x * mouse_sensitivity.x;
            let delta_pitch = -motion.delta.y *  mouse_sensitivity.y;

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
    mut follow_camera: Query<(&mut Position, &mut Rotation, &FollowCamera)>,
    cameras: Query<&Transform>,
) {
    for (mut position, mut rotation, camera_entity) in &mut follow_camera {
        let camera = cameras.get(camera_entity.0).unwrap();
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
