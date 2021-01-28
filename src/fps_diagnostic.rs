//! Prints fps at top left of screen

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
pub struct FPSScreenDiagnostic;

struct FPSTimer(Timer);

impl Plugin for FPSScreenDiagnostic {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(FPSTimer(Timer::from_seconds(0.1, true)))
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_system(update_fps.system());
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
