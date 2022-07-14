use bevy::prelude::*;
use bevy_ecs_tilemap::{MapQuery, TilePos};
use iyes_loopless::prelude::*;

use crate::{entity::Player, input::CurrentMousePosition, ActiveState, GameState};

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
        for y in (position.1 - r)..=(position.1 + r) {
            for x in (position.0 - i)..=(position.0 + i) {
                tiles.push(TilePos(x, y));
            }
            if y >= position.1 + 1 {
                i -= 1;
            } else if y < position.1 - 1 {
                i += 1;
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
) {
    let mut player_cursor = player_query.single_mut();
    *player_cursor = TileCursor(**mouse_position);
}
