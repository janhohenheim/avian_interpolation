use std::f32::consts::TAU;

use avian2d::prelude::*;
use avian_interpolation2d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*, sprite::MaterialMesh2dBundle};

mod util;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(200.0),
            AvianInterpolation2dPlugin::default(),
            util::plugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_box)
        .run();
}

#[derive(Component)]
struct Moving;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Name::new("Player Camera"), Camera2dBundle::default()));

    let box_shape = Rectangle::from_size(Vec2::splat(100.0));
    commands.spawn((
        Name::new("Box"),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(box_shape)).into(),
            material: materials.add(Color::from(tailwind::EMERALD_300)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::from(box_shape),
        Moving,
    ));
}

fn move_box(time: Res<Time>, mut moving: Query<&mut Position, With<Moving>>) {
    let elapsed = time.elapsed_seconds();
    let max_offset = 300.;
    let oscillations_per_second = 0.6;
    for mut position in &mut moving {
        let interpolant = elapsed * oscillations_per_second * TAU;
        position.0.x = interpolant.sin() * max_offset;
    }
}
