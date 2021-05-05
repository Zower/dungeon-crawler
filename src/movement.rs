//! Handles everything related to player movement.

use bevy::core::FixedTimestep;
use bevy::prelude::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::{
    level::{GridPiece, Level, TileType},
    Blob, GridPosition, Levels, Path, Player,
};
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
#[derive(Debug, PartialEq, Eq, Hash)]
struct PieceDistance {
    piece: GridPiece,
    cost: i32,
}

pub fn a_star(level: &Level, start: GridPosition, goal: GridPosition) -> Vec<GridPosition> {
    let mut current = level
        .get_piece(start.x() as usize, start.y() as usize)
        .unwrap()
        .clone();

    // https://www.redblobgames.com/pathfinding/a-star/introduction.html
    let mut frontier = BinaryHeap::new();
    frontier.push(PieceDistance {
        cost: current.cost(),
        piece: current, // Unwrap, if this fails something else has gone wrong
    });
    let mut came_from = HashMap::new();
    came_from.insert(
        level
            .get_piece(start.x() as usize, start.y() as usize) // start
            .unwrap()
            .clone(), // Unwrap, if this fails something else has gone wrong
        level
            .get_piece(start.x() as usize, start.y() as usize) // end
            .unwrap()
            .clone(), // Unwrap, if this fails something else has gone wrong
    ); // First just points to itself

    while !frontier.is_empty() {
        current = frontier.pop().unwrap().piece;

        if current.grid_position() == goal {
            break;
        }

        for next in level.get_neighbours(current.gridx() as usize, current.gridy() as usize) {
            if !came_from.contains_key(next) {
                if next.tile_type() != TileType::Wall {
                    frontier.push(PieceDistance {
                        piece: next.clone(),
                        cost: next.cost(),
                    });
                    came_from.insert(next.clone(), current.clone());
                }
            }
        }
    }

    let begin = level
        .get_piece(start.x() as usize, start.y() as usize)
        .unwrap();
    let to_go = level
        .get_piece(goal.x() as usize, goal.y() as usize)
        .unwrap();

    let mut current = to_go;
    let mut path = Vec::new();

    while current != begin {
        path.push(current.grid_position().clone());
        current = &came_from[&current];
    }

    path.reverse();

    path
}

fn update_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
    levels: Res<Levels>,
    mut query: Query<(&mut Path, &GridPosition), With<Player>>,
) {
    let level = &levels.levels[levels.current];

    for (mut path, pos) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            path.0.clear();
            match level.get_safe_neighbour(pos.x() as usize, pos.y() as usize, Direction::Up) {
                Ok(neighbour) => path.0.push(neighbour.grid_position().clone()),
                Err(_) => (),
            }
        }
        if keyboard_input.pressed(KeyCode::A) {
            path.0.clear();
            match level.get_safe_neighbour(pos.x() as usize, pos.y() as usize, Direction::Left) {
                Ok(neighbour) => path.0.push(neighbour.grid_position().clone()),
                Err(_) => (),
            }
        }
        if keyboard_input.pressed(KeyCode::S) {
            path.0.clear();
            match level.get_safe_neighbour(pos.x() as usize, pos.y() as usize, Direction::Down) {
                Ok(neighbour) => path.0.push(neighbour.grid_position().clone()),
                Err(_) => (),
            }
        }
        if keyboard_input.pressed(KeyCode::D) {
            path.0.clear();
            match level.get_safe_neighbour(pos.x() as usize, pos.y() as usize, Direction::Right) {
                Ok(neighbour) => path.0.push(neighbour.grid_position().clone()),
                Err(_) => (),
            }
        }
    }
}

fn move_player(
    levels: Res<Levels>,
    // mut query_blob: Query<(&mut GridPosition, &mut Transform), With<Blob>>,
    // mut query_player: Query<(&mut GridPosition, &mut Transform, &mut Path), With<Player>>,
    mut query: QuerySet<(
        Query<(&mut GridPosition, &mut Transform, &mut Path), With<Player>>,
        Query<(&mut GridPosition, &mut Transform), With<Blob>>,
    )>,
) {
    let mut blob_x = 0.0;
    let mut blob_y = 0.0;
    let mut blob_gridx = 0;
    let mut blob_gridy = 0;

    for (mut pos, mut transform, mut path) in query.q0_mut().iter_mut() {
        blob_x = transform.translation.x;
        blob_y = transform.translation.y;
        blob_gridx = pos.x;
        blob_gridy = pos.y;
        if !path.0.is_empty() {
            pos.x = path.0[0].x();
            pos.y = path.0[0].y();
            path.0.remove(0);

            let trans = levels.levels[levels.current]
                .get_translation(pos.x as usize, pos.y as usize)
                .unwrap(); // If this fails, something has gone wrong somewhere else

            transform.translation.x = trans.0 as f32;
            transform.translation.y = trans.1 as f32;
        }
    }
    for (mut blob_pos, mut blob_transform) in query.q1_mut().iter_mut() {
        blob_transform.translation.x = blob_x;
        blob_transform.translation.y = blob_y;

        blob_pos.x = blob_gridx;
        blob_pos.y = blob_gridy;
    }
}

impl Ord for PieceDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for PieceDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
