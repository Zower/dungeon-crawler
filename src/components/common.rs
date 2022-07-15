use bevy::prelude::{Component, Deref, DerefMut};

#[derive(Debug, Component, Deref, DerefMut)]
pub struct Health(pub i32);

impl Health {
    // pub fn damage(&mut self, amount: i32) {
    //     **self -= amount;
    // }
    // pub fn heal(&mut self, amount: i32) {
    //     **self += amount;
    // }
}

#[derive(Debug, Component)]
pub struct HealthText;

// #[derive(Component, Debug, Deref, DerefMut)]
// pub struct Velocity(pub bevy::math::Vec2);
