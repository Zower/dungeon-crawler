/// A 2D point on the grid. For screen position, use the ScreenPoint wrapper.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Wrapper for point, should be used for screen positions, not game board
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ScreenPoint(pub Point);

/// Size of a rectangular object
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    /// Constructs a Size with width and height set to size
    fn splat(size: i32) -> Size {
        Size {
            width: size,
            height: size,
        }
    }
}

/// A path to walk, this should always be a valid path as there is no validity-checking when moving an entity based on this path. Could be a reference of Points?
/// The first element is the next piece, NOT current, last is the goal
#[derive(Debug)]
pub struct WalkPath(pub Vec<Point>);
