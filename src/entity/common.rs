use crate::level::Point;

#[derive(Debug)]
pub struct Health(pub i32);

impl Health {
    pub fn damage(&mut self, amount: i32) {
        self.0 -= amount;
    }
    pub fn heal(&mut self, amount: i32) {
        self.0 += amount;
    }
}

pub struct HealthText;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct Position(pub Point);
