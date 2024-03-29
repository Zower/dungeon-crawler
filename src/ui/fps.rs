//! Prints fps at top left of screen

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_console::ConsoleCommand;

use super::console::AddConvar;

/// The plugin representing the FPS UI element
pub struct FPSPlugin;

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup)
            .add_system(update_text)
            .add_system(toggle_visibility)
            .add_convar(UiFps { on: true });
    }
}

// Component held by the TextBundle to identify the right text.
#[derive(Debug, Component)]
struct FPSText;

#[derive(Debug, Default, ConsoleCommand)]
#[console_command(name = "fps")]
/// Toggles FPS counter
struct UiFps {
    on: bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(5.),
                    right: Val::Percent(5.),
                    ..Default::default()
                },
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
            visibility: Visibility { is_visible: true },
            ..Default::default()
        })
        .insert(FPSText);
    // .insert(Timer::from_seconds(0.1, true));
}

/// System that checks if visibility should be toggled based on UiFps Convar.
fn toggle_visibility(
    fps_toggled: Res<UiFps>,
    mut fps_text_query: Query<&mut Visibility, With<FPSText>>,
) {
    fps_text_query.single_mut().is_visible = fps_toggled.on;
}

/// System that updates the FPS values
fn update_text(
    diagnostics: Res<Diagnostics>,
    mut fps_text_query: Query<(&mut Text, &Visibility), With<FPSText>>,
) {
    let (mut text, visible) = fps_text_query.single_mut();
    if visible.is_visible {
        if let Some(diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = diagnostic.value() {
                if let Some(avg) = diagnostic.average() {
                    let value = value as i32;
                    let avg = avg as i32;
                    text.sections[1].value = format!("{value} ({avg})");
                }
            }
        }
    }
}
