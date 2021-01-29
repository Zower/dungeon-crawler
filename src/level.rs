//! Everything related to the levels of the game

use bevy::prelude::*;

const _TILE_SIZE: f32 = 32.0;

/// A level to be played on
#[derive(Debug)]
pub struct Level {
    /// A list of level tiles
    grid: Vec<GridPiece>,
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

impl LevelBuilder {
    pub fn new(length: i32, height: i32) -> Self {
        LevelBuilder {
            size: LevelSize { length, height },
            ..Default::default()
        }
    }

    pub fn square(size: i32) -> Self {
        LevelBuilder {
            size: LevelSize {
                length: size,
                height: size,
            },
            ..Default::default()
        }
    }

    pub fn add_texture(
        &mut self,
        tile_type: TileType,
        texture: Handle<ColorMaterial>,
    ) -> &mut Self {
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

        for x in 0..self.size.length {
            for y in 0..self.size.height {
                grid.push(GridPiece {
                    texture: TileTexture {
                        texture: self.textures[0].texture.clone(),
                        tile: self.textures[0].tile.clone(),
                    }, // This is temporarily only the first texture
                    x,
                    y,
                })
            }
        }

        Ok(Level { grid })
    }

    pub fn print(&self) {
        println!("Hello world!, size: {:?}", self.size);
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

#[derive(Debug)]
/// The size of a level, currently square
//TODO: Should be a size? is the map always square?
struct LevelSize {
    length: i32,
    height: i32,
}

impl Default for LevelSize {
    fn default() -> Self {
        LevelSize {
            length: 10,
            height: 10,
        }
    }
}

///One piece on the grid, will have a texture
#[derive(Debug)]
struct GridPiece {
    texture: TileTexture,
    x: i32,
    y: i32,
}

impl Level {
    pub fn get_texture(&self, grid_pos: i32) -> Handle<ColorMaterial> {
        self.grid[0].texture.texture.clone()
    }

    pub fn print(&self) {
        println!("{:?}", self.grid)
    }
}

// impl Plugin for LevelPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.add_resource(LevelSize {
//             length: 10,
//             height: 10,
//         })
//         .add_startup_system(setup.system());
//     }
// }

// fn setup(
//     commands: &mut Commands,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     asset_server: Res<AssetServer>,
//     size: Res<LevelSize>,
//     //windows: Res<Windows>,
// ) {
//     let texture_tile = asset_server.load("tiles/floor.png");

//     //let window = windows.get_primary().unwrap();

//     let mut posx = -100.0;
//     let mut posy = 200.0;

//     let mut level_batch_sprite = Vec::new();
//     for _ in 0..size.length {
//         for _ in 0..size.height {
//             level_batch_sprite.push(SpriteBundle {
//                 sprite: Sprite::new(Vec2::new(TILE_SIZE, TILE_SIZE)),
//                 material: materials.add(texture_tile.clone().into()),
//                 //material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
//                 transform: Transform {
//                     translation: Vec3::new(posx, posy, 0.0),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             });
//             posx += TILE_SIZE + 2.0;
//         }
//         posx = -100.0;
//         posy -= TILE_SIZE + 2.0;
//     }

//     commands.spawn_batch(level_batch_sprite);
// }

pub struct EmptyTextureError;

impl std::fmt::Debug for EmptyTextureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tried to build with no textures set!")
    }
}
