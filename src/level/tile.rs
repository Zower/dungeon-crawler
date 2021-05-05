pub const TILE_SIZE: i32 = 32;

use super::common::{Point, ScreenPoint};
use bevy::prelude::{ColorMaterial, Handle};
///One tile on the grid
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Tile {
    pub tile_type: TileType,
    /// The piece's Position on the board, necessary?
    pub position: Point,
    pub screen_position: ScreenPoint,
    // X coordinate
    // x: i32,
    // Y coordinate
    // y: i32,
    /// Currently always 1, potentially some different terrain etc.
    pub cost: i32,
}
// Component used by tiles
// pub struct Tile;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Surface {
    Floor,
    Wall,
}
/// Contains the texture to be used by bevy, and a type
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TileType {
    pub texture: Handle<ColorMaterial>,
    pub surface: Surface,
}

impl Tile {
    /// Calculated the world position of this tile
    pub fn screen_position(&self) -> ScreenPoint {
        self.screen_position
    }

    pub fn is_safe(&self) -> bool {
        !self.is_wall() // && others...
    }

    pub fn is_wall(&self) -> bool {
        self.tile_type.surface == Surface::Wall
    }
}

/// A component that can be used to identify tiles in the ECS.
pub struct TileComponent;
