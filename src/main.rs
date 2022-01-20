// #![windows_subsystem = "windows"]
//! A to be failed attempt at a 2D pixel dungeon-crawler

mod entity;
mod input;
mod level;
mod logic;
mod ui;

use dungeon_crawler_derive::Convar;
use entity::*;
use input::*;
use level::{
    set_visible, Map, MapBuilder, Point, Size, Surface, TileComponent, ViewedState, WalkPath,
    TILE_SIZE,
};
use logic::{CameraPlugin, CollisionPlugin, MovementPlugin};
use ui::*;

use bevy::prelude::*;

#[derive(Debug, Default, Convar)]
struct GlobalVision(bool);

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
        .insert_resource(ClearColor(Color::rgb(0.52, 0.149, 0.3412)))
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
        .add_plugin(CameraPlugin)
        // .add_startup_system(set_icon)
        .add_startup_system(setup)
        .add_system(test_fov)
        .add_convar_default::<GlobalVision>()
        .run();
}

fn test_fov(
    global: Res<GlobalVision>,
    mut level: ResMut<Level>,
    player: Query<&Point, With<Player>>,
    mut map_sprites: Query<(&mut Sprite, &mut Visibility, &TileComponent)>,
) {
    let map = level.get_current_mut();

    let player_pos = player.get_single().unwrap();
    set_visible(map, *player_pos);

    for (mut sprite, mut visibility, pos) in map_sprites.iter_mut() {
        if global.0 {
            visibility.is_visible = true;
        } else {
            match map.get_tile(&pos.0).unwrap().revealed {
                ViewedState::NotViewed => {
                    visibility.is_visible = false;
                }
                ViewedState::InView => {
                    visibility.is_visible = true;
                    sprite.color = Color::WHITE;
                }
                ViewedState::PreviouslyViewed => {
                    visibility.is_visible = true;
                    sprite.color = Color::GRAY;
                }
            }
        }
    }
}

fn build_and_insert_map(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Point {
    let floor = asset_server.load("tiles/Purple_floor.png");
    let wall = asset_server.load("tiles/Purple_wall.png");

    let mut map_builder = MapBuilder::new();
    let (map, rooms) = map_builder
        .depth(0)
        .size(Size::splat(95))
        .room_size(3..8, 3..8)
        .nr_rooms(50)
        .build();
    // Spawns the tiles sprites, this is never used for any logic, they are just drawn on the screen.
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
