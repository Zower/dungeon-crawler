/// Various Tile related structures
use bevy::prelude::Component;

///One tile on the grid
// #[derive(Debug, PartialEq)]
// pub struct Tile {
//     pub surface: Surface,
//     /// The piece's Position on the board, necessary?
//     pub position: ,
//     pub revealed: bool,
//     pub screen_position: Vec2,
//     /// Currently always 1, potentially some different terrain etc.
//     pub cost: i32,
// }

#[derive(Debug, Component)]
pub struct Wall;

#[derive(Debug, Component)]
pub struct Floor;
// /// Contains the texture to be used by bevy, and a type
// #[derive(Debug, PartialEq, Eq, Hash)]
// pub struct TileType {
//     pub texture: Handle<Image>,
//     pub surface: Surface,
// }

// impl Tile {
//     pub fn new(surface: Surface, position: TilePos) -> Self {
//         Self {
//             surface,
//             position,
//             revealed: false,
//             screen_position: Vec2::new(
//                 position.x as f32 * TILE_SIZE,
//                 position.y as f32 * TILE_SIZE,
//             ),
//             cost: 1,
//         }
//     }
//     /// Get the world position of this tile
//     pub fn screen_position(&self) -> Vec2 {
//         self.screen_position
//     }

//     pub fn is_safe(&self) -> bool {
//         !self.is_wall() // && others...
//     }

//     pub fn is_wall(&self) -> bool {
//         self.surface == Surface::Wall
//     }
// }

// /// A component that can be used to identify tiles in the ECS.
// #[derive(Debug, Component)]
// pub struct TileComponent(pub TilePos);
