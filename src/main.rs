// #![windows_subsystem = "windows"]
//! A to be failed attempt at a 2D pixel dungeon-crawler

mod entity;
mod input;
mod level;
mod logic;
mod ui;

use entity::*;
use input::*;
use level::{Map, MapBuilder, Point, Size, Surface, TileComponent, WalkPath, TILE_SIZE};
use logic::{CollisionPlugin, MovementPlugin};
use rand::Rng;
use ui::*;

use bevy::prelude::*;

/// Holds all the maps currently generated. The 0th element is the starting level, and as the player descends the index increases.
#[derive(Debug)]
struct Level {
    maps: Vec<Map>,
    current_map: usize,
}

impl Level {
    fn new(map: Map) -> Self {
        Self {
            maps: vec![map],
            current_map: 0,
        }
    }

    fn get_current(&self) -> &Map {
        &self.maps[self.current_map]
    }

    fn push(&mut self, map: Map) {
        self.maps.push(map);
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game".to_string(),
            width: 800f32,
            height: 600f32,
            vsync: false,
            resizable: true,
            transparent: false,
            position: None,
            resize_constraints: bevy::window::WindowResizeConstraints {
                min_width: 50f32,
                max_width: 1920f32,
                min_height: 50f32,
                max_height: 1080f32,
            },
            scale_factor_override: None,
            mode: bevy::window::WindowMode::Windowed,
            cursor_locked: false,
            cursor_visible: true,
            decorations: true,
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        // .insert_resource(SpriteSettings {
        //     frustum_culling_enabled: true,
        // })
        .add_plugins(DefaultPlugins)
        // .add_plugin(EnemyPlugin)
        .add_plugin(KeyboardMovementPlugin)
        .add_plugin(MouseMovementPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(ConvarPlugin)
        // .add_startup_system(set_icon)
        .add_startup_system(setup)
        .add_system(update_camera)
        .run();
}

fn build_and_insert_map(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Point {
    let floor = asset_server.load("tiles/floor.png");
    let wall = asset_server.load("tiles/wall.png");

    let mut map_builder = MapBuilder::new();
    let (map, rooms) = map_builder
        .depth(0)
        .size(Size::splat(50))
        .room_size(3..5, 3..5)
        .nr_rooms(30)
        .build();

    // Spawns the tiles sprites, this is never used for any logic, they are just drawn on the screen.
    for row in 0..map.size.width {
        for column in 0..map.size.height {
            let tile = map.get_tile(Point { x: row, y: column }).unwrap();
            let screen_position = tile.screen_position();
            commands
                .spawn_bundle(SpriteBundle {
                    texture: if tile.surface == Surface::Wall {
                        wall.clone()
                    } else {
                        floor.clone()
                    },
                    // material: tile.tile_type.texture.clone(),
                    transform: Transform {
                        translation: Vec3::new(screen_position.x, screen_position.y, 0f32),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(TileComponent);
        }
    }

    commands.insert_resource(Level::new(map));

    rooms[0].center()
}

fn update_camera(
    mut query: QuerySet<(
        QueryState<&Transform, With<Player>>,
        QueryState<(&bevy::render::camera::Camera, &mut Transform)>,
    )>,
) {
    // Can't borrow q at the same time, so need to remember values
    let mut new_x = 0.0;
    let mut new_y = 0.0;

    let mut q0 = query.q0();
    // No idea what the second value means, maybe if With<Player> is satisifed?
    let ply_pos = q0.single_mut();

    new_x = ply_pos.translation.x;
    new_y = ply_pos.translation.y;

    let mut q1 = query.q1();
    for (camera, mut transform) in q1.iter_mut() {
        if camera.name == Some(String::from("camera_2d")) {
            transform.translation.x = new_x;
            transform.translation.y = new_y;
        }
    }
}

// Currently broken after bevy 5.0
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

/// Set up for the initial game state
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let safe_player_pos = build_and_insert_map(&mut commands, &asset_server);

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));
    commands.spawn_bundle(camera);

    commands.spawn_bundle(UiCameraBundle::default());

    // let texture_char = asset_server.load("chars/new_juniper.png");
    let font = asset_server.load("fonts/PublicPixel.ttf");

    // Create the player entity
    commands
        .spawn_bundle(SpriteBundle {
            // material: materials.add(texture_char.into()),
            texture: asset_server.load("chars/new_juniper.png"),
            transform: Transform {
                translation: Vec3::new(
                    safe_player_pos.x as f32 * TILE_SIZE,
                    safe_player_pos.y as f32 * TILE_SIZE,
                    1.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(safe_player_pos)
        .insert(WalkPath(Vec::<Point>::new()))
        .insert(Health(100));

    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "100",
                TextStyle {
                    font,
                    font_size: 30.0,
                    color: Color::DARK_GREEN,
                },
                TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            style: Style {
                position: Rect {
                    bottom: Val::Percent(5.),
                    left: Val::Percent(5.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HealthText);

    // Spawn the "Blob"
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("chars/blob.png"),
            // material: materials.add(texture_char.into()),
            transform: Transform {
                translation: Vec3::new(64.0, 32.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Blob)
        .insert(safe_player_pos);
}
