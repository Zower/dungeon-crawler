//! Mouse movement.

use bevy::{input::mouse::MouseButtonInput, prelude::*};

use crate::{
    level::{Level, Point, WalkPath, TILE_SIZE},
    Levels, Player,
};

/// Handles moving the player with the mouse
pub struct MouseMovementPlugin;

impl Plugin for MouseMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LatestMousePosition(Vec2::new(0.0, 0.0)))
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
    levels: Res<Levels>,
    windows: Res<Windows>,
) {
    for event in mouse_button_input_events.iter() {
        if event.state == bevy::input::ElementState::Pressed {
            if let Some(window) = windows.get_primary() {
                let (mut player_path, player_position) = player_query.single_mut();
                for (cam, cam_trans) in camera_query.iter_mut() {
                    if cam.name == Some(String::from("camera_2d")) {
                        let phys = window.physical_cursor_position().unwrap();

                        let mut mouse_pos = Vec2::new(
                            (phys.x / window.scale_factor()) as f32,
                            (phys.y / window.scale_factor()) as f32,
                        );

                        mouse_pos -= Vec2::new(window.width() / 2., window.height() / 2.);
                        mouse_pos += Vec2::splat(TILE_SIZE / 2.);

                        let desiredx = ((mouse_pos.x + cam_trans.translation.x) / TILE_SIZE).ceil();
                        let desiredy = ((mouse_pos.y + cam_trans.translation.y) / TILE_SIZE).ceil();

                        let goal = Point {
                            x: desiredx as i32,
                            y: desiredy as i32,
                        };

                        debug!("Registered request to move to x:{desiredx}, y:{desiredy}");

                        let level = levels.current();
                        if level.in_bounds(goal) {
                            let goal = level.get_tile(goal).unwrap();
                            if goal.is_safe() {
                                player_path.0 =
                                    Level::a_star(&level, *player_position, goal.position);
                            }
                        }
                    }
                }
            }
        }
    }
}
