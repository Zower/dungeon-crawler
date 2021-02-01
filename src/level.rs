//! Everything related to the levels of the game

use bevy::prelude::*;

pub const TILE_SIZE: i32 = 32;

/// A level to be played on
#[derive(Debug)]
pub struct Level {
    /// A list of level tiles
    grid: Vec<Vec<GridPiece>>,
    size: LevelSize,
}
///One piece on the grid, should have a type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GridPiece {
    texture: TileTexture,
    gridx: i32,
    gridy: i32,
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TileTexture {
    texture: Handle<ColorMaterial>,
    tile: TileType,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TileType {
    Floor,
    Wall,
}

pub struct LevelBuilder {
    textures: Vec<TileTexture>,
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
        self.textures.push(TileTexture {
            texture: texture,
            tile: tile_type,
        });
        self
    }

    pub fn build(&self) -> Result<Level, EmptyTextureError> {
        let mut grid = Vec::new();

        if self.textures.len() == 0 {
            println!("Trying to build with no textures");
            return Err(EmptyTextureError); //Custom error type yada yada
        }

        let mut x = 0;
        let mut y = 0;

        for gridx in 0..self.size.width {
            let mut grid_depth = Vec::new();
            for gridy in 0..self.size.height {
                grid_depth.push(GridPiece {
                    texture: TileTexture {
                        texture: self.textures[0].texture.clone(), // This is temporarily only the first texture
                        tile: self.textures[0].tile.clone(), // This is temporarily only the first tile
                    },
                    gridx,
                    gridy,
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
            textures: Vec::<TileTexture>::new(),
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
        self.gridx
    }

    pub fn gridy(&self) -> i32 {
        self.gridy
    }
}

impl Level {
    pub fn get_texture(&self, gridx: usize, gridy: usize) -> Handle<ColorMaterial> {
        self.grid[gridx][gridy].texture.texture.clone() // Error handling..
    }

    pub fn tiles(&self) -> std::slice::Iter<'_, Vec<GridPiece>> {
        self.grid.iter()
    }

    pub fn get_piece(&self, gridx: usize, gridy: usize) -> &GridPiece {
        return &self.grid[gridx][gridy]; // Please handle errors
    }

    pub fn get_translation(&self, gridx: usize, gridy: usize) -> (i32, i32) {
        let x = self.grid[gridx][gridy].x(); // Do I even need to say anything
        let y = self.grid[gridx][gridy].y();
        (x, y)
    }

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
