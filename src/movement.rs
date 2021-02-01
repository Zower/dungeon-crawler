use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::{level::GridPiece, level::Level, GridPosition, Levels, Path, Player};
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(MoveTimer(Timer::from_seconds(0.08, true))) // TODO: FixedTimestep?
            .add_system(update_player_direction.system())
            .add_system(move_player.system());
    }
}

pub fn a_star(level: &Level, start: GridPosition, goal: GridPosition) -> Vec<GridPiece> {
    let mut current: &GridPiece;

    let mut frontier = VecDeque::new();
    frontier.push_back(level.get_piece(start.x as usize, start.y as usize));
    let mut came_from = HashMap::new();
    came_from.insert(
        level.get_piece(start.x as usize, start.y as usize), // start
        level.get_piece(start.x as usize, start.y as usize), // end
    ); // First just points to itself

    while !frontier.is_empty() {
        current = frontier.pop_front().unwrap();

        if current.gridx() == goal.x && current.gridy() == goal.y {
            break;
        }

        for next in level.get_neighbours(current.gridx() as usize, current.gridy() as usize) {
            if !came_from.contains_key(next) {
                frontier.push_back(next);
                came_from.insert(next, current);
            }
        }
    }

    let begin = level.get_piece(start.x as usize, start.y as usize);
    let to_go = level.get_piece(goal.x as usize, goal.y as usize);

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
    mut query: Query<&mut MoveState, With<Player>>,
) {
    for mut move_state in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            move_state.0 = Direction::Up;
        }
        if keyboard_input.pressed(KeyCode::A) {
            move_state.0 = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::S) {
            move_state.0 = Direction::Down;
        }
        if keyboard_input.pressed(KeyCode::D) {
            move_state.0 = Direction::Right;
        }
        if keyboard_input.just_released(KeyCode::W)
            || keyboard_input.just_released(KeyCode::A)
            || keyboard_input.just_released(KeyCode::S)
            || keyboard_input.just_released(KeyCode::D)
        {
            move_state.0 = Direction::Still
        }
    }
}

fn move_player(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    levels: Res<Levels>,
    mut query: Query<(&mut GridPosition, &mut Transform, &mut Path, &MoveState), With<Player>>,
) {
    if !timer.0.tick(time.delta_seconds()).finished() {
        return;
    }

    for (mut pos, mut transform, mut path, move_state) in query.iter_mut() {
        match move_state.0 {
            Direction::Up => {
                pos.y += 1;
                path.0.clear()
            }
            Direction::Down => {
                pos.y -= 1;
                path.0.clear()
            }
            Direction::Left => {
                pos.x -= 1;
                path.0.clear()
            }
            Direction::Right => {
                pos.x += 1;
                path.0.clear()
            }
            Direction::Still => (),
        }

        for p in &path.0 {}

        if !path.0.is_empty() {
            pos.x = path.0[0].gridx();
            pos.y = path.0[0].gridy();
            path.0.remove(0);
        }

        //TODO: Only if changed
        let trans = levels.0[0].get_translation(pos.x as usize, pos.y as usize);

        transform.translation.x = trans.0 as f32;
        transform.translation.y = trans.1 as f32;
    }
}

pub struct MoveState(pub Direction);
struct MoveTimer(Timer);
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Still,
}
