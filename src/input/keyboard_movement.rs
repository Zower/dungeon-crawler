//! Keyboard movement.

use std::time::Duration;

use bevy::prelude::*;

use crate::{
    level::{Point, WalkPath},
    logic::{Direction, MOVEMENT_STEP},
    Level, Player,
};

/// Handles WASD movement
pub struct KeyboardMovementPlugin;

impl Plugin for KeyboardMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyboardInUse(false))
            .insert_resource(KeyboardMovement::new())
            .add_system(update_player_direction);
    }
}

struct KeyboardMovement {
    up_timer: Timer,
    down_timer: Timer,
    right_timer: Timer,
    left_timer: Timer,
    queued_move: Option<Direction>,
}

impl KeyboardMovement {
    pub fn new() -> Self {
        Self {
            up_timer: Timer::new(Duration::from_millis(MOVEMENT_STEP), false),
            down_timer: Timer::new(Duration::from_millis(MOVEMENT_STEP), false),
            right_timer: Timer::new(Duration::from_millis(MOVEMENT_STEP), false),
            left_timer: Timer::new(Duration::from_millis(MOVEMENT_STEP), false),
            queued_move: None,
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        self.up_timer.tick(delta);
        self.down_timer.tick(delta);
        self.left_timer.tick(delta);
        self.right_timer.tick(delta);
    }

    pub fn try_queue_move(&mut self, dir: Direction) {
        if dir != Direction::Still {
            if self.allowed_to_move(dir) {
                self.queued_move = Some(dir);
            }
        }
    }

    pub fn get_queued_move(&mut self) -> Option<Direction> {
        self.queued_move.take()
    }

    pub fn allowed_to_move(&mut self, dir: Direction) -> bool {
        match dir {
            Direction::Up => {
                let done = self.up_timer.finished();
                if done {
                    self.up_timer.reset();
                }
                done
            }
            Direction::Down => {
                let done = self.down_timer.finished();
                if done {
                    self.down_timer.reset();
                }
                done
            }
            Direction::Left => {
                let done = self.left_timer.finished();
                if done {
                    self.left_timer.reset();
                }
                done
            }
            Direction::Right => {
                let done = self.right_timer.finished();
                if done {
                    self.right_timer.reset();
                }
                done
            }
            Direction::Still => true,
        }
    }
}

fn update_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
    dont_move: Res<KeyboardInUse>,
    mut keyboard_movement: ResMut<KeyboardMovement>,
    levels: Res<Level>,
    mut player_query: Query<(&Point, &mut WalkPath), With<Player>>,
    time: Res<Time>,
) {
    keyboard_movement.tick(time.delta());

    if dont_move.0 {
        return;
    }

    let mut req_direction = Direction::Still;
    if keyboard_input.pressed(KeyCode::W) {
        req_direction = Direction::Up;
    }
    if keyboard_input.pressed(KeyCode::A) {
        req_direction = Direction::Left;
    }
    if keyboard_input.pressed(KeyCode::S) {
        req_direction = Direction::Down;
    }
    if keyboard_input.pressed(KeyCode::D) {
        req_direction = Direction::Right;
    }

    let direction = if let Some(dir) = keyboard_movement.get_queued_move() {
        keyboard_movement.try_queue_move(req_direction);
        dir
    } else if keyboard_movement.allowed_to_move(req_direction) {
        req_direction
    } else {
        keyboard_movement.try_queue_move(req_direction);
        Direction::Still
    };

    let (player_position, mut player_path) = player_query.single_mut();
    let map = levels.get_current();

    let current_tile = map.get_tile(player_position).unwrap();

    match direction {
        Direction::Still => (),
        _ => {
            player_path.0.clear();
            if let Some(next) = map.get_neighbour(current_tile, req_direction) {
                if next.is_safe() {
                    player_path.0.push(next.position);
                }
            }
        }
    }
}

#[derive(Debug)]
/// Don't move the character
pub struct KeyboardInUse(pub bool);
