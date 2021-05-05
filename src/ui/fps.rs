//! Plugin that prints fps at top left of screen, F1 to toggle.

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

/// The plugin representing the FPS UI element
pub struct FPSPlugin;

// Component held by the TextBundle to identify the right text.
struct FPSText;

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup.system())
            .add_system(update_text.system())
            .add_system(toggle_visibility.system());
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 25.0,
                            color: Color::AQUAMARINE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 25.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..Default::default()
            },
            visible: Visible {
                is_transparent: false,
                is_visible: true,
            },
            ..Default::default()
        })
        .insert(FPSText)
        .insert(Timer::from_seconds(0.1, true));
}

/// System that checks if the user pressed F1, and toggles the visibility accordingly.
fn toggle_visibility(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Visible, With<FPSText>>,
) {
    if let Ok(mut visible) = query.single_mut() {
        if keyboard_input.just_pressed(KeyCode::F1) {
            visible.is_visible = !visible.is_visible
        }
    } else {
        println!("WARNING: FPSText UI element not found");
    }
}

/// System that updates the FPS values, if FPStimer is finished and text is visible
fn update_text(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<(&mut Text, &Visible, &mut Timer), With<FPSText>>,
) {
    if let Ok((mut text, visible, mut timer)) = query.single_mut() {
        // Check if its time to update the FPS
        if visible.is_visible && timer.tick(time.delta()).just_finished() {
            if let Some(diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(mut value) = diagnostic.value() {
                    if let Some(mut avg) = diagnostic.average() {
                        value = (value as i32).into();
                        avg = (avg as i32).into();
                        text.sections[1].value =
                            format!("{} ({})", &value.to_string(), &avg.to_string());
                    }
                }
            }
        }
    } else {
        println!("WARNING: FPSText UI element not found");
    }
}
