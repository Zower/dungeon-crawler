use bevy::{input::mouse::MouseButtonInput, prelude::*, window::CursorMoved};

use crate::{level::TILE_SIZE, GridPosition, Player};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(LatestMove(Vec2::new(0.0, 0.0)))
            .add_system(print_mouse_events_system.system());
    }
}
#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

struct LatestMove(Vec2);

/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    windows: Res<Windows>,
    mut latest: ResMut<LatestMove>,
    mut state: Local<State>,
    mut query_cam: Query<(&bevy::render::camera::Camera, &Transform)>,
    mut query: Query<&mut GridPosition, With<Player>>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    if let Some(window) = windows.get_primary() {
        let origin = Vec2::new(-(window.width() / 2.0), -(window.height() / 2.0));

        //println!("{:?}", origin);

        for event in state
            .mouse_button_event_reader
            .iter(&mouse_button_input_events)
        {
            if event.state == bevy::input::ElementState::Pressed {
                for mut grid_pos in query.iter_mut() {
                    // Players
                    for (cam, trans_cam) in query_cam.iter_mut() {
                        // The camera
                        if cam.name == Some(String::from("Camera2d")) {
                            //    mouse pos   + camera's current position / tilesize rounded
                            grid_pos.x = ((latest.0[0] + trans_cam.translation.x) / TILE_SIZE)
                                .round() as i32;
                            grid_pos.y = ((latest.0[1] + trans_cam.translation.y) / TILE_SIZE)
                                .round() as i32;
                        }
                    }
                    //println!("Clicked: {:?}", latest.0)
                }
            }
        }

        for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
            // Jank way to translate window coordinates into game coordinates
            //println!("{:?}", event);
            latest.0[0] = event.position[0] + origin[0];
            latest.0[1] = event.position[1] + origin[1];
        }
    }
}
