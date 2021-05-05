// #![windows_subsystem = "windows"]
//! A to be failed attempt at a 2D pixel dungeon-crawler

use bevy::{prelude::*, winit::WinitWindows};

mod level;
mod mouse;
mod movement;
mod ui;

use level::{GridPosition, LevelBuilder, Tile};
use mouse::MousePlugin;
use movement::MovementPlugin;
use ui::FPSPlugin;

struct Player;

struct Blob;

/// A path to walk, this should always be a valid path as there is no validity-checking when moving an entity based on this path
/// The first element is the next piece, last is the goal
struct Path(Vec<GridPosition>);

#[derive(Debug)]
struct Levels {
    levels: Vec<level::Level>,
    current: usize,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Game".to_string(),
            width: 800f32,
            height: 600f32,
            vsync: false,
            resizable: true,
            resize_constraints: bevy::window::WindowResizeConstraints {
                min_width: 800f32,
                max_width: 800f32,
                min_height: 600f32,
                max_height: 600f32,
            },
            scale_factor_override: None,
            mode: bevy::window::WindowMode::Windowed,
            cursor_locked: false,
            cursor_visible: true,
            decorations: true,
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(LevelBuilder::square(7))
        .insert_resource(Levels {
            levels: Vec::<level::Level>::new(),
            current: 0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MovementPlugin)
        .add_plugin(FPSPlugin)
        .add_plugin(MousePlugin)
        .add_startup_system(build_level.system())
        // .add_startup_system(set_icon.system())
        .add_startup_system(setup.system())
        .add_system(update_camera.system())
        .run();
}

fn build_level(
    mut commands: Commands,
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
        .add_tile(
            level::TileType::Wall,
            materials.add(asset_server.load("tiles/wall.png").into()),
        )
        .build()
        .unwrap();

    for rows in level.tiles() {
        for piece in rows {
            commands
                .spawn_bundle(SpriteBundle {
                    material: level.get_texture(piece.gridx() as usize, piece.gridy() as usize),
                    transform: Transform {
                        translation: Vec3::new(piece.x() as f32, piece.y() as f32, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(GridPosition {
                    x: piece.gridx(),
                    y: piece.gridy(),
                })
                .insert(Tile);
        }
    }

    // How to spawn a "overlay"
    // commands.spawn(SpriteBundle {
    //     material: materials.add(Color::rgba(0.2, 0.66, 0.70, 0.3).into()),
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 0.0, 2.0),
    //         ..Default::default()
    //     },
    //     sprite: Sprite::new(Vec2::new(32.0, 32.0)),
    //     ..Default::default()
    // });

    levels.levels.push(level);
}

fn update_camera(
    // mut query_camera: Query<(&bevy::render::camera::Camera, &mut Transform)>,
    // query_player: Query<&Transform, With<Player>>,
    mut query: QuerySet<(
        Query<(&bevy::render::camera::Camera, &mut Transform)>,
        Query<(&mut Transform, With<Player>)>,
    )>,
) {
    // Can't borrow q at the same time, so need to remember values
    let mut new_x = 0.0;
    let mut new_y = 0.0;

    // No idea what the second value means, maybe if With<Player> is satisifed?
    for (ply_pos, _) in query.q1_mut().iter_mut() {
        new_x = ply_pos.translation.x;
        new_y = ply_pos.translation.y;
    }
    for (camera, mut transform) in query.q0_mut().iter_mut() {
        if camera.name == Some(String::from("Camera2d")) {
            transform.translation.x = new_x;
            transform.translation.y = new_y;
        }
    }
}

// NOTE(erlend):
// systems that access Resources run on the main thread
// and winit_window.set_window_icon hangs(deadlock?) when it
// runs from a different thread...
// fn set_icon(_: &mut World, resources: &mut Resources) {
//     let winit_windows = resources.get::<WinitWindows>().unwrap();
//     let windows = resources.get::<Windows>().unwrap();

//     let img = image::open("assets/logo/logo.png").unwrap();

//     if let Some(window) = windows.get_primary() {
//         if let Some(winit_window) = winit_windows.get_window(window.id()) {
//             winit_window.set_window_icon(Some(
//                 winit::window::Icon::from_rgba(img.to_bytes(), 32, 32)
//                     .expect("Failed to create icon"), //Error handling? No.
//             ));
//         }
//     }
// }

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_char = asset_server.load("chars/new_juniper.png");

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));
    commands.spawn_bundle(camera);

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture_char.into()),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(GridPosition { x: 0, y: 0 })
        .insert(Path(Vec::<GridPosition>::new()));

    let texture_char = asset_server.load("chars/blob.png");

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture_char.into()),
            transform: Transform {
                translation: Vec3::new(32.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GridPosition { x: 1, y: 0 })
        .insert(Blob);
}
