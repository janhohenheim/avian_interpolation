use avian3d::prelude::*;
use avian_interpolation3d::prelude::*;
use bevy::{
    app::RunFixedMainLoop, color::palettes::tailwind, prelude::*, time::run_fixed_main_schedule,
};

mod util;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Disabling SyncPlugin is optional, but will get you a performance boost.
            PhysicsPlugins::default().build().disable::<SyncPlugin>(),
            AvianInterpolationPlugin::default(),
            util::plugin(util::Example::Moving),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            RunFixedMainLoop,
            handle_input.before(run_fixed_main_schedule),
        )
        .add_systems(FixedUpdate, clear_accumulated_input)
        .add_systems(Update, follow_target.chain())
        .run();
}

#[derive(Component)]
struct Moving;

#[derive(Debug, Component, Default, Deref, DerefMut)]
struct AccumulatedInput(Vec2);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let prop_material = materials.add(Color::from(tailwind::EMERALD_300));
    let pillar_material = materials.add(Color::from(tailwind::RED_300));

    commands.spawn((
        Name::new("Player Camera"),
        Camera3dBundle {
            // top-down view
            transform: Transform::from_xyz(0.0, 8.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            transform: Transform::from_xyz(0.0, 8.0, 0.0),
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 10_000_000.0,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
    ));

    let tile_mesh = meshes.add(Cuboid::from_size(Vec3::splat(3.0)));

    // Take a look at this reference background while running the example to see the effect of the interpolation.
    let tile_repeat_per_quadrant = 10;
    let tile_spacing = 3.5;
    for i in -tile_repeat_per_quadrant..tile_repeat_per_quadrant {
        for j in -tile_repeat_per_quadrant..tile_repeat_per_quadrant {
            let x = i as f32 * tile_spacing;
            let z = j as f32 * tile_spacing;
            commands.spawn((
                Name::new("Background Tile"),
                PbrBundle {
                    mesh: tile_mesh.clone(),
                    material: pillar_material.clone(),
                    transform: Transform::from_xyz(x, -1.5, z),
                    ..default()
                },
            ));
        }
    }

    let box_shape = Cuboid::from_size(Vec3::splat(0.5));
    commands.spawn((
        Name::new("Box"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(box_shape)),
            material: prop_material.clone(),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::from(box_shape),
        AccumulatedInput::default(),
        Moving,
    ));
}

/// Handle keyboard input and accumulate it in the `AccumulatedInput` component.
/// There are many strategies for how to handle all the input that happened since the last fixed timestep.
/// This is a very simple one: we just accumulate the input and average it out by normalizing it.
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AccumulatedInput, &mut LinearVelocity)>,
) {
    const SPEED: f32 = 12.0;
    for (mut input, mut velocity) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            input.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            input.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            input.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            input.x += 1.0;
        }

        // Need to normalize and scale because otherwise
        // diagonal movement would be faster than horizontal or vertical movement.
        // This effectively averages the accumulated input.
        velocity.0 = -input.extend(0.0).yzx().normalize_or_zero() * SPEED;
    }
}

fn clear_accumulated_input(mut inputs: Query<&mut AccumulatedInput, With<Moving>>) {
    for mut input in &mut inputs {
        // Reset the input accumulator, as we are currently consuming all input that happened since the last fixed timestep.
        input.0 = Vec2::ZERO;
    }
}

fn follow_target(
    mut cameras: Query<&mut Transform, With<Camera>>,
    target: Query<&Transform, (With<Moving>, Without<Camera>)>,
) {
    for mut transform in &mut cameras {
        for target_transform in &target {
            let target = target_transform.translation;
            transform.translation.x = target.x;
            transform.translation.z = target.z;
        }
    }
}
