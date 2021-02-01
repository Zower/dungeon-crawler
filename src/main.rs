// #![windows_subsystem = "windows"]
//! A to be failed attempt at a 2D pixel dungeon-crawler

use bevy::{prelude::*, winit::WinitWindows};

mod fps_diagnostic;
mod level;
mod mouse;
mod movement;

use fps_diagnostic::FPSScreenDiagnostic;
use level::{GridPiece, LevelBuilder, LevelSize};
use mouse::MousePlugin;
use movement::{Direction, MoveState, MovementPlugin};

struct Player;

struct Tile;
#[derive(Debug)]
pub struct GridPosition {
    x: i32,
    y: i32,
}

struct Path(Vec<GridPiece>);

#[derive(Debug)]
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
        .add_plugins(DefaultPlugins)
        .add_plugin(MovementPlugin)
        .add_plugin(FPSScreenDiagnostic)
        .add_plugin(MousePlugin)
        .add_startup_system(build_level.system())
        .add_startup_system(set_icon.system())
        .add_startup_system(setup.system())
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
        .add_tile(
            level::TileType::Floor,
            materials.add(asset_server.load("tiles/floor.png").into()),
        )
        .set_size(LevelSize {
            height: 7,
            width: 7,
        })
        .build()
        .unwrap();

    for rows in level.tiles() {
        for piece in rows {
            commands
                .spawn(SpriteBundle {
                    material: level.get_texture(piece.gridx() as usize, piece.gridy() as usize),
                    transform: Transform {
                        translation: Vec3::new(piece.x() as f32, piece.y() as f32, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(GridPosition {
                    x: piece.gridx(),
                    y: piece.gridy(),
                })
                .with(Tile);
        }
    }

    levels.0.push(level);
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
        .with(MoveState(Direction::Still))
        .with(Path(Vec::<GridPiece>::new()));
}
