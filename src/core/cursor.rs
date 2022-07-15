use bevy::prelude::*;
use bevy_ecs_tilemap::{MapQuery, TilePos};
use iyes_loopless::prelude::*;

use crate::{
    components::Player,
    map::{Floor, TilePaint},
    ActiveState, GameState,
};

use super::mouse::CurrentMousePosition;

#[derive(Debug, Deref, DerefMut, Component, Clone, Copy)]
pub struct TileCursor(Option<TilePos>);

impl TileCursor {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn rect(&self, mut map: MapQuery, size: Size<u32>) -> Option<Vec<TilePos>> {
        // debug_assert!(size.width % 2 == 0);
        // debug_assert!(size.height % 2 == 0);
        return if let Some(hovered) = &self.0 {
            // - self.pos, impl sub for point
            let initial_pos = TilePos(
                hovered.0 - ((size.width as f32 - 2.) / 2.).ceil() as u32,
                hovered.1 - ((size.height as f32 - 2.) / 2.).ceil() as u32,
            );

            let mut tiles = vec![];
            for x in initial_pos.0..initial_pos.0 + size.width {
                for y in initial_pos.1..initial_pos.1 + size.height {
                    if map.get_tile_entity(TilePos(x, y), 0, 0).is_ok() {
                        tiles.push(TilePos(x, y));
                    }
                }
            }

            Some(tiles)
        } else {
            None
        };
    }

    pub fn circle(&self, r: u32) -> Option<Vec<TilePos>> {
        return if let Some(hovered) = &self.0 {
            Some(Self::draw_circle(hovered, r))
        } else {
            None
        };
    }

    pub fn draw_circle(position: &TilePos, r: u32) -> Vec<TilePos> {
        let mut tiles = vec![];
        let mut i = 1;
        for y in (position.1.saturating_sub(r))..=(position.1.saturating_add(r)) {
            for x in (position.0.saturating_sub(i))..=(position.0.saturating_add(i)) {
                tiles.push(TilePos(x, y));
            }
            if y >= position.1.saturating_add(1) {
                i = i.saturating_sub(1);
            } else if y < position.1.saturating_sub(1) {
                i = i.saturating_add(1);
            }
        }

        tiles
    }
}

//TODO: doesnt need to be a plugin
pub struct PlayerHoveredPlugin;

impl Plugin for PlayerHoveredPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            hovered_player_system
                .run_in_state(ActiveState::Playing)
                .run_not_in_state(GameState::GeneratingMap),
        );
    }
}

fn hovered_player_system(
    mut player_query: Query<&mut TileCursor, With<Player>>,
    mouse_position: Res<CurrentMousePosition>,
    mut map: MapQuery,
    mut tiles_query: Query<&mut TilePaint, With<Floor>>,
) {
    let mut player_cursor = player_query.single_mut();

    *player_cursor = TileCursor(**mouse_position);

    for tile in player_cursor.circle(2).unwrap_or(vec![]) {
        if let Ok(ent) = map.get_tile_entity(tile, 0, 0) {
            if let Ok(mut current) = tiles_query.get_mut(ent) {
                *current = current.greater_of(TilePaint::CursorDraw(Color::GREEN));
            }
        }
    }
}
