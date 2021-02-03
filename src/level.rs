//! Everything related to the levels of the game

use core::panic;
use std::thread;

use rand::{thread_rng, Rng};

use bevy::prelude::*;

use crate::movement::Direction;

pub const TILE_SIZE: i32 = 32;

/// A level to be played on
#[derive(Debug)]
pub struct Level {
    /// A 2D list of level tiles
    grid: Vec<Vec<GridPiece>>,
    size: LevelSize,
}
///One piece on the grid
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GridPiece {
    tile: TileTexture,
    ///Game grid position
    grid_position: GridPosition,
    ///X coordinate
    x: i32,
    ///Y coordinate
    y: i32,
}
/// A position on the game grid
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

/// Component used by tiles
pub struct Tile;

/// Contains the texture to be used by bevy, and a type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TileTexture {
    texture: Handle<ColorMaterial>,
    tile_type: TileType,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TileType {
    Floor,
    Wall,
}

/// Builder structure that creates levels, will eventually hold procedural generation logic.
pub struct LevelBuilder {
    tiles: Vec<TileTexture>,
    size: LevelSize,
}

#[derive(Debug, Copy, Clone)]
/// The size of a level, currently square
//TODO: Should be a size? is the map always square?
pub struct LevelSize {
    pub width: i32,
    pub height: i32,
}

impl LevelBuilder {
    pub fn new(width: i32, height: i32) -> Self {
        LevelBuilder {
            size: LevelSize { width, height },
            ..Default::default()
        }
    }

    pub fn square(size: i32) -> Self {
        LevelBuilder {
            size: LevelSize {
                width: size,
                height: size,
            },
            ..Default::default()
        }
    }

    pub fn set_size(&mut self, size: LevelSize) -> &mut Self {
        self.size = size;
        self
    }

    pub fn add_tile(&mut self, tile_type: TileType, texture: Handle<ColorMaterial>) -> &mut Self {
        self.tiles.push(TileTexture {
            texture: texture,
            tile_type,
        });
        self
    }

    pub fn build(&self) -> Result<Level, EmptyTextureError> {
        let mut grid = Vec::new();

        if self.tiles.len() == 0 {
            return Err(EmptyTextureError); //Custom error type yada yada
        }

        let mut x = 0;
        let mut y = 0;

        let mut rng = rand::thread_rng();

        for gridx in 0..self.size.width {
            let mut grid_depth = Vec::new();
            for gridy in 0..self.size.height {
                let mut tile_type = rng.gen_range(0..11);
                match tile_type {
                    0 => tile_type = 1,
                    _ => tile_type = 0,
                } // Yeah this is jank, just testing walls
                grid_depth.push(GridPiece {
                    tile: TileTexture {
                        texture: self.tiles[tile_type].texture.clone(), // This is temporarily only the first texture
                        tile_type: self.tiles[tile_type].tile_type.clone(), // This is temporarily only the first tile
                    },
                    grid_position: GridPosition { x: gridx, y: gridy },
                    x,
                    y,
                });
                y += TILE_SIZE;
            }
            grid.push(grid_depth);
            x += TILE_SIZE;
            y = 0;
        }

        Ok(Level {
            grid,
            size: self.size.clone(),
        })
    }
}

impl Default for LevelBuilder {
    fn default() -> Self {
        LevelBuilder {
            size: LevelSize::default(),
            tiles: Vec::<TileTexture>::new(),
        }
    }
}

impl Default for LevelSize {
    fn default() -> Self {
        LevelSize {
            width: 10,
            height: 10,
        }
    }
}

impl GridPiece {
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn gridx(&self) -> i32 {
        self.grid_position.x
    }

    pub fn gridy(&self) -> i32 {
        self.grid_position.y
    }
    pub fn tile_type(&self) -> TileType {
        self.tile.tile_type
    }

    pub fn is_wall(&self) -> bool {
        self.tile.tile_type == TileType::Wall
    }
}

