//! Everything related to the levels of the game

use bevy::prelude::*;

pub const TILE_SIZE: f32 = 32.0;

/// A level to be played on
#[derive(Debug)]
pub struct Level {
    /// A list of level tiles
    grid: Vec<GridPiece>,
    size: LevelSize,
}
///One piece on the grid, will have a texture
#[derive(Debug)]
pub struct GridPiece {
    texture: TileTexture,
    gridx: i32,
    gridy: i32,
    x: f32,
    y: f32,
}
#[derive(Debug, Copy, Clone)]
pub enum TileType {
    Floor,
    Wall,
}

#[derive(Debug)]
pub struct TileTexture {
    texture: Handle<ColorMaterial>,
    tile: TileType,
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

        let mut x = 0.0;
        let mut y = 0.0;

        for gridx in 0..self.size.width {
            for gridy in 0..self.size.height {
                grid.push(GridPiece {
                    texture: TileTexture {
                        texture: self.textures[0].texture.clone(),
                        tile: self.textures[0].tile.clone(),
                    }, // This is temporarily only the first texture
                    gridx,
                    gridy,
                    x,
                    y,
                });
                y += TILE_SIZE;
            }
            x += TILE_SIZE;
            y = 0.0;
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
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
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
    pub fn get_texture(&self, grid_pos: usize) -> Handle<ColorMaterial> {
        self.grid[grid_pos].texture.texture.clone()
    }

    pub fn tiles(&self) -> std::slice::Iter<'_, GridPiece> {
        self.grid.iter()
    }

    pub fn get_translation(&self, gridx: i32, gridy: i32) -> (f32, f32) {
        println!("{}, {}", gridx, gridy);

        for (i, piece) in self.grid.iter().enumerate() {
            println!(
                "index: {}, gridx: {}, gridy: {}",
                i,
                piece.gridx(),
                piece.gridy()
            );
            // if piece.gridx() == gridx && piece.gridy() == gridy {
            //     return (piece.x, piece.y);
            // }
        }
        (5.0, 5.0)
    }
}

pub struct EmptyTextureError;

impl std::fmt::Debug for EmptyTextureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tried to build with no textures set!")
    }
}
