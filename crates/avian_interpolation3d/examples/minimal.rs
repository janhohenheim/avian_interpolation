use avian3d::prelude::*;
use avian_interpolation3d::prelude::*;
use bevy::{
    app::RunFixedMainLoop, color::palettes::tailwind, input::mouse::MouseMotion, prelude::*,
    time::run_fixed_main_schedule,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            AvianInterpolation3dPlugin::default(),
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
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((Name::new("Player Camera"), Camera3dBundle::default()));

    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            transform: Transform::from_xyz(3.0, 8.0, 3.0),
            ..default()
        },
    ));

    let box_shape = Cuboid::from_size(Vec3::splat(0.5));
    commands.spawn((
        Name::new("Box"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(box_shape)),
            material: materials.add(Color::from(tailwind::EMERALD_300)),
            transform: Transform::from_xyz(0.0, 0.0, -5.),
            ..default()
        },
        RigidBody::Static,
        Collider::from(box_shape),
        Moving,
    ));
}

fn move_box(time: Res<Time>, mut moving: Query<&mut Position, With<Moving>>) {
    let elapsed = time.elapsed_seconds();
    let max_offset = 3.;
    for mut position in &mut moving {
        position.0.x = elapsed.sin() * max_offset;
    }
}