impl GridPosition {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

impl Level {
    /// CLones the texture of a certain piece. Panics if out-of-bounds, as this is unexpected behavior.
    pub fn get_texture(&self, gridx: usize, gridy: usize) -> Handle<ColorMaterial> {
        match self.grid.get(gridx) {
            Some(column) => match column.get(gridy) {
                Some(piece) => return piece.tile.texture.clone(),
                None => panic!("Attempt to load texture for non-existing piece"),
            },
            None => panic!("Attempt to load texture for non-existing row"),
        }
    }

    pub fn tiles(&self) -> std::slice::Iter<'_, Vec<GridPiece>> {
        self.grid.iter()
    }

    pub fn is_safe(&self, gridx: usize, gridy: usize) -> bool {
        match self.grid.get(gridx) {
            Some(row) => match row.get(gridy) {
                // In bounds
                Some(piece) => {
                    if piece.is_wall() {
                        return false;
                    }
                    true
                }
                None => false,
            },
            None => false,
        }
    }

    /// Get a reference to a piece
    pub fn get_piece(&self, gridx: usize, gridy: usize) -> Result<&GridPiece, &str> {
        match self.grid.get(gridx) {
            Some(column) => match column.get(gridy) {
                Some(piece) => Ok(&piece),
                None => Err("Attempt to load non-existing piece"),
            },
            None => Err("Attempt to load non-existing row"),
        }
    }

    // Get x and y coordinates of a piece
    pub fn get_translation(&self, gridx: usize, gridy: usize) -> Result<(i32, i32), &str> {
        match self.grid.get(gridx) {
            Some(column) => match column.get(gridy) {
                Some(piece) => Ok((piece.x(), piece.y())),
                None => Err("Attempt to get position of non-existing piece"),
            },
            None => Err("Attempt to get position of non-existing row"),
        }
    }

    // Get neighbour of a certain piece
    pub fn get_neighbour(
        &self,
        gridx: usize,
        gridy: usize,
        direction: Direction,
    ) -> Result<&GridPiece, &str> {
        let mut x = gridx;
        let mut y = gridy;
        match direction {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }

        match self.grid.get(x) {
            Some(row) => match row.get(y) {
                Some(piece) => Ok(piece),
                None => Err("Attempt to get neighour that doesn't exist"),
            },
            None => Err("Attempt to get row of neighbour that doesn't exist"),
        }
    }

    // Same as get_neighbour, but checks for walls
    pub fn get_safe_neighbour(
        &self,
        gridx: usize,
        gridy: usize,
        direction: Direction,
    ) -> Result<&GridPiece, &str> {
        let mut x = gridx;
        let mut y = gridy;
        match direction {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }

        match self.grid.get(x) {
            Some(row) => match row.get(y) {
                Some(piece) => {
                    if piece.is_wall() {
                        Err("Neighbour is a wall!")
                    } else {
                        Ok(piece)
                    }
                }
                None => Err("Attempt to get neighour that doesn't exist"),
            },
            None => Err("Attempt to get row of neighbour that doesn't exist"),
        }
    }

    /// Get all neighbours (4-directional) of a piece, as a Vector
    pub fn get_neighbours(&self, gridx: usize, gridy: usize) -> Vec<&GridPiece> {
        let mut neighbours = Vec::new();

        match self.grid.get(gridx) {
            Some(row) => {
                match row.get(gridy + 1) {
                    Some(piece) => neighbours.push(piece),
                    None => (),
                }
                match row.get(gridy - 1) {
                    Some(piece) => neighbours.push(piece),
                    None => (),
                }
            }
            None => (),
        };

        match self.grid.get(gridx - 1) {
            Some(row) => match row.get(gridy) {
                Some(piece) => neighbours.push(piece),
                None => (),
            },
            None => (),
        };

        match self.grid.get(gridx + 1) {
            Some(row) => match row.get(gridy) {
                Some(piece) => neighbours.push(piece),
                None => (),
            },
            None => (),
        };

        neighbours
    }
}

pub struct EmptyTextureError;

impl std::fmt::Debug for EmptyTextureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tried to build with no textures set!")
    }
}
