//#![windows_subsystem = "windows"]
//! A to be failed attempt at a 2D pixel dungeon-crawler
mod fps_diagnostic;
mod level;

use fps_diagnostic::FPSScreenDiagnostic;

use level::LevelBuilder;

use bevy::prelude::*;

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
            mode: bevy::window::WindowMode::Windowed,
            cursor_locked: false,
            cursor_visible: true,
            decorations: true,
        })
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_resource(LevelBuilder::default())
        .add_plugins(DefaultPlugins)
        //.add_plugin(LevelPlugin)
        .add_plugin(FPSScreenDiagnostic)
        .add_startup_system(setup.system())
        .add_system(move_player_grid.system())
        .add_system(move_player_transform.system())
        .add_system(update_camera.system())
        .add_startup_system(test_builder.system())
        .run();
}

fn test_builder(level_builder: Res<LevelBuilder>) {
    let level = level_builder.build();
    level.print();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_char = asset_server.load("chars/new_juniper.png");

    // Cameras
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

fn update_camera(
    mut query_cam: Query<(&bevy::render::camera::Camera, &mut Transform)>,
    query_player: Query<&Transform, With<Player>>,
) {
    for (cam, mut trans_cam) in query_cam.iter_mut() {
        if cam.name == Some(String::from("Camera2d")) {
            for trans_player in query_player.iter() {
                trans_cam.translation.x = trans_player.translation.x;
                trans_cam.translation.y = trans_player.translation.y;
            }
        }
    }
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
