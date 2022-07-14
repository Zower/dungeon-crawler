// #![windows_subsystem = "windows"]
////! A to be failed attempt at a 2D pixel dungeon-crawler

// #![deny(warnings)]
#![deny(
    missing_debug_implementations,
    // missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    // unused_results
)]

mod debug;
mod entity;
mod input;
mod level;
mod logic;
mod render;
mod tilemap;
mod ui;
mod util;

use bevy::{prelude::*, window::PresentMode};
use bevy_console::ConsoleOpen;
use bevy_easings::EasingsPlugin;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;
use entity::*;
use input::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};
use level::{FieldOfView, FovPlugin};
use logic::{MovementAction, MovementPlugin, PlayerHoveredPlugin, SpellPlugin, TileCursor};
use render::RenderPlugin;
use tilemap::Rect2;
use ui::*;
use util::TILE_SIZE;

use crate::debug::DebugPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActiveState {
    /// Console is open.
    Paused,
    /// Console closed. All logic may consider the game active.
    Playing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    /// Map is being generated.
    GeneratingMap,
    /// Player is walking around freely, taking as many actions as they like
    FreeRoam,
    /// The game is running in a turn based state (combat).
    TurnBased,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
// pub enum GameStage {
//     Input,
//     Logic,
//     Render,
// }

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Game".to_string(),
        width: 800f32,
        height: 600f32,
        present_mode: PresentMode::Immediate,
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
    .insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..Default::default()
    })
    // .add_stage_before(CoreStage::Update, GameStage::Input, SystemStage::parallel())
    // .add_stage_after(GameStage::Input, GameStage::Logic, SystemStage::parallel())
    // .add_stage_after(GameStage::Logic, GameStage::Render, SystemStage::parallel())
    .add_loopless_state(ActiveState::Paused)
    .add_loopless_state(GameState::GeneratingMap)
    .insert_resource(ClearColor(Color::rgb(0.15, 0.1, 0.15)))
    // .add_plugins(RetroPlugins::default())
    .add_plugins(DefaultPlugins)
    .add_plugin(TilemapPlugin)
    .add_plugin(EasingsPlugin)
    .add_plugin(MousePlugin)
    .add_plugin(MovementPlugin)
    .add_plugin(UiPlugin)
    .add_plugin(FovPlugin)
    .add_plugin(PlayerHoveredPlugin)
    .add_plugin(RenderPlugin)
    .add_plugin(SpellPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.))
    .add_plugin(RapierDebugRenderPlugin::default())
    // todo disable features
    .add_exit_system(GameState::GeneratingMap, setup_player)
    .add_system(update_state);

    #[cfg(debug_assertions)]
    app.add_plugin(DebugPlugin);

    app.run();
}

/// Set up for the initial game state
fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, rooms: Query<&Rect2>) {
    // let safe_player_pos = build_and_insert_map(&mut commands, &asset_server);

    let font = asset_server.load("fonts/PublicPixel.ttf");

    let room = rooms.single().center();

    // Create the player entity
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("chars/player.png"),
            transform: Transform {
                translation: Vec3::new(room.x * TILE_SIZE, room.y * TILE_SIZE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(PassiveTilePos(TilePos(room.x as u32, room.y as u32)))
        .insert(FieldOfView::new(6))
        .insert(TileCursor::new())
        .insert(Health(100))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Friction::new(0.))
        .insert(Collider::ball(7.5))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(InputManagerBundle::<MovementAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::F, MovementAction::Up),
                (KeyCode::R, MovementAction::Left),
                (KeyCode::S, MovementAction::Down),
                (KeyCode::T, MovementAction::Right),
            ]),
        });

    // TOOD: should be in ui
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
                translation: Vec3::new(48.0, 16.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Blob)
        .insert(TilePos(0, 0));
}

fn update_state(
    mut commands: Commands,
    console_open: Res<ConsoleOpen>,
    state: Res<CurrentState<ActiveState>>,
) {
    if console_open.is_changed() && !console_open.is_added() {
        let new_state = if console_open.open {
            ActiveState::Paused
        } else {
            ActiveState::Playing
        };

        info!("State update {:?} -> {:?}", state.0, new_state);
        commands.insert_resource(NextState(new_state));
    }
}

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
