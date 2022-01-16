use bevy::prelude::Component;

/// A 2D point on the game grid.
#[derive(Debug, Component, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Size of a rectangular object
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    /// Constructs a Size with width and height set to size
    pub fn splat(size: i32) -> Size {
        Size {
            width: size,
            height: size,
        }
    }
}

/// A path to walk, this should always be a valid path as there is no validity-checking when moving an entity based on this path.
/// The first element is the next piece, NOT current, last is the goal
#[derive(Debug, Component)]
pub struct WalkPath(pub Vec<Point>);
