use bevy::{
    math::Vec2,
    prelude::*,
    render::camera::{Camera2d, RenderTarget},
    window::Windows,
};
use bevy_ecs_tilemap::TilePos;

use crate::util::TILE_SIZE;

#[derive(Debug, Deref, DerefMut)]
pub struct CurrentMousePosition(Option<TilePos>);

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentMousePosition(None))
            .add_system(set_current_mouse_position);
        // .add_system(mouse_update_grid);
    }
}

fn set_current_mouse_position(
    mut current_position: ResMut<CurrentMousePosition>,
    // need to get window dimensions
    windows: Res<Windows>,
    // query to get camera transform
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    current_position.0 = None;
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_query.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width(), wnd.height());

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        let x = ((world_pos.x / TILE_SIZE).ceil() as u32).checked_sub(1);
        let y = ((world_pos.y / TILE_SIZE).ceil() as u32).checked_sub(1);

        if let Some(x) = x {
            if let Some(y) = y {
                **current_position = Some(TilePos(x, y));
            }
        }
    }
}
