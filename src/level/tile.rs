/// Various Tile related structures
use super::common::{Point, ScreenPoint};
use bevy::prelude::{ColorMaterial, Handle};

pub const TILE_SIZE: i32 = 32;
///One tile on the grid
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Tile {
    pub tile_type: TileType,
    /// The piece's Position on the board, necessary?
    pub position: Point,
    pub screen_position: ScreenPoint,
    /// Currently always 1, potentially some different terrain etc.
    pub cost: i32,
}

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
    /// Get the world position of this tile
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
