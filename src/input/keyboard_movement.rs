//! Keyboard movement.

use bevy::prelude::*;

use crate::{
    level::{Point, WalkPath},
    logic::Direction,
    Levels, Player,
};

/// Handles WASD movement
pub struct KeyboardMovementPlugin;

impl Plugin for KeyboardMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_player_direction);
    }
}

fn update_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
    levels: Res<Levels>,
    mut player_query: Query<(&Point, &mut WalkPath), With<Player>>,
) {
    let (player_position, mut player_path) = player_query.single_mut();
    let level = levels.current();

    let current_tile = level.get_tile(*player_position).unwrap();
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
            if let Some(next) = level.get_neighbour(current_tile, direction) {
                if next.is_safe() {
                    player_path.0.push(next.position);
                }
            }
        }
    }
}
