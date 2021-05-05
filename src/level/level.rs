//! Everything related to the levels of the game

use bevy::prelude::*;
use core::panic;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use super::common::{Point, ScreenPoint, Size};
use super::tile::*;
use crate::input::Direction;

/// Represents a single level
#[derive(Debug)]
pub struct Level {
    /// The 'number' of this level, if the player has descended 3 times, this level is number 4, etc.
    number: usize,
    /// A list of level tiles, ALWAYS in order. E.g., (0,0), (0,1), (0,2), NOT (0,1), (0,0), (0,2)
    grid: Vec<Tile>,
    /// The size of the level
    pub size: Size,
}

/// Builder structure that creates levels, will eventually hold procedural generation logic.
pub struct LevelBuilder {
    /// The tiles that this builder can use
    building_tiles: Vec<TileType>,
    /// The size of the level to be generated.
    build_size: Size,
}

impl LevelBuilder {
    pub fn new(build_size: Size) -> Self {
        LevelBuilder {
            build_size,
            building_tiles: Vec::<TileType>::new(),
        }
    }

    pub fn square(size: i32) -> Self {
        LevelBuilder {
            build_size: Size {
                width: size,
                height: size,
            },
            building_tiles: Vec::<TileType>::new(),
        }
    }

    /// Set the build_size of this builder
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.build_size = size;
        self
    }

    /// Add a tile to this level, it needs a type and a texture.
    pub fn add_tile(&mut self, surface: Surface, texture: Handle<ColorMaterial>) -> &mut Self {
        self.building_tiles.push(TileType { texture, surface });
        self
    }

    pub fn build(&self, number: usize) -> Result<Level, Box<dyn std::error::Error>> {
        if self.building_tiles.len() == 0 {
            return Err(Box::new(EmptyTextureError));
        }

        let mut grid = Vec::new();
        let mut rng = rand::thread_rng();

        for row in 0..self.build_size.width {
            for column in 0..self.build_size.height {
                let mut surface = rng.gen_range(0..11);
                match surface {
                    0 => surface = 1,
                    _ => surface = 0,
                } // Yeah this is jank, just testing walls
                grid.push(Tile {
                    tile_type: TileType {
                        texture: self.building_tiles[surface].texture.clone(),
                        surface: self.building_tiles[surface].surface,
                    },
                    position: Point { x: row, y: column },
                    screen_position: ScreenPoint(Point {
                        x: row * TILE_SIZE,
                        y: column * TILE_SIZE,
                    }),
                    cost: 1,
                })
            }
        }

        Ok(Level {
            grid,
            size: self.build_size,
            number,
        })
    }
}

