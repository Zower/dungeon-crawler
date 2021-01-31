// #![windows_subsystem = "windows"]
//! A to be failed attempt at a 2D pixel dungeon-crawler
mod fps_diagnostic;
mod level;
mod mouse;

use fps_diagnostic::FPSScreenDiagnostic;

use level::{LevelBuilder, LevelSize};

use mouse::MousePlugin;

use bevy::{prelude::*, winit::WinitWindows};

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
        .add_resource(MoveTimer(Timer::from_seconds(0.08, true)))
        .add_plugins(DefaultPlugins)
        .add_plugin(FPSScreenDiagnostic)
        .add_plugin(MousePlugin)
        .add_startup_system(build_level.system())
        .add_startup_system(set_icon.system())
        .add_startup_system(setup.system())
        .add_system(update_direction.system())
        .add_system(move_player.system())
        .add_system(update_camera.system())
        .run();
}

fn build_level(
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
            height: 7,
            width: 7,
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

fn update_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut MoveState, With<Player>>,
) {
    for mut move_state in query.iter_mut() {
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
    }
}

fn move_player(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    levels: Res<Levels>,
    mut query: Query<(&mut GridPosition, &mut Transform), With<Player>>,
    mut state: Query<&MoveState, With<Player>>,
) {
    if !timer.0.tick(time.delta_seconds()).finished() {
        return;
    }

    for (mut pos, mut transform) in query.iter_mut() {
        for move_state in state.iter_mut() {
            match move_state.0 {
                Direction::Up => pos.y += 1,
                Direction::Down => pos.y -= 1,
                Direction::Left => pos.x -= 1,
                Direction::Right => pos.x += 1,
                Direction::Still => (),
            }
        }
        //TODO: Only if changed
        let trans = levels.0[0].get_translation(pos.x, pos.y);

        transform.translation.x = trans.0;
        transform.translation.y = trans.1;
    }
}

// NOTE(erlend):
// systems that access Resources run on the main thread
// and winit_window.set_window_icon hangs(deadlock?) when it
// runs from a different thread...
fn set_icon(_: &mut World, resources: &mut Resources) {
    let winit_windows = resources.get::<WinitWindows>().unwrap();
    let windows = resources.get::<Windows>().unwrap();

    let img = image::open("assets/logo/logo.png").unwrap();

    if let Some(window) = windows.get_primary() {
        if let Some(winit_window) = winit_windows.get_window(window.id()) {
            winit_window.set_window_icon(Some(
                winit::window::Icon::from_rgba(img.to_bytes(), 32, 32)
                    .expect("Failed to create icon"), //Error handling? No.
            ));
        }
    }
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
<<<<<<< HEAD
    //Set window icon :)
    let img = image::open("assets/logo/logo.png").unwrap();

    if let Some(window) = windows.get_primary() {
        if let Some(winit_window) = winit_windows.get_window(window.id()) {
            winit_window.set_window_icon(Some(
                winit::window::Icon::from_rgba(img.to_bytes(), 32, 32)
                    .expect("Failed to create icon"), //Error handling? No.
            ));
        }
    }

=======
>>>>>>> e24b1e3979c040c7b28e8d1c2027ccdcaa2355a2
    let texture_char = asset_server.load("chars/new_juniper.png");

    // Cameras
    commands
        .spawn(CameraUiBundle::default())
        .spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_char.into()),
            ..Default::default()
        })
        .with(Player)
        .with(GridPosition { x: 0, y: 0 })
        .with(MoveState(Direction::Still));
}
