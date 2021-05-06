//! The actual Level and LevelBuilder structs

use core::panic;
use rand::Rng;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

use super::{
    common::{Point, ScreenPoint, Size},
    tile::*,
};
use crate::logic::Direction;

use bevy::prelude::*;

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
    /// Panics if the tile position doesnt match the input.
    pub fn get_tile(&self, point: Point) -> Option<&Tile> {
        if let Some(index) = self.translate(point) {
            let tile = self.grid.get(index).unwrap();
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

            if let Some(index) = self.translate(neighbour_point) {
                self.grid.get(index)
            } else {
                None
            }
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

            // Get index of the point, should always be valid so unwrap() is safe
            if let Some(index) = self.translate(neighbour_point) {
                // Push the neighbour to final vector.
                neighbours.push(self.grid.get(index).unwrap());
            }

            // Changing to right neighbour
            neighbour_point.x += 2;
            if let Some(index) = self.translate(neighbour_point) {
                neighbours.push(self.grid.get(index).unwrap());
            }

            // Changing to above neighbour
            neighbour_point.x -= 1;
            neighbour_point.y += 1;
            if let Some(index) = self.translate(neighbour_point) {
                neighbours.push(self.grid.get(index).unwrap());
            }

            // Changing to below neighbour
            neighbour_point.y -= 2;
            if let Some(index) = self.translate(neighbour_point) {
                neighbours.push(self.grid.get(index).unwrap());
            }

            // Return valid neighbours
            neighbours
        } else {
            error!("Tried to get neighbours of a Tile that is not in the grid, panicing!");
            panic!();
        }
    }
}

// More utility focused implementations
impl Level {
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x < self.size.width && point.y < self.size.height && point.x >= 0 && point.y >= 0
    }

    /// Translate 2D coordinates into a index
    /// Returns None if the point is OOB.
    pub fn translate(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            let mut index = point.x * self.size.height;
            index += point.y;

            return Some(index as usize);
        }
        None
    }
    /// A* implemented from https://www.redblobgames.com/pathfinding/a-star/introduction.html
    pub fn a_star(&self, start: Point, goal: Point) -> Vec<Point> {
        let start_tile = self.get_tile(start).unwrap();
        let mut frontier = BinaryHeap::new();
        frontier.push(Reverse(TilePriority {
            tile: start_tile,
            priority: 0,
        }));

        let mut came_from = HashMap::new();
        came_from.insert(start_tile, start_tile); // First just points to itself

        let mut cost_so_far = HashMap::new();
        cost_so_far.insert(start_tile, 0);

        while let Some(current) = frontier.pop() {
            // Pop returns PiecePriority, need the tile itself.
            let current = current.0.tile;

            if current.position == goal {
                break;
            }

            for neighbour in self.get_neighbours(current) {
                // Neighbour is not in came_from
                let new_cost = cost_so_far.get(current).unwrap() + neighbour.cost;
                // if !came_from.contains_key(neighbour) {
                if !cost_so_far.contains_key(neighbour)
                    || new_cost < *cost_so_far.get(neighbour).unwrap()
                {
                    // Make sure its not a wall, etc.
                    if neighbour.is_safe() {
                        cost_so_far.insert(neighbour, new_cost);
                        let priority = new_cost + Level::heuristic(goal, neighbour.position);
                        frontier.push(Reverse(TilePriority {
                            tile: neighbour,
                            priority,
                        }));
                        came_from.insert(neighbour, current);
                    }
                }
            }
        }

        // Came_from now includes the correct path, that can be traced back from came_from[goal]

        let mut path = Vec::new();

        let mut current = self.get_tile(goal).unwrap();

        while current != start_tile {
            path.push(current.position);
            current = came_from.get(current).unwrap();
        }

        path.reverse();

        path
    }
    fn heuristic(a: Point, b: Point) -> i32 {
        i32::abs(a.x - b.x) + i32::abs(a.y - b.y)
    }
}
/// A wrapper struct that can be put into a priorityqueue, prioritized by cost, so that I dont have to implement ordering on Tile.
#[derive(Debug, PartialEq, Eq)]
struct TilePriority<'a> {
    tile: &'a Tile,
    priority: i32,
}
impl<'a> Ord for TilePriority<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<'a> PartialOrd for TilePriority<'a> {
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
