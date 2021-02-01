//! Handles everything related to player movement.

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::{level::GridPiece, level::Level, Blob, GridPosition, Levels, Path, Player};
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage(
            "fixed_update",
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(0.08).with_label("movement"))
                .with_system(move_player.system()),
        )
        .add_system(update_player_direction.system());
    }
}

pub fn a_star(level: &Level, start: GridPosition, goal: GridPosition) -> Vec<GridPiece> {
    let mut current: &GridPiece;

    // https://www.redblobgames.com/pathfinding/a-star/introduction.html
    let mut frontier = VecDeque::new();
    frontier.push_back(
        level
            .get_piece(start.x() as usize, start.y() as usize)
            .unwrap(), // Unwrap, if this fails something else has gone wrong
    );
    let mut came_from = HashMap::new();
    came_from.insert(
        level
            .get_piece(start.x() as usize, start.y() as usize) // start
            .unwrap(), // Unwrap, if this fails something else has gone wrong
        level
            .get_piece(start.x() as usize, start.y() as usize) // end
            .unwrap(), // Unwrap, if this fails something else has gone wrong
    ); // First just points to itself

    while !frontier.is_empty() {
        current = frontier.pop_front().unwrap();

        if current.gridx() == goal.x() && current.gridy() == goal.y() {
            break;
        }

        for next in level.get_neighbours(current.gridx() as usize, current.gridy() as usize) {
            if !came_from.contains_key(next) {
                frontier.push_back(next);
                came_from.insert(next, current);
            }
        }
    }

    let begin = level
        .get_piece(start.x() as usize, start.y() as usize)
        .unwrap();
    let to_go = level
        .get_piece(goal.x() as usize, goal.y() as usize)
        .unwrap();

    current = to_go;
    let mut path = Vec::new();

    while current != begin {
        path.push(current.clone());
        current = came_from[current];
    }

    path.reverse();

    path
}

fn update_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
    levels: Res<Levels>,
    mut query: Query<(&mut Path, &GridPosition), With<Player>>,
) {
    let level = levels.0.last().unwrap();

    for (mut path, pos) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            path.0.clear();
            match level.get_neighbour(pos.x() as usize, pos.y() as usize, Direction::Up) {
                Ok(dir) => path.0.push(dir),
                Err(_) => (),
            }
        }
        if keyboard_input.pressed(KeyCode::A) {
            path.0.clear();
            match level.get_neighbour(pos.x() as usize, pos.y() as usize, Direction::Left) {
                Ok(dir) => path.0.push(dir),
                Err(_) => (),
            }
        }
        if keyboard_input.pressed(KeyCode::S) {
            path.0.clear();
            match level.get_neighbour(pos.x() as usize, pos.y() as usize, Direction::Down) {
                Ok(dir) => path.0.push(dir),
                Err(_) => (),
            }
        }
        if keyboard_input.pressed(KeyCode::D) {
            path.0.clear();
            match level.get_neighbour(pos.x() as usize, pos.y() as usize, Direction::Right) {
                Ok(dir) => path.0.push(dir),
                Err(_) => (),
            }
        }
    }
}

fn move_player(
    levels: Res<Levels>,
    mut query_blob: Query<(&mut GridPosition, &mut Transform), With<Blob>>,
    mut query_player: Query<(&mut GridPosition, &mut Transform, &mut Path), With<Player>>,
) {
    for (mut pos, mut transform, mut path) in query_player.iter_mut() {
        if !path.0.is_empty() {
            let blob_x = transform.translation.x;
            let blob_y = transform.translation.y;
            let blob_gridx = pos.x;
            let blob_gridy = pos.y;

            pos.x = path.0[0].gridx();
            pos.y = path.0[0].gridy();
            path.0.remove(0);

            let trans = levels.0[0]
                .get_translation(pos.x as usize, pos.y as usize)
                .unwrap(); // If this fails, something has gone wrong somewhere else

            transform.translation.x = trans.0 as f32;
            transform.translation.y = trans.1 as f32;

            for (mut blob_pos, mut blob_transform) in query_blob.iter_mut() {
                blob_transform.translation.x = blob_x;
                blob_transform.translation.y = blob_y;

                blob_pos.x = blob_gridx;
                blob_pos.y = blob_gridy;
            }
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
