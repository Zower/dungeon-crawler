use std::ops::Add;

use bevy::prelude::info;

use super::{Map, Point};

pub fn set_visible(map: &mut Map, init_position: Point) {
    for octant in 0..=7 {
        set_visible_octant(map, octant, init_position);
    }
}

fn set_visible_octant(map: &mut Map, octant: u8, init_position: Point) {
    let mut blocked_fov = BlockedFov::new();
    for row in 1..7 {
        if !map.in_bounds(
            init_position + BlockedFov::rotate_for_octant(OctantPoint::new(row, 0), octant),
        ) {
            break;
        }

        for col in 0..=row {
            let pos =
                init_position + BlockedFov::rotate_for_octant(OctantPoint::new(row, col), octant);

            if !map.in_bounds(pos) {
                break;
            }

            if blocked_fov.is_fully_blocked() {
                map.get_tile_mut(pos).unwrap().revealed = false;
            } else {
                let blocker = ShadowBorder::new(OctantPoint::new(row, col));

                let visible = !blocked_fov.is_not_viewable(&blocker);
                map.get_tile_mut(pos).unwrap().revealed = visible;

                if visible && map.get_tile(pos).unwrap().is_wall() {
                    blocked_fov.add_blocker(blocker);
                }
            }
        }
    }
}

pub struct BlockedFov {
    pub borders: Vec<ShadowBorder>,
}

impl BlockedFov {
    pub fn new() -> Self {
        Self { borders: vec![] }
    }

    pub fn add_blocker(&mut self, new_border: ShadowBorder) {
        let index = self
            .borders
            .iter()
            .position(|b| b.start >= new_border.start)
            .unwrap_or(self.borders.len());

        let len = self.borders.len();
        let (first, second) = self.borders.split_at_mut(index);

        let overlapping_prev = (index > 0 && first.last().unwrap().end > new_border.start)
            .then(|| first.last_mut().unwrap());

        let overlapping_next =
            (index < len && second[0].start < new_border.end).then(|| &mut second[0]);

        if let Some(next) = overlapping_next {
            if let Some(prev) = overlapping_prev {
                prev.end = next.end;
                self.borders.remove(index);
            } else {
                next.start = new_border.start;
            }
        } else {
            if let Some(prev) = overlapping_prev {
                prev.end = new_border.end;
            } else {
                self.borders.insert(index, new_border);
            }
        }
    }

    pub fn is_not_viewable(&self, tile_border: &ShadowBorder) -> bool {
        self.borders.iter().any(|t_b| t_b.contains(tile_border))
    }

    pub fn is_fully_blocked(&self) -> bool {
        self.borders.len() == 1 && self.borders[0].start == 0. && self.borders[0].end == 1.
    }

    /// Panics if octant is not in 0..=7
    pub fn rotate_for_octant(point: OctantPoint, octant: u8) -> OctantPoint {
        let (x, y) = match octant {
            0 => (point.0.y, point.0.x),
            1 => (point.0.x, point.0.y),
            2 => (point.0.x, -point.0.y),
            3 => (point.0.y, -point.0.x),
            4 => (-point.0.y, -point.0.x),
            5 => (-point.0.x, -point.0.y),
            6 => (-point.0.x, point.0.y),
            7 => (-point.0.y, point.0.x),
            _ => panic!("Octants may only be rotated 360 degrees, octant should be in range 0..=7"),
        };

        OctantPoint::new(x, y)
    }
}

/// Where 0 < start < end <= 1,
#[derive(Debug)]
pub struct ShadowBorder {
    start: f32,
    end: f32,
}

impl ShadowBorder {
    pub fn new(point: OctantPoint) -> Self {
        Self {
            start: point.0.y as f32 / (point.0.x + 2) as f32,
            end: (point.0.y + 1) as f32 / (point.0.x + 1) as f32,
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

/// A point in an FOV octant. Origin is 0, 0 no matter where the entity being calculated on is.
pub struct OctantPoint(Point);

impl OctantPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self(Point::new(x, y))
    }
}

impl Add<OctantPoint> for Point {
    type Output = Self;

    fn add(self, rhs: OctantPoint) -> Self::Output {
        Self::new(self.x + rhs.0.x, self.y + rhs.0.y)
    }
}