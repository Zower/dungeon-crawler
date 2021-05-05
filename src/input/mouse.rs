//! Handles mouse input

use bevy::{input::mouse::MouseButtonInput, prelude::*, window::CursorMoved};

use crate::{
    level::{Level, Point, TILE_SIZE},
    Levels, Player,
};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(LatestMousePosition(Vec2::new(0.0, 0.0)))
            .add_system(mouse_update_position.system())
            .add_system(mouse_update_grid.system());
    }
}

struct LatestMousePosition(Vec2);

fn mouse_update_grid(
    mut query_cam: Query<(&bevy::render::camera::Camera, &Transform)>,
    mut query_player: Query<&mut Player>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    latest: Res<LatestMousePosition>,
    levels: Res<Levels>,
) {
    for event in mouse_button_input_events.iter() {
        if event.state == bevy::input::ElementState::Pressed {
            if let Ok(mut player) = query_player.single_mut() {
                for (cam, cam_trans) in query_cam.iter_mut() {
                    // The camera
                    if cam.name == Some(String::from("Camera2d")) {
                        //                                  (mouse pos   + camera's current position) / tilesize rounded
                        let desiredx = (latest.0.x + cam_trans.translation.x) / TILE_SIZE as f32;
                        let desiredy = (latest.0.y + cam_trans.translation.y) / TILE_SIZE as f32;
                        let goal = Point {
                            x: desiredx as i32,
                            y: desiredy as i32,
                        };

                        if let Some(current_level) = levels.current {
                            let level = &levels.levels[current_level];
                            if level.in_bounds(goal) {
                                let goal = level.get_tile(goal).unwrap();
                                if goal.is_safe() {
                                    player.path.0 =
                                        Level::a_star(&level, player.current, goal.position);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Sets the value of LatestMousePosition to the latest hovered screen coordinate
fn mouse_update_position(
    mut latest: ResMut<LatestMousePosition>,
    windows: Res<Windows>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    if let Some(window) = windows.get_primary() {
        let origin = Vec2::new(-(window.width() / 2.0), -(window.height() / 2.0));
        for event in cursor_moved_events.iter() {
            latest.0 = event.position + origin;
        }
    }
}
