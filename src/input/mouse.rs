use bevy::{math::Vec2, prelude::*, window::Windows};

use crate::level::{Point, TILE_SIZE};

use super::mouse_update_grid;

pub struct CurrentMousePosition(Option<Point>);

impl CurrentMousePosition {
    pub fn position(&self) -> Option<Point> {
        self.0
    }
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentMousePosition(None))
            .add_system(set_current_mouse_position)
            .add_system(mouse_update_grid);
    }
}

fn set_current_mouse_position(
    windows: Res<Windows>,
    mut camera_query: Query<(&bevy::render::camera::Camera, &Transform)>,
    mut current_position: ResMut<CurrentMousePosition>,
) {
    current_position.0 = None;

    if let Some(window) = windows.get_primary() {
        if let Some(phys) = window.physical_cursor_position() {
            for (cam, cam_trans) in camera_query.iter_mut() {
                if cam.name == Some(String::from("camera_2d")) {
                    let mut mouse_pos = Vec2::new(
                        (phys.x / window.scale_factor()) as f32,
                        (phys.y / window.scale_factor()) as f32,
                    );

                    mouse_pos -= Vec2::new(window.width() / 2., window.height() / 2.);
                    mouse_pos -= Vec2::splat(TILE_SIZE / 2.);

                    let desiredx = ((mouse_pos.x + cam_trans.translation.x) / TILE_SIZE).ceil();
                    let desiredy = ((mouse_pos.y + cam_trans.translation.y) / TILE_SIZE).ceil();

                    let goal = Point {
                        x: desiredx as i32,
                        y: desiredy as i32,
                    };

                    current_position.0 = Some(goal);
                }
            }
        }
    }
}
