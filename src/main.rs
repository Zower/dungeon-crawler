//#![windows_subsystem = "windows"]
//! A to be failed attempt at a 2D pixel dungeon-crawler
mod fps_diagnostic;
mod level;

use fps_diagnostic::FPSScreenDiagnostic;

use level::{LevelBuilder, LevelSize};

use bevy::prelude::*;

struct Player;

struct MoveState(Direction);

struct MoveTimer(Timer);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Still,
}

#[derive(Debug)]
struct GridPosition {
    x: i32,
    y: i32,
}

struct Levels(Vec<level::Level>);

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
        .add_resource(Levels(Vec::<level::Level>::new()))
        .add_resource(MoveState(Direction::Still))
        .add_resource(MoveTimer(Timer::from_seconds(0.08, true)))
        .add_plugins(DefaultPlugins)
        .add_plugin(FPSScreenDiagnostic)
        .add_startup_system(test_builder.system())
        .add_startup_system(setup.system())
        .add_system(update_direction.system())
        .add_system(move_player.system())
        .add_system(update_camera.system())
        .run();
}
fn test_builder(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut level_builder: ResMut<LevelBuilder>,
    mut levels: ResMut<Levels>,
) {
    let level = level_builder
        .add_texture(
            level::TileType::Floor,
            materials.add(asset_server.load("tiles/floor.png").into()),
        )
        .set_size(LevelSize {
            height: 20,
            length: 20,
        })
        .build()
        .unwrap();

    let mut tiles = vec![];

    for (i, tile) in level.tiles().enumerate() {
        tiles.push(SpriteBundle {
            material: level.get_texture(i),
            transform: Transform {
                translation: Vec3::new(tile.x(), tile.y(), 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
    }

    levels.0.push(level);

    commands.spawn_batch(tiles);
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

fn update_direction(mut move_state: ResMut<MoveState>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::W) {
        move_state.0 = Direction::Up;
    }
    if keyboard_input.pressed(KeyCode::A) {
        move_state.0 = Direction::Left;
    }
    if keyboard_input.pressed(KeyCode::S) {
        move_state.0 = Direction::Down;
    }
    if keyboard_input.pressed(KeyCode::D) {
        move_state.0 = Direction::Right;
    }
    if keyboard_input.just_released(KeyCode::W)
        || keyboard_input.just_released(KeyCode::A)
        || keyboard_input.just_released(KeyCode::S)
        || keyboard_input.just_released(KeyCode::D)
    {
        move_state.0 = Direction::Still
    }

    //println!("{:?}", move_state.0);
}

fn move_player(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    move_state: Res<MoveState>,
    levels: Res<Levels>,
    mut query: Query<(&mut GridPosition, &mut Transform), With<Player>>,
) {
    if !timer.0.tick(time.delta_seconds()).finished() {
        return;
    }

    for (mut pos, mut transform) in query.iter_mut() {
        match move_state.0 {
            Direction::Up => pos.y += 1,
            Direction::Down => pos.y -= 1,
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1,
            Direction::Still => (),
        }
        println!("{:?}, x: {}, y: {}", move_state.0, pos.x, pos.y);

        let trans = levels.0[0].get_translation(pos.x, pos.y);

        transform.translation.x = trans.0;
        transform.translation.y = trans.1;
    }
}

use bevy::winit::WinitWindows;

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
    mut winit_windows: ResMut<WinitWindows>,
) {
    // Set window icon :)
    if let Some(window) = windows.get_primary() {
        if let Some(winit_window) = winit_windows.get_window(window.id()) {
            winit_window.set_window_icon(Some(winit::window::Icon::from_rgba(
                vec![123, 112, 24, 213, 3, 213, 84, 3, 54, 123, 253, 215],
                1,
                3,
            ).unwrap()));
        }
    }

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
