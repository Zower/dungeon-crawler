use bevy::diagnostic::Diagnostics;
use bevy::{
    diagnostic::{DiagnosticId, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::WindowMode,
};
struct Game;

const FPS_ID: DiagnosticId = DiagnosticId::from_u128(288146834822086093791974408528866909483);

impl Plugin for Game {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(print_fps.system());
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Game".to_string(),
            width: 1200.0,
            height: 900.0,
            vsync: false,
            resizable: false,
            mode: WindowMode::Windowed,
            cursor_locked: false,
            cursor_visible: true,
            decorations: true,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(Game)
        .run();
}

fn print_fps(diagnostics: Res<Diagnostics>) {
    if let Some(value) = diagnostics.get(FPS_ID).unwrap().value() {
        println!("{}", value);
    }
}
