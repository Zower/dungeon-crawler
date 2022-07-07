// #![windows_subsystem = "windows"]
//! A to be failed attempt at a 2D pixel dungeon-crawler

mod entity;
mod input;
mod level;
mod logic;
mod render;
mod ui;

use std::time::Duration;

use bevy_easings::{Ease, EaseFunction, EasingComponent, EasingType, EasingsPlugin};
use entity::*;
use input::*;
use level::{
    FieldOfView, FovPlugin, Map, MapBuilder, Point, Size, Surface, TileComponent, WalkPath,
    TILE_SIZE,
};
use logic::{CollisionPlugin, Cursor, MovementPlugin, PlayerHoveredPlugin, SpellPlugin};
use render::RenderPlugin;
use ui::*;

use bevy::prelude::*;

/// Holds all the maps currently generated. The 0th element is the starting level, and as the player descends the index increases.
/// TODO: Consider rewrite as Entity with maps being children
#[derive(Debug)]
pub struct Level {
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
                min_width: 200f32,
                min_height: 200f32,
                ..Default::default()
            },
            scale_factor_override: None,
            mode: bevy::window::WindowMode::Windowed,
            cursor_locked: false,
            cursor_visible: true,
            decorations: true,
        })
        .insert_resource(ClearColor(Color::rgb(0.15, 0.1, 0.15)))
        // .add_system_set(SystemSet::new().label("test").)
        .add_plugin(EasingsPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(KeyboardMovementPlugin)
        .add_plugin(MousePlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(ConvarPlugin)
        .add_plugin(FovPlugin)
        .add_plugin(PlayerHoveredPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(SpellPlugin)
        .add_startup_system(setup)
        // .add_system(easing_q)
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

// fn easing_q(
//     mut commands: Commands,
//     // mut query: Query<(Entity, &mut Sprite), With<Test>>,
//     mut query: Query<(Entity, &mut Style, &Children), With<Test>>,
//     mut button_query: Query<&Interaction, With<Button>>,
//     input: Res<Input<KeyCode>>,
// ) {
//     let (ent, style, children) = query.single_mut();
//     let button = button_query.get_mut(children[0]);
//     if let Ok(button) = button {
//         match *button {
//             Interaction::None => {
//                 commands.entity(ent).remove::<EasingComponent<Style>>();
//                 commands.entity(ent).insert(style.clone().ease_to(
//                     Style {
//                         size: bevy::prelude::Size::new(Val::Px(64.), Val::Px(96.)),
//                         position: Rect {
//                             left: Val::Percent(50.),
//                             bottom: Val::Percent(5.0),
//                             ..Default::default()
//                         },
//                         ..Default::default()
//                     },
//                     EaseFunction::ExponentialOut,
//                     EasingType::Once {
//                         duration: Duration::from_millis(400),
//                     },
//                 ));
//                 return;
//             }
//             _ => (),
//         };
//         info!("{button:?}");
//         commands.entity(ent).remove::<EasingComponent<Style>>();
//         commands.entity(ent).insert(style.clone().ease_to(
//             Style {
//                 size: bevy::prelude::Size::new(Val::Px(80.), Val::Px(120.)),
//                 position: Rect {
//                     left: Val::Percent(50.),
//                     bottom: Val::Percent(12.5),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//             EaseFunction::ExponentialOut,
//             EasingType::Once {
//                 duration: Duration::from_millis(400),
//             },
//         ));
//     } else {
//     }
//     // // info!("{entity:?}");
//     if input.just_pressed(KeyCode::X) {
//         commands.entity(ent).remove::<EasingComponent<Style>>();
//         commands.entity(ent).insert(style.clone().ease_to(
//             Style {
//                 size: bevy::prelude::Size::new(Val::Px(80.), Val::Px(120.)),
//                 position: Rect {
//                     left: Val::Percent(50.),
//                     bottom: Val::Percent(12.5),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//             EaseFunction::ExponentialOut,
//             EasingType::Once {
//                 duration: Duration::from_millis(400),
//             },
//         ));
//     } else if input.just_pressed(KeyCode::O) {
//         commands.entity(ent).remove::<EasingComponent<Style>>();
//         commands.entity(ent).insert(style.clone().ease_to(
//             Style {
//                 size: bevy::prelude::Size::new(Val::Px(64.), Val::Px(96.)),
//                 position: Rect {
//                     left: Val::Percent(50.),
//                     bottom: Val::Percent(5.0),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//             EaseFunction::ExponentialOut,
//             EasingType::Once {
//                 duration: Duration::from_millis(400),
//             },
//         ));
//     }
// }

#[derive(Component)]
struct Test;

/// Set up for the initial game state
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let safe_player_pos = build_and_insert_map(&mut commands, &asset_server);

    let font = asset_server.load("fonts/PublicPixel.ttf");

    // commands
    //     .spawn_bundle(NodeBundle {
    //         style: Style {
    //             size: bevy::prelude::Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    //             justify_content: JustifyContent::SpaceBetween,
    //             ..Default::default()
    //         },
    //         color: Color::NONE.into(),
    //         ..Default::default()
    //     })
    //     .with_children(|parent| {
    //         parent
    //             .spawn_bundle(ImageBundle {
    //                 style: Style {
    //                     position_type: PositionType::Absolute,
    //                     position: bevy::prelude::Rect {
    //                         left: Val::Percent(50.),
    //                         bottom: Val::Percent(5.),
    //                         ..Default::default()
    //                     },
    //                     size: bevy::prelude::Size::new(Val::Px(64.), Val::Px(96.)),
    //                     ..Default::default()
    //                 },
    //                 image: asset_server.load("cards/fireball.png").into(),
    //                 ..Default::default()
    //             })
    //             .insert(Test)
    //             .with_children(|parent| {
    //                 parent.spawn_bundle(ButtonBundle {
    //                     style: Style {
    //                         size: bevy::prelude::Size::new(Val::Percent(100.), Val::Percent(150.)),
    //                         // center button
    //                         margin: Rect::all(Val::Auto),
    //                         justify_content: JustifyContent::Center,
    //                         align_items: AlignItems::Center,
    //                         ..Default::default()
    //                     },
    //                     color: Color::NONE.into(),
    //                     ..Default::default()
    //                 });
    //             });
    //     });

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
        .insert(Cursor::new())
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
