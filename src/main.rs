// #![windows_subsystem = "windows"]
//! A to be failed attempt at a 2D pixel dungeon-crawler

mod entity;
mod input;
mod level;
mod logic;
mod ui;

use entity::{Blob, Player};
use input::*;
use level::{LevelBuilder, Path, Point, Size, TileComponent};
use logic::MovementPlugin;
use ui::*;

use bevy::{input::keyboard::KeyboardInput, prelude::*, winit::WinitWindows};

/// Holds all the levels currently generated. The 0th element is the starting level, and as the player descends the index increases.
#[derive(Debug)]
struct Levels {
    levels: Vec<level::Level>,
    /// Holds the index of the current level.
    /// Could be a reference to a level instead? Avoid levels.levels.get[levels.current], unsure if this adds unnescessary complexity.
    current: Option<usize>,
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
        .insert_resource(LevelBuilder::new(Size {
            width: 7,
            height: 7,
        }))
        .insert_resource(Levels {
            levels: Vec::<level::Level>::new(),
            current: None,
        })
        .insert_resource(ConvarStore::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(KeyboardMovementPlugin)
        .add_plugin(MouseMovementPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(ConsolePlugin)
        .add_plugin(FPSPlugin)
        .add_startup_system(build_level.system())
        // .add_startup_system(set_icon.system())
        .add_startup_system(setup.system())
        .add_startup_system(convar_setup.system())
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
    let current = 0;
    levels.current = Some(current);

    let level = level_builder
        .add_tile(
            level::Surface::Floor,
            materials.add(asset_server.load("tiles/floor.png").into()),
        )
        .add_tile(
            level::Surface::Wall,
            materials.add(asset_server.load("tiles/wall.png").into()),
        )
        .build(current)
        .unwrap();

    // Spawns the tiles sprites, this is never used for any logic, they are just drawn on the screen.
    for row in 0..level.size.width {
        for column in 0..level.size.height {
            let tile = level.get_tile(Point { x: row, y: column }).unwrap();
            let screen_position = tile.screen_position();
            commands
                .spawn_bundle(SpriteBundle {
                    material: tile.tile_type.texture.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            screen_position.0.x as f32,
                            screen_position.0.y as f32,
                            0f32,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(TileComponent);
        }
    }

    levels.levels.push(level);
}

fn update_camera(
    mut query: QuerySet<(
        Query<(&bevy::render::camera::Camera, &mut Transform)>,
        Query<(&Transform, With<Player>)>,
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

    // Create the player entity
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture_char.into()),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            current: Point { x: 0, y: 0 },
            path: Path(Vec::<Point>::new()),
        });
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
        .insert(Blob {
            current: Point { x: 0, y: 1 },
        });
}

fn convar_setup(mut store: ResMut<ConvarStore>) {
    store.add("fps", Var::Toggleable(0));
}
