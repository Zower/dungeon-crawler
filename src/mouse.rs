//! Handles mouse input

use bevy::{input::mouse::MouseButtonInput, prelude::*, window::CursorMoved};

use crate::{level::TILE_SIZE, movement::a_star, GridPosition, Levels, Path, Player};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(LatestMove(Vec2::new(0.0, 0.0)))
            .add_system(mouse_update_grid.system());
    }
}
#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

struct LatestMove(Vec2);

fn mouse_update_grid(
    windows: Res<Windows>,
    levels: Res<Levels>,
    mut latest: ResMut<LatestMove>,
    mut state: Local<State>,
    mut query_cam: Query<(&bevy::render::camera::Camera, &Transform)>,
    mut query: Query<(&GridPosition, &mut Path), With<Player>>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    if let Some(window) = windows.get_primary() {
        for event in state
            .mouse_button_event_reader
            .iter(&mouse_button_input_events)
        {
            if event.state == bevy::input::ElementState::Pressed {
                for (grid_pos, mut path) in query.iter_mut() {
                    // Players
                    for (cam, trans_cam) in query_cam.iter_mut() {
                        // The camera
                        if cam.name == Some(String::from("Camera2d")) {
                            //           (mouse pos   + camera's current position) / tilesize rounded
                            let desiredx = ((latest.0[0] + trans_cam.translation.x)
                                / TILE_SIZE as f32)
                                .round() as i32;
                            let desiredy = ((latest.0[1] + trans_cam.translation.y)
                                / TILE_SIZE as f32)
                                .round() as i32;

                            let level = levels.0.last().unwrap();

                            let goal = GridPosition {
                                x: desiredx,
                                y: desiredy,
                            };

                            if level.in_bounds(&goal) {
                                path.0 = a_star(
                                    level,
                                    GridPosition {
                                        // A bit messy, hehe
                                        x: grid_pos.x(),
                                        y: grid_pos.y(),
                                    },
                                    goal,
                                );
                            }
                        }
                    }
                }
            }
        }

        let origin = Vec2::new(-(window.width() / 2.0), -(window.height() / 2.0));

        for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
            // Jank way to translate window coordinates into game coordinates
            latest.0[0] = event.position[0] + origin[0];
            latest.0[1] = event.position[1] + origin[1];
        }
    }
}
