use crate::level::{Path, Point};

#[derive(Debug)]
pub struct Player {
    pub current: Point,
    pub path: Path,
}

pub struct Blob {
    pub current: Point,
}
