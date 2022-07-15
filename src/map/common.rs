use bevy::{prelude::{Component, Deref, DerefMut}, math::Vec2};

#[derive(Debug, Component)]
pub struct TileRect {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

impl TileRect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &TileRect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            (self.x1 + self.x2) as f32 / 2.,
            (self.y1 + self.y2) as f32 / 2.,
        )
    }
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct Room(pub TileRect);