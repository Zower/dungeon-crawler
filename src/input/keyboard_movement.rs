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
            .insert_resource(RecentlyPressed::default())
            .insert_resource(LastKeyboardMovedDirection(Direction::Still))
            .add_system(update_player_direction);
    }
}

#[derive(Default, Debug)]
struct RecentlyPressed {
    up_down: Option<Duration>,
    left_right: Option<Duration>,
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
            Direction::Up => Self::reset_if_finished(&mut self.up_timer),
            Direction::Down => Self::reset_if_finished(&mut self.down_timer),
            Direction::Left => Self::reset_if_finished(&mut self.left_timer),
            Direction::Right => Self::reset_if_finished(&mut self.right_timer),
            Direction::Still => true,
        }
    }

    fn reset_if_finished(timer: &mut Timer) -> bool {
        let done = timer.finished();
        if done {
            timer.reset()
        }
        done
    }
}

fn update_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
    dont_move: Res<KeyboardInUse>,
    levels: Res<Level>,
    time: Res<Time>,
    mut recently_pressed: ResMut<RecentlyPressed>,
    mut keyboard_movement: ResMut<KeyboardMovement>,
    mut last_direction: ResMut<LastKeyboardMovedDirection>,
    mut player_query: Query<(&Point, &mut WalkPath), With<Player>>,
) {
    keyboard_movement.tick(time.delta());

    if dont_move.0 {
        return;
    }

    let up_pressed: i8 = keyboard_input.pressed(KeyCode::W).into();
    let left_pressed: i8 = keyboard_input.pressed(KeyCode::A).into();
    let down_pressed: i8 = keyboard_input.pressed(KeyCode::S).into();
    let right_pressed: i8 = keyboard_input.pressed(KeyCode::D).into();

    let y_dir = match up_pressed - down_pressed {
        -1 => Direction::Down,
        0 => Direction::Still,
        1 => Direction::Up,
        _ => unreachable!("Cast from boolean return number other than 0 or 1"),
    };

    recently_pressed.up_down = if y_dir == Direction::Still {
        None
    } else {
        if let Some(dur) = recently_pressed.up_down {
            Some(dur + time.delta())
        } else {
            Some(time.delta())
        }
    };

    let x_dir = match right_pressed - left_pressed {
        -1 => Direction::Left,
        0 => Direction::Still,
        1 => Direction::Right,
        _ => unreachable!("Cast from boolean return number other than 0 or 1"),
    };

    recently_pressed.left_right = if x_dir == Direction::Still {
        None
    } else {
        if let Some(dur) = recently_pressed.left_right {
            Some(dur + time.delta())
        } else {
            Some(time.delta())
        }
    };

    let req_direction = match (x_dir, y_dir) {
        (Direction::Still, Direction::Still) => Direction::Still,
        (_, Direction::Still) => x_dir,
        (Direction::Still, _) => y_dir,
        (x, y) => {
            let up_down_wins =
                recently_pressed.up_down.unwrap() < recently_pressed.left_right.unwrap();
            if up_down_wins {
                y
            } else {
                x
            }
        }
    };

    let direction = if let Some(dir) = keyboard_movement.get_queued_move() {
        keyboard_movement.try_queue_move(req_direction);
        dir
    } else if keyboard_movement.allowed_to_move(req_direction) {
        req_direction
    } else {
        if req_direction != last_direction.0 {
            keyboard_movement.try_queue_move(req_direction);
        }
        Direction::Still
    };

    match direction {
        Direction::Still => (),
        _ => {
            let (player_position, mut player_path) = player_query.single_mut();
            let map = levels.get_current();

            let current_tile = map.get_tile(player_position).unwrap();
            player_path.0.clear();
            if let Some(next) = map.get_neighbour(current_tile, req_direction) {
                if next.is_safe() {
                    last_direction.0 = direction;
                    player_path.0.push(next.position);
                }
            }
        }
    }
}

#[derive(Debug)]
struct LastKeyboardMovedDirection(Direction);

#[derive(Debug)]
/// Don't move the character
pub struct KeyboardInUse(pub bool);
