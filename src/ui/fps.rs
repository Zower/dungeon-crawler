//! Prints fps at top left of screen

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use dungeon_crawler_derive::Convar;

use crate::input::{AddConvar, Convar};

/// The plugin representing the FPS UI element
pub struct FPSPlugin;

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup)
            .add_system(update_text)
            .add_system(toggle_visibility)
            .add_convar::<UiFps>();
    }
}

// Component held by the TextBundle to identify the right text.
#[derive(Debug, Component)]
struct FPSText;

#[derive(Debug, Default, Convar)]
struct UiFps(bool);

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
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(FPSText)
        .insert(Timer::from_seconds(0.1, true));
}

/// System that checks if visibility should be toggled
fn toggle_visibility(
    fps_toggled: Res<UiFps>,
    mut fps_text_query: Query<&mut Visibility, With<FPSText>>,
) {
    let mut visible = fps_text_query.single_mut();

    visible.is_visible = fps_toggled.0;
}

/// System that updates the FPS values, if FPStimer is finished and text is visible
fn update_text(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut fps_text_query: Query<(&mut Text, &Visibility, &mut Timer), With<FPSText>>,
) {
    let (mut text, visible, mut timer) = fps_text_query.single_mut();
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
}
