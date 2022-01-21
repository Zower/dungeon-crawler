// #![windows_subsystem = "windows"]
//! A to be failed attempt at a 2D pixel dungeon-crawler

mod entity;
mod input;
mod level;
mod logic;
mod ui;

use entity::*;
use input::*;
use level::{
    FieldOfView, FovPlugin, Map, MapBuilder, Point, Size, Surface, TileComponent, WalkPath,
    TILE_SIZE,
};
use logic::{CameraPlugin, CollisionPlugin, MovementPlugin};
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

    fn get_current_mut(&mut self) -> &mut Map {
        &mut self.maps[self.current_map]
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
        .insert_resource(ClearColor(Color::rgb(0.15, 0.1, 0.15)))
        .add_plugins(DefaultPlugins)
        .add_plugin(KeyboardMovementPlugin)
        .add_plugin(MouseMovementPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(ConvarPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(FovPlugin)
        .add_startup_system(setup)
        .run();
}

fn build_and_insert_map(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Point {
    let floor = asset_server.load("tiles/Purple_floor.png");
    let wall = asset_server.load("tiles/Purple_wall.png");

    let mut map_builder = MapBuilder::new();
    let (map, rooms) = map_builder
        .depth(0)
        .size(Size::splat(50))
        .room_size(6..14, 6..14)
        .nr_rooms(22)
        .build();

    // Spawns the tiles sprites
    for row in 0..map.size.width {
        for column in 0..map.size.height {
            let tile = map.get_tile(&Point { x: row, y: column }).unwrap();
            let screen_position = tile.screen_position();
            commands
                .spawn_bundle(SpriteBundle {
                    visibility: Visibility { is_visible: false },
                    texture: if tile.surface == Surface::Wall {
                        wall.clone()
                    } else {
                        floor.clone()
                    },
                    transform: Transform {
                        translation: Vec3::new(screen_position.x, screen_position.y, 0f32),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(TileComponent(Point::new(row, column)));
        }
    }

    commands.insert_resource(Level::new(map));

    rooms.get(0).map(|r| r.center()).unwrap_or(Point::new(1, 1))
}

/// Set up for the initial game state
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let safe_player_pos = build_and_insert_map(&mut commands, &asset_server);

    let font = asset_server.load("fonts/PublicPixel.ttf");

    // Create the player entity
    commands
        .spawn_bundle(SpriteBundle {
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
        .insert(FieldOfView::new(6))
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
                position_type: PositionType::Absolute,
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
            transform: Transform {
                translation: Vec3::new(64.0, 32.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Blob)
        .insert(safe_player_pos);
}
