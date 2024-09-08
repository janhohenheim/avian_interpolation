use avian3d::prelude::*;
use avian_interpolation3d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_text)
        // Purely aesthetic systems go in `Update`.
        .add_systems(
            Update,
            (toggle_interpolation.run_if(input_just_pressed(KeyCode::Space)),).chain(),
        );
}

fn toggle_interpolation(
    query: Query<(Entity, Has<NonInterpolated>), With<Position>>,
    mut commands: Commands,
) {
    for (entity, non_interpolated) in query.iter() {
        if non_interpolated {
            commands.entity(entity).remove::<NonInterpolated>();
        } else {
            commands.entity(entity).insert(NonInterpolated);
        }
    }
}

/// Spawn instructions for the user, depending on the example.
fn spawn_text(mut commands: Commands, query: Query<&NonInterpolated>) {
    let interpolated = if query.is_empty() { "ON" } else { "OFF" };
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!(
                    concat!(
                        "Press Space to toggle interpolation.\n",
                        "Interpolation: {interpolated}",
                    ),
                    interpolated = interpolated
                ),
                TextStyle {
                    font_size: 25.0,
                    ..default()
                },
            ));
        });
}
