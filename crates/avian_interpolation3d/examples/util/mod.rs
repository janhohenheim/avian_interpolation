use avian3d::prelude::*;
use avian_interpolation3d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub fn plugin(example: Example) -> impl Plugin {
    move |app: &mut App| {
        app.add_systems(Startup, spawn_text(example))
            // Purely aesthetic systems go in `Update`.
            .add_systems(
                Update,
                (
                    toggle_interpolation.run_if(input_just_pressed(KeyCode::Space)),
                    update_text,
                )
                    .chain(),
            );
        app.observe(add_interpolation_mode);
    }
}

/// Used to tell `spawn_text` which instructions to spawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Example {
    /// The minimal set of instructions.
    #[allow(dead_code)]
    Generic,
    /// Includes instructions for moving a box.
    #[allow(dead_code)]
    Moving,
    /// Includes instructions for rotating the camera.
    #[allow(dead_code)]
    RotateCamera,
}

fn add_interpolation_mode(
    trigger: Trigger<OnAdd, RigidBody>,
    q_rigid_body: Query<&RigidBody>,
    mut commands: Commands,
) {
    let Ok(rigid_body) = q_rigid_body.get(trigger.entity()) else {
        return;
    };
    if rigid_body.is_static() {
        return;
    }
    commands
        .entity(trigger.entity())
        // You don't need this in your own code, we just add it to make sure
        // we have something to toggle in `toggle_interpolation` :)
        .insert(InterpolateTransformFields::default());
}

fn toggle_interpolation(
    mut query: Query<&mut InterpolateTransformFields>,
    mut current_mode: Local<InterpolationMode>,
) {
    for mut interpolation in &mut query {
        interpolation.translation = *current_mode;
        interpolation.rotation = *current_mode;
        *current_mode = match *current_mode {
            InterpolationMode::Linear => InterpolationMode::Last,
            InterpolationMode::Last => InterpolationMode::Linear,
            InterpolationMode::None => unreachable!("Not shown in this example."),
        };
    }
}

#[derive(Component)]
struct InstructionText;

/// Spawn instructions for the user, depending on the example.
fn spawn_text(example: Example) -> impl Fn(Commands) {
    move |mut commands: Commands| {
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
                let text = |val: &str| {
                    TextSection::new(
                        val,
                        TextStyle {
                            font_size: 25.0,
                            ..default()
                        },
                    )
                };
                let sections = match example {
                    Example::Generic => vec![
                        "Press Space to toggle interpolation modes.\n",
                        "Current interpolation mode: ",
                    ],
                    Example::Moving => vec![
                        "Use WASD to move the box.\n",
                        "Press Space to toggle interpolation modes.\n",
                        "Current interpolation mode: ",
                    ],
                    Example::RotateCamera => vec![
                        "Move the mouse to rotate the camera.\n",
                        "Press Space to toggle interpolation modes.\n",
                        "Current interpolation mode: ",
                    ],
                };
                parent.spawn((
                    TextBundle::from_sections(sections.into_iter().map(text)),
                    InstructionText,
                ));
            });
    }
}

fn update_text(
    mut texts: Query<&mut Text, With<InstructionText>>,
    q_interpolation: Query<&InterpolateTransformFields>,
) {
    let Some(interpolation) = q_interpolation.iter().next() else {
        return;
    };
    let interpolated = match interpolation.translation {
        InterpolationMode::Linear => "ON",
        InterpolationMode::Last => "OFF",
        InterpolationMode::None => unreachable!("Not shown in this example."),
    };
    for mut text in &mut texts {
        text.sections.last_mut().unwrap().value =
            format!("Current interpolation mode: {interpolated}");
    }
}
