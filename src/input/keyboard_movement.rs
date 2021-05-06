//! Handles movement from WASD.

use bevy::prelude::*;

use crate::{logic::Direction, Levels, Player};
pub struct KeyboardMovementPlugin;

impl Plugin for KeyboardMovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_player_direction.system());
    }
}

fn update_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
    levels: Res<Levels>,
    mut query: Query<&mut Player>,
) {
    if let Some(current_level) = levels.current {
        let level = &levels.levels[current_level];

        if let Ok(mut player) = query.single_mut() {
            let current_tile = level.get_tile(player.current).unwrap();
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
                    player.path.0.clear();
                    if let Some(next) = level.get_neighbour(current_tile, direction) {
                        if next.is_safe() {
                            player.path.0.push(next.position);
                        }
                    }
                }
            }
        }
    }
}
