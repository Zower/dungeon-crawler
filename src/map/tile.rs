use std::cmp::Ordering;

/// Various Tile related structures
use bevy::prelude::{Color, Component};

#[derive(Debug, Component)]
pub struct Wall;

#[derive(Debug, Component)]
pub struct Floor;

#[derive(PartialEq, Component, Copy, Clone)]
/// How to paint a certain tile this frame.
pub enum TilePaint {
    /// This tile is being painted with a color to indicate that the player is hovering over it.
    CursorDraw(Color),
    /// This tile is visible. Color is white.
    Visible,
    /// This tile has previously been seen.
    PreviouslySeen,
    /// This tile has not been seen yet.
    Invisible,
}

impl TilePaint {
    pub fn greater_of(&self, new: Self) -> Self {
        match self.partial_cmp(&new) {
            Some(Ordering::Greater) => *self,
            Some(Ordering::Less) => new,
            Some(Ordering::Equal) => new,
            None => unreachable!(),
        }
    }
}
