//! Mouse movement.

use bevy::{input::mouse::MouseButtonInput, prelude::*};

use crate::{
    level::{Map, Point, WalkPath},
    Level, Player,
};

use super::CurrentMousePosition;

/// Checks if the player pressed mouse1
/// If they did use A* to find the path there.
pub fn mouse_update_grid(
    mut player_query: Query<(&mut WalkPath, &Point), With<Player>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    maps: Res<Level>,
    mouse_position: Res<CurrentMousePosition>,
) {
    for event in mouse_button_input_events.iter() {
        if event.state == bevy::input::ElementState::Pressed {
            if let Some(goal) = mouse_position.position() {
                debug!("Registered request to move to {goal:?}");

                let (mut player_path, player_position) = player_query.single_mut();
                let map = maps.get_current();
                if map.in_bounds(&goal) {
                    let goal = map.get_tile(&goal).unwrap();
                    if goal.is_safe() {
                        player_path.0 = Map::a_star(&map, *player_position, goal.position);
                    }
                }
            }
        }
    }
}
