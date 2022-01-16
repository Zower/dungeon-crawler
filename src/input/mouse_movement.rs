//! Mouse movement.

use bevy::{input::mouse::MouseButtonInput, prelude::*, window::CursorMoved};

use crate::{
    level::{Level, Point, WalkPath, TILE_SIZE},
    Levels, Player,
};

/// Handles moving the player with the mouse
pub struct MouseMovementPlugin;

impl Plugin for MouseMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LatestMousePosition(Vec2::new(0.0, 0.0)))
            .add_system(mouse_update_position)
            .add_system(mouse_update_grid);
    }
}

/// The latest known mouse position of the player
struct LatestMousePosition(Vec2);

/// Checks if the player pressed mouse1
/// If they did, calculate what grid they pressed, then use A* to find the path there.
/// Then set the players desired path equal to the A* calculation.
fn mouse_update_grid(
    mut camera_query: Query<(&bevy::render::camera::Camera, &Transform)>,
    mut player_query: Query<(&mut WalkPath, &Point), With<Player>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    latest: Res<LatestMousePosition>,
    levels: Res<Levels>,
) {
    for event in mouse_button_input_events.iter() {
        if event.state == bevy::input::ElementState::Pressed {
            println!("1");
            let (mut player_path, player_position) = player_query.single_mut();
            for (cam, cam_trans) in camera_query.iter_mut() {
                println!("{cam:?}");
                if cam.name == Some(String::from("camera_2d")) {
                    println!("2");
                    //                 (mouse pos  + camera current position)  / tilesize
                    let desiredx = (latest.0.x + cam_trans.translation.x) / TILE_SIZE as f32;
                    let desiredy = (latest.0.y + cam_trans.translation.y) / TILE_SIZE as f32;
                    let goal = Point {
                        x: desiredx as i32,
                        y: desiredy as i32,
                    };

                    let level = levels.current();
                    if level.in_bounds(goal) {
                        println!("3");
                        let goal = level.get_tile(goal).unwrap();
                        if goal.is_safe() {
                            println!("4");
                            player_path.0 = Level::a_star(&level, *player_position, goal.position);
                            println!("{player_path:?}");
                        }
                    }
                }
            }
        }
    }
}

/// Sets the value of LatestMousePosition to the latest hovered in-game coordinate
fn mouse_update_position(
    mut latest: ResMut<LatestMousePosition>,
    windows: Res<Windows>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    if let Some(window) = windows.get_primary() {
        let origin = Vec2::new(
            -(window.width() / 2.0) + (TILE_SIZE / 2.0) as f32,
            -(window.height() / 2.0) + (TILE_SIZE / 2.0) as f32,
        );
        for event in cursor_moved_events.iter() {
            latest.0 = event.position + origin;
        }
    }
}
