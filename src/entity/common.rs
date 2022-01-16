use bevy::prelude::Component;

#[derive(Debug, Component)]
pub struct Health(pub i32);

impl Health {
    pub fn damage(&mut self, amount: i32) {
        self.0 -= amount;
    }
    pub fn heal(&mut self, amount: i32) {
        self.0 += amount;
    }
}

#[derive(Debug, Component)]
pub struct HealthText;
