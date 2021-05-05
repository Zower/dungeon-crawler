//! Handles movement from WASD.

use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::{logic::Direction, Blob, Levels, Player};
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage(
            "fixed_update",
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(0.09).with_label("movement"))
                .with_system(move_player.system()),
        )
        .add_system(update_player_direction.system());
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

fn move_player(
    levels: Res<Levels>,
    mut query: QuerySet<(
        Query<(&mut Player, &mut Transform)>,
        Query<(&mut Blob, &mut Transform)>,
    )>,
) {
    if let Ok((mut player, mut player_transform)) = query.q0_mut().single_mut() {
        let blob_x = player_transform.translation.x;
        let blob_y = player_transform.translation.y;
        let blob_position = player.current;

        if !player.path.0.is_empty() {
            if let Some(current_level) = levels.current {
                let next_tile = player.path.0.remove(0);
                let new_translation = levels.levels[current_level]
                    .get_tile(next_tile)
                    .unwrap()
                    .screen_position();

                player.current = next_tile;
                player_transform.translation.x = new_translation.0.x as f32;
                player_transform.translation.y = new_translation.0.y as f32;

                if let Ok((mut blob, mut blob_transform)) = query.q1_mut().single_mut() {
                    blob_transform.translation.x = blob_x;
                    blob_transform.translation.y = blob_y;

                    blob.current = blob_position;
                }
            }
        }
    }
}
