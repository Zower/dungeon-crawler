use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    prelude::*,
    window::WindowMode,
};

struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut AppBuilder) {}
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
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(Game)
        .run();
}
