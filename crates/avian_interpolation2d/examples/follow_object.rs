use avian2d::prelude::*;
use avian_interpolation2d::prelude::*;
use bevy::{
    app::RunFixedMainLoop,
    color::palettes::tailwind,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    time::run_fixed_main_schedule,
};

mod util;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(200.0),
            AvianInterpolationPlugin::default(),
            util::plugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            RunFixedMainLoop,
            handle_input.before(run_fixed_main_schedule),
        )
        .add_systems(FixedUpdate, clear_accumulated_input)
        .add_systems(Update, follow_target)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let prop_material = materials.add(Color::from(tailwind::EMERALD_300));
    let pillar_material = materials.add(Color::from(tailwind::RED_300));

    commands.spawn((Name::new("Player Camera"), Camera2dBundle::default()));

    let tile_mesh = Mesh2dHandle(meshes.add(Rectangle::from_size(Vec2::splat(200.0))));

    // Take a look at this reference background while running the example to see the effect of the interpolation.
    let tile_repeat_per_quadrant = 10;
    let tile_spacing = 210.0;
    for i in -tile_repeat_per_quadrant..tile_repeat_per_quadrant {
        for j in -tile_repeat_per_quadrant..tile_repeat_per_quadrant {
            let x = i as f32 * tile_spacing;
            let y = j as f32 * tile_spacing;
            commands.spawn((
                Name::new("Background Tile"),
                MaterialMesh2dBundle {
                    mesh: tile_mesh.clone(),
                    material: pillar_material.clone(),
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
            ));
        }
    }

    let box_shape = Rectangle::from_size(Vec2::splat(50.));
    commands.spawn((
        Name::new("Box"),
        MaterialMesh2dBundle {
            mesh: meshes.add(box_shape).into(),
            material: prop_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::from(box_shape),
        AccumulatedInput::default(),
        Moving,
    ));
}

#[derive(Debug, Component)]
struct Moving;

#[derive(Debug, Component, Default, Deref, DerefMut)]
struct AccumulatedInput(Vec2);

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
            transform.translation.y = target.y;
        }
    }
}

/// Handle keyboard input and accumulate it in the `AccumulatedInput` component.
/// There are many strategies for how to handle all the input that happened since the last fixed timestep.
/// This is a very simple one: we just accumulate the input and average it out by normalizing it.
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AccumulatedInput, &mut LinearVelocity)>,
) {
    /// Since Bevy's default 2D camera setup is scaled such that
    /// one unit is one pixel, you can think of this as
    /// "How many pixels per second should the player move?"
    const SPEED: f32 = 800.0;
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
        velocity.0 = input.normalize_or_zero() * SPEED;
    }
}
