//! The game is played on a 'level', consisting of a set number of randomly generated maps.

use core::panic;
use rand::Rng;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::ops::Range;

use super::{
    common::{Point, Size},
    tile::*,
};
use crate::logic::Direction;

use super::rect::Rect;

use bevy::prelude::*;

/// Represents a single level
#[derive(Debug)]
pub struct Map {
    /// The depth of this level, if the player has descended 3 times, this level is number 4, etc.
    depth: usize,
    /// A list of level tiles, in order.
    grid: Vec<Tile>,
    /// The size of the level
    pub size: Size,
}

/// Builder structure that creates levels, will eventually hold procedural generation logic.

pub struct MapBuilder {
    /// The size of the map to be generated.
    map_size: Size,
    /// Number of rooms that can be generated.
    nr_rooms: i32,
    /// How big rooms can be.
    room_size_range_x: Range<i32>,
    /// How big rooms can be.
    room_size_range_y: Range<i32>,
    /// The depth of this level.
    depth: usize,
}

impl MapBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn square(size: i32) -> Self {
        Self {
            map_size: Size::splat(size),
            ..Default::default()
        }
    }

    /// Set the map size
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.map_size = size;
        self
    }

    /// Sets depth of level.
    pub fn depth(&mut self, depth: usize) -> &mut Self {
        self.depth = depth;
        self
    }

    pub fn nr_rooms(&mut self, amount: i32) -> &mut Self {
        self.nr_rooms = amount;
        self
    }

    /// Sets possible size of rooms.
    pub fn room_size(&mut self, x: Range<i32>, y: Range<i32>) -> &mut Self {
        self.room_size_range_x = x;
        self.room_size_range_y = y;
        self
    }

    pub fn build_all_floors(&self) -> (Map, Vec<Rect>) {
        let mut map = Map {
            depth: self.depth,
            grid: Vec::with_capacity((self.map_size.width * self.map_size.height) as usize),
            size: self.map_size,
        };

        debug!(
            "Generating map with all floors with size: x: {:?}, y: {:?}",
            self.map_size.width, self.map_size.height
        );

        for x in 0..self.map_size.width {
            for y in 0..self.map_size.height {
                map.grid.push(Tile::new(Surface::Floor, Point::new(x, y)));
            }
        }

        (map, vec![])
    }

    /// Build a random map.
    pub fn build(&self) -> (Map, Vec<Rect>) {
        let mut map = Map {
            depth: self.depth,
            grid: Vec::with_capacity((self.map_size.width * self.map_size.height) as usize),
            size: self.map_size,
        };

        debug!(
            "Generating map with size: x: {:?}, y: {:?}",
            self.map_size.width, self.map_size.height
        );

        for x in 0..self.map_size.width {
            for y in 0..self.map_size.height {
                map.grid.push(Tile::new(Surface::Wall, Point::new(x, y)));
            }
        }

        let mut rng = rand::thread_rng();
        let mut rooms: Vec<Rect> = Vec::new();

        for _ in 1..=self.nr_rooms {
            let w = rng.gen_range(self.room_size_range_x.clone());
            let h = rng.gen_range(self.room_size_range_y.clone());
            let x = rng.gen_range(1..self.map_size.width - w - 1);
            let y = rng.gen_range(1..self.map_size.height - h - 1);

            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                debug!("Creating room: {new_room:?}");
                for x in new_room.x1..=new_room.x2 {
                    for y in new_room.y1..=new_room.y2 {
                        map.grid[Map::xy_idx(&Point::new(x, y), self.map_size.width)].surface =
                            Surface::Floor;
                    }
                }

                if !rooms.is_empty() {
                    let new_point = new_room.center();
                    let prev_point = rooms[rooms.len() - 1].center();
                    if rng.gen_range(0..2) == 1i32 {
                        map.apply_horizontal_tunnel(prev_point.x, new_point.x, prev_point.y);
                        map.apply_vertical_tunnel(prev_point.y, new_point.y, new_point.x);
                    } else {
                        map.apply_vertical_tunnel(prev_point.y, new_point.y, prev_point.x);
                        map.apply_horizontal_tunnel(prev_point.x, new_point.x, new_point.y);
                    }
                }

                rooms.push(new_room);
            }
        }

        (map, rooms)
    }
}

impl Map {
    /// Get a reference to a piece
    /// Returns None if the point is OOB for the current level size, or values are less than 0.
    pub fn get_tile(&self, point: &Point) -> Option<&Tile> {
        if let Some(index) = self.translate(point) {
            self.grid.get(index)
        } else {
            None
        }
    }

    pub fn get_tile_mut(&mut self, point: &Point) -> Option<&mut Tile> {
        if let Some(index) = self.translate(point) {
            self.grid.get_mut(index)
        } else {
            None
        }
    }

