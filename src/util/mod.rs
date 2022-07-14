//! Modules for the game UI.

mod consts;
mod queries;

use bevy::math::Vec2;
use bevy_ecs_tilemap::TilePos;
pub use consts::*;
pub use queries::*;

pub fn tile_from_trans(translation: &Vec2) -> TilePos {
    TilePos(
        (translation.x / TILE_SIZE) as u32,
        (translation.y / TILE_SIZE) as u32,
    )
}

pub fn trans_from_tile(pos: &TilePos) -> Vec2 {
    Vec2::new(
        (pos.0 as f32 * TILE_SIZE) + HALF_TILE_SIZE,
        (pos.1 as f32 * TILE_SIZE) + HALF_TILE_SIZE,
    )
}