impl Level {
    /// Returns an iterator over the grid
    pub fn tiles(&self) -> std::slice::Iter<'_, Tile> {
        self.grid.iter()
    }

    /// Get a reference to a piece
    /// Returns None if the point is OOB for the current level size, or values are less than 0.
    /// # Panics
    /// Panics if the 2D to 1D conversion returns an index that doesn't exist, or if the tile position doesnt match the input.
    pub fn get_tile(&self, point: Point) -> Option<&Tile> {
        if point.x < self.size.width && point.y < self.size.height && point.x >= 0 && point.y >= 0 {
            let index = Level::translate(point, self.size.height);

            if let Some(tile) = self.grid.get(index) {
                if tile.position.x == point.x && tile.position.y == point.y {
                    Some(tile)
                } else {
                    error!(
                        "Tile position {:?} doesn't match input {:?}, panicing!",
                        tile.position, point
                    );
                    panic!()
                }
            } else {
                error!(
                    "Tried to get tile at P: {:?}, I: {:?}, but that index doesnt exist, panicing!",
                    point, index
                );
                panic!()
            }
        } else {
            None
        }
    }

    // Get neighbour of a certain tile
    pub fn get_neighbour(&self, tile: &Tile, direction: Direction) -> Option<&Tile> {
        if self.grid.contains(tile) {
            let mut neighbour_point = tile.position;
            match direction {
                Direction::Up => neighbour_point.y += 1,
                Direction::Down => neighbour_point.y -= 1,
                Direction::Left => neighbour_point.x -= 1,
                Direction::Right => neighbour_point.x += 1,
                Direction::Still => (),
            }

            self.grid
                .get(Level::translate(neighbour_point, self.size.height))
        } else {
            error!("Tried to get neighbours of a Tile that is not in the grid, panicing!");
            panic!();
        }
    }

    /// Get all neighbours (4-directional) of a piece
    pub fn get_neighbours(&self, tile: &Tile) -> Vec<&Tile> {
        // Check that a valid reference was passed
        if self.grid.contains(tile) {
            let mut neighbours = Vec::new();
            // Left neighbour potential point
            let mut neighbour_point = Point {
                x: tile.position.x - 1,
                y: tile.position.y,
            };

            // If that neighbour exists
            if let Some(left_neighbour) = self
                .grid
                .get(Level::translate(neighbour_point, self.size.height))
            {
                // Push the neighbour to final vector.
                neighbours.push(left_neighbour)
            }
            // Changing to right neighbour
            neighbour_point.x += 2;
            if let Some(right_neighbour) = self
                .grid
                .get(Level::translate(neighbour_point, self.size.height))
            {
                neighbours.push(right_neighbour)
            }
            // Changing to above neighbour
            neighbour_point.x -= 1;
            neighbour_point.y += 1;

            if let Some(above_neighbour) = self
                .grid
                .get(Level::translate(neighbour_point, self.size.height))
            {
                neighbours.push(above_neighbour)
            }
            // Changing to below neighbour
            neighbour_point.y -= 2;

            if let Some(below_neighbour) = self
                .grid
                .get(Level::translate(neighbour_point, self.size.height))
            {
                neighbours.push(below_neighbour)
            }

            // Return valid neighbours
            neighbours
        } else {
            error!("Tried to get neighbours of a Tile that is not in the grid, panicing!");
            panic!();
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x < self.size.width && point.y < self.size.height
    }
}
/// A wrapper struct that can be put into a priorityqueue, prioritized by cost, so that I dont have to implement ordering on Tile.
#[derive(Debug, PartialEq, Eq)]
struct PiecePriority<'a> {
    tile: &'a Tile,
}

impl Level {
    // Translate 2D coordinates into a index
    pub fn translate(point: Point, height: i32) -> usize {
        let mut index = point.x * height;
        index += point.y;

        index as usize
    }
    /// A* implemented from https://www.redblobgames.com/pathfinding/a-star/introduction.html, not finished, TODO: Reverse priority_queue, heuristics
    /// Should be member function? idk
    pub fn a_star(level: &Level, start: Point, goal: Point) -> Vec<Point> {
        let mut start_tile = level.get_tile(start).unwrap();

        let mut frontier = BinaryHeap::new();
        frontier.push(PiecePriority { tile: start_tile });

        let mut came_from = HashMap::new();

        came_from.insert(start_tile, start_tile); // First just points to itself

        while let Some(current) = frontier.pop() {
            // Pop returns PiecePriority, need the tile itself.
            let current = current.tile;

            if current.position == goal {
                break;
            }

            for neighbour in level.get_neighbours(current) {
                // Neighbour is not in came_from
                if !came_from.contains_key(neighbour) {
                    // Make sure its not a wall, etc.
                    if neighbour.is_safe() {
                        frontier.push(PiecePriority { tile: neighbour });
                        came_from.insert(neighbour, current);
                    }
                }
            }
        }

        // Came_from now includes the correct path, that can be traced back from came_from[goal]

        let mut path = Vec::new();

        let mut current = level.get_tile(goal).unwrap();

        while current != start_tile {
            path.push(current.position);
            current = came_from[&current];
        }

        path.reverse();

        path
    }
}
impl<'a> Ord for PiecePriority<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tile.cost.cmp(&other.tile.cost)
    }
}

impl<'a> PartialOrd for PiecePriority<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct EmptyTextureError;

impl std::fmt::Display for EmptyTextureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tried to build with no textures set!")
    }
}

impl std::error::Error for EmptyTextureError {}
