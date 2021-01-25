//#![windows_subsystem = "windows"]

use bevy::diagnostic::Diagnostics;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowMode};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Game".to_string(),
            width: 800.0,
            height: 600.0,
            vsync: false,
            resizable: true,
            mode: WindowMode::Windowed,
            cursor_locked: false,
            cursor_visible: true,
            decorations: true,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup.system())
        .add_system(update_fps.system())
        .run();
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands.spawn(CameraUiBundle::default()).spawn(TextBundle {
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

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text>) {
    if let Some(value) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .unwrap()
        .value()
    {
        let fps = value as i32;
        for mut text in query.iter_mut() {
            text.value = "FPS: ".to_owned() + &fps.to_string();
        }
    }
}
