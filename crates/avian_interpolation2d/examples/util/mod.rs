use avian_interpolation2d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_text)
        // Purely aesthetic systems go in `Update`.
        .add_systems(
            Update,
            (
                toggle_interpolation.run_if(input_just_pressed(KeyCode::Space)),
                update_text,
            )
                .chain(),
        );
}

fn toggle_interpolation(mut query: Query<&mut InterpolationMode>) {
    for mut interpolation_mode in &mut query {
        *interpolation_mode = match *interpolation_mode {
            InterpolationMode::Linear => InterpolationMode::None,
            InterpolationMode::None => InterpolationMode::Linear,
        };
    }
}

#[derive(Component)]
struct InstructionText;

fn spawn_text(mut commands: Commands) {
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
            parent.spawn((
                TextBundle::from_sections([
                    text("Press Space to toggle interpolation modes.\n"),
                    text("Current interpolation mode: "),
                ]),
                InstructionText,
            ));
        });
}

fn update_text(
    mut texts: Query<&mut Text, With<InstructionText>>,
    interpolation_modes: Query<&InterpolationMode>,
) {
    let Some(interpolation_mode) = interpolation_modes.iter().next() else {
        return;
    };
    let interpolated = match interpolation_mode {
        InterpolationMode::Linear => "Linear",
        InterpolationMode::None => "None",
    };
    for mut text in &mut texts {
        text.sections[1].value = format!("Current interpolation mode: {interpolated}");
    }
}