    pub fn update_visibility(&mut self, visible_tiles: &Vec<Point>) {
        for point in visible_tiles {
            self.get_tile_mut(point).unwrap().revealed = true;
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

            if let Some(index) = self.translate(&neighbour_point) {
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
    pub fn get_neighbours(&self, position: &Point) -> Vec<&Tile> {
        debug_assert!(self.in_bounds(position));

        let mut neighbours = Vec::new();
        // Left neighbour potential point
        let mut neighbour_point = Point {
            x: position.x - 1,
            y: position.y,
        };

        // Get index of the point, should always be valid so unwrap() is safe
        if let Some(index) = self.translate(&neighbour_point) {
            // Push the neighbour to final vector.
            neighbours.push(self.grid.get(index).unwrap());
        }

        // Changing to right neighbour
        neighbour_point.x += 2;
        if let Some(index) = self.translate(&neighbour_point) {
            neighbours.push(self.grid.get(index).unwrap());
        }

        // Changing to above neighbour
        neighbour_point.x -= 1;
        neighbour_point.y += 1;
        if let Some(index) = self.translate(&neighbour_point) {
            neighbours.push(self.grid.get(index).unwrap());
        }

        // Changing to below neighbour
        neighbour_point.y -= 2;
        if let Some(index) = self.translate(&neighbour_point) {
            neighbours.push(self.grid.get(index).unwrap());
        }

        // Return valid neighbours
        neighbours
    }
}

// More utility focused implementations
impl Map {
    pub fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = Map::xy_idx(&Point::new(x, y), self.size.height);
            if idx > 0 && idx < 80 * 50 {
                self.grid[idx as usize].surface = Surface::Floor;
            }
        }
    }

    pub fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = Map::xy_idx(&Point::new(x, y), self.size.height);
            if idx > 0 && idx < 80 * 50 {
                self.grid[idx as usize].surface = Surface::Floor;
            }
        }
    }
    pub fn in_bounds(&self, point: &Point) -> bool {
        point.x < self.size.width && point.y < self.size.height && point.x >= 0 && point.y >= 0
    }

    /// Translate 2D coordinates into a index
    /// Returns None if the point is OOB.
    pub fn translate(&self, point: &Point) -> Option<usize> {
        if self.in_bounds(point) {
            return Some(Self::xy_idx(point, self.size.height));
        }
        None
    }

    pub fn xy_idx(point: &Point, height: i32) -> usize {
        (point.x * height + point.y) as usize
    }

    /// A* implemented from <https://www.redblobgames.com/pathfinding/a-star/introduction.html>
    pub fn a_star(&self, start: Point, goal: Point) -> Vec<Point> {
        let mut frontier = BinaryHeap::new();
        let start_point = self.get_tile(&start).unwrap().position;
        frontier.push(Reverse(TilePriority {
            point: &start_point,
            priority: 0,
        }));

        let mut came_from = HashMap::new();
        came_from.insert(&start_point, &start_point); // First just points to itself

        let mut cost_so_far = HashMap::new();
        cost_so_far.insert(&start_point, 0);

        while let Some(current) = frontier.pop() {
            // Pop returns PiecePriority, need the tile itself.
            let current = current.0.point;

            if *current == goal {
                break;
            }

            for neighbour in self.get_neighbours(current) {
                // Neighbour is not in came_from
                let new_cost = cost_so_far.get(current).unwrap() + neighbour.cost;
                // if !came_from.contains_key(neighbour) {
                if !cost_so_far.contains_key(&neighbour.position)
                    || new_cost < *cost_so_far.get(&neighbour.position).unwrap()
                {
                    // Make sure its not a wall, etc.
                    if neighbour.is_safe() {
                        cost_so_far.insert(&neighbour.position, new_cost);
                        let priority = new_cost + Map::heuristic(goal, neighbour.position);
                        frontier.push(Reverse(TilePriority {
                            point: &neighbour.position,
                            priority,
                        }));
                        came_from.insert(&neighbour.position, current);
                    }
                }
            }
        }

        // Came_from now includes the correct path, that can be traced back from came_from[goal]
        let mut path = Vec::new();

        let mut current = self.get_tile(&goal).unwrap().position;

        while current != start_point {
            path.push(current);
            current = *came_from.remove(&current).unwrap();
        }

        path.reverse();

        path
    }

    pub fn reveal_all(&mut self) {
        for tile in &mut self.grid {
            tile.revealed = true;
        }
    }

    fn heuristic(a: Point, b: Point) -> i32 {
        i32::abs(a.x - b.x) + i32::abs(a.y - b.y)
    }
}

impl Default for MapBuilder {
    fn default() -> Self {
        Self {
            map_size: Size::splat(50),
            nr_rooms: 25,
            room_size_range_x: 4..10,
            room_size_range_y: 2..8,
            depth: 0,
        }
    }
}

/// A wrapper struct that can be put into a priorityqueue, prioritized by cost, so that I dont have to implement ordering on Tile.
#[derive(Debug, PartialEq, Eq)]
struct TilePriority<'a> {
    point: &'a Point,
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
