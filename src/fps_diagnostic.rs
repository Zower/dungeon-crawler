//! Prints fps at top left of screen

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FPSScreenDiagnostic;
struct FPSTimer(Timer);

struct DiagnosticText(DiagnosticState);
#[derive(Debug)]
enum DiagnosticState {
    On,
    Off,
}

impl Plugin for FPSScreenDiagnostic {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(FPSTimer(Timer::from_seconds(0.1, true)))
            .add_resource(DiagnosticText(DiagnosticState::On))
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup.system())
            .add_system(update_fps.system())
            .add_system(check_on.system());
    }
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TextBundle {
        text: Text {
            value: "FPS: ".to_string(),
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            style: TextStyle {
                font_size: 25.0,
                alignment: TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Left,
                },
                color: Color::AQUAMARINE,
            },
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn check_on(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<DiagnosticText>,
    mut texts: Query<Entity, With<Text>>, // Currently finds all text, TODO: Fix
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        match state.0 {
            DiagnosticState::On => {
                for entity in texts.iter_mut() {
                    commands.despawn(entity);
                }
                state.0 = DiagnosticState::Off
            }
            DiagnosticState::Off => {
                commands.spawn(TextBundle {
                    text: Text {
                        value: "FPS: ".to_string(),
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        style: TextStyle {
                            font_size: 25.0,
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Top,
                                horizontal: HorizontalAlign::Left,
                            },
                            color: Color::AQUAMARINE,
                        },
                    },
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Px(5.0),
                            left: Val::Px(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                });
                state.0 = DiagnosticState::On
            }
        }
    }
}

fn update_fps(
    time: Res<Time>,
    mut timer: ResMut<FPSTimer>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    let diagnostic = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).unwrap();
    if let Some(mut value) = diagnostic.value() {
        if let Some(mut avg) = diagnostic.average() {
            value = (value as i32).into();
            avg = (avg as i32).into();
            for mut text in query.iter_mut() {
                text.value =
                    "FPS: ".to_owned() + &value.to_string() + " (" + &avg.to_string() + ")";
            }
        }
    }
}
