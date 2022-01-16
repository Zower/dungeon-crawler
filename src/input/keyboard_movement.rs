//! Keyboard movement.

use std::time::Duration;

use bevy::prelude::*;

use crate::{
    level::{Point, WalkPath},
    logic::Direction,
    Level, Player,
};

/// Handles WASD movement
pub struct KeyboardMovementPlugin;

impl Plugin for KeyboardMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyboardInUse(false))
            .add_system(update_player_direction);
    }
}

fn update_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
    dont_move: Res<KeyboardInUse>,
    levels: Res<Level>,
    mut player_query: Query<(&Point, &mut WalkPath), With<Player>>,
) {
    if dont_move.0 {
        return;
    }

    let (player_position, mut player_path) = player_query.single_mut();
    let map = levels.get_current();

    let current_tile = map.get_tile(*player_position).unwrap();
    let mut direction = Direction::Still;
    if keyboard_input.pressed(KeyCode::W) {
        direction = Direction::Up;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction = Direction::Left;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction = Direction::Down;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction = Direction::Right;
    }

    match direction {
        Direction::Still => (),
        _ => {
            player_path.0.clear();
            if let Some(next) = map.get_neighbour(current_tile, direction) {
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
