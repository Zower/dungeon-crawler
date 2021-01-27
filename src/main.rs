//#![windows_subsystem = "windows"]

mod level;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::WindowMode,
};

struct Player;
#[derive(Debug)]
struct GridPosition {
    x: i32,
    y: i32,
}

// struct Size {
//     length: i32,
//     height: i32,
// }

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
        .add_plugin(level::Level)
        .add_startup_system(setup.system())
        .add_system(update_fps.system())
        .add_system(move_player_grid.system())
        .add_system(move_player_transform.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_char = asset_server.load("chars/new_juniper.png");

    commands
        .spawn(CameraUiBundle::default())
        .spawn(Camera2dBundle::default());

    commands
        .spawn(TextBundle {
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
        })
        .spawn(SpriteBundle {
            material: materials.add(texture_char.into()),
            ..Default::default()
        })
        .with(Player)
        .with(GridPosition { x: 0, y: 0 });
}

fn move_player_grid(
    keyboard_input: Res<Input<KeyCode>>,
    mut grid: Query<&mut GridPosition, With<Player>>,
) {
    for mut pos in grid.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            pos.y += 1;
        }
        if keyboard_input.pressed(KeyCode::A) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::S) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::D) {
            pos.x += 1;
        }
    }
}

fn move_player_transform(mut query: Query<(&GridPosition, &mut Transform), With<Player>>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.y = pos.y as f32 * 5.0;
        transform.translation.x = pos.x as f32 * 5.0;
        transform.translation.y = pos.y as f32 * 5.0;
        transform.translation.x = pos.x as f32 * 5.0;
    }
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
