/// Experimental collision system, can register a collision between any two entities.
use std::time::Duration;

use bevy::{core::Timer, prelude::*};

use crate::{
    entity::{Blob, Player},
    level::Point,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Collisions::new())
            // .add_system(update_all_collisions)
            .add_system(register_player_collisions)
            .add_system(process_all_collision_timers)
            .add_system(get_player_collisions);
    }
}
// fn register_all_collisions(
//     valid_colliders: Query<(Entity, &Position)>,
//     mut collisions: ResMut<Collisions>,
// ) {
//     for (entity, position) in valid_colliders.iter() {
//         for (other_entity, other_position) in valid_colliders.iter() {
//             if !(entity == other_entity) && position == other_position {
//                 collisions.try_add(entity, other_entity);
//             }
//         }
//     }
// }

fn register_player_collisions(
    player_query: Query<(Entity, &Point), With<Player>>,
    all_ai_entities_query: Query<(Entity, &Point), (Without<Player>, Without<Blob>)>,
    mut collisions: ResMut<Collisions>,
) {
    let (player, player_position) = player_query.single();
    for (other, other_position) in all_ai_entities_query.iter() {
        if player_position == other_position {
            collisions.try_add(player, other);
        }
    }
}

fn get_player_collisions(
    player_query: Query<Entity, With<Player>>,
    mut collisions: ResMut<Collisions>,
) {
    let player = player_query.single();
    let with = collisions.colliding_with(player);
    if !with.is_empty() {
        debug!("Player collided with {with:?}");
    }
}

fn process_all_collision_timers(time: Res<Time>, mut collisions: ResMut<Collisions>) {
    collisions.process_timers(time.delta())
}

pub struct Collisions {
    pub collisions: Vec<Collision>,
}

impl Collisions {
    pub fn new() -> Self {
        Self {
            collisions: Vec::new(),
        }
    }

    /// Adds the collision if the two entities are not already colliding
    pub fn try_add(&mut self, entity1: Entity, entity2: Entity) {
        if !self
            .collisions
            .iter()
            .any(|c| c.has_entities(entity1, entity2))
        {
            self.collisions.push(Collision::new(entity1, entity2));
        }
    }

    /// Processes the timer of each collision
    pub fn process_timers(&mut self, duration: Duration) {
        for c in self.collisions.iter_mut() {
            c.timer.tick(duration);
        }
        self.collisions.retain(|c| !c.finished());
    }
    /// Find all entities (from: Entity) is colliding with. Marks all those collisions as processed,
    /// meaning they wont be returned until the collision is removed from the system (after the collision is > Collision:COLLIDE_TIME_SECONDS old and has been updated with process_timers())
    pub fn colliding_with(&mut self, from: Entity) -> Vec<Entity> {
        let mut entities = Vec::new();
        for collision in self.collisions.iter_mut() {
            if let Some(e) = collision.colliding_with(from) {
                entities.push(e);
                collision.processed = true;
            }
        }
        entities
    }
}

#[derive(Debug)]
pub struct Collision {
    entity1: Entity,
    entity2: Entity,
    pub processed: bool,
    timer: Timer,
}

impl Collision {
    fn new(entity1: Entity, entity2: Entity) -> Self {
        Self {
            entity1,
            entity2,
            processed: false,
            timer: Timer::from_seconds(Collision::COLLIDE_TIME_SECONDS, false),
        }
    }

    const COLLIDE_TIME_SECONDS: f32 = 1.0;

    fn colliding_with(&self, from: Entity) -> Option<Entity> {
        if !self.processed {
            if self.entity1 == from {
                // self.processed = true;
                return Some(self.entity2);
            } else if self.entity2 == from {
                // self.processed = true;
                return Some(self.entity1);
            }
        }
        None
    }

    fn has_entities(&self, entity1: Entity, entity2: Entity) -> bool {
        (self.entity1 == entity1 && self.entity2 == entity2)
            || (self.entity2 == entity1 && self.entity1 == entity2)
    }

    fn finished(&self) -> bool {
        self.timer.finished()
    }
}
