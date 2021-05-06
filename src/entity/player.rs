use crate::{
    level::{Path, Point},
    logic::Moveable,
};

#[derive(Debug)]
pub struct Player {
    pub current: Point,
    pub path: Path,
}

pub struct Blob {
    pub current: Point,
}

impl Moveable for Player {
    fn path(&self) -> &Path {
        &self.path
    }
}
