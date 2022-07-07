use bevy::prelude::*;

use crate::{
    entity::Player,
    input::CurrentMousePosition,
    level::{Map, Point, Size, Tile},
};

#[derive(Debug, Component, Clone, Copy)]
pub struct Cursor {
    hovered: Option<Hovered>,
}

impl Cursor {
    pub fn new() -> Self {
        Self { hovered: None }
    }

    pub fn rect<'a>(&self, map: &'a Map, size: Size) -> Option<Vec<&'a Tile>> {
        debug_assert!(size.width % 2 == 0);
        debug_assert!(size.height % 2 == 0);
        return if let Some(hovered) = &self.hovered {
            // - self.pos, impl sub for point
            let initial_pos = Point::new(
                hovered.position.x - ((size.width as f32 - 2.) / 2.).ceil() as i32,
                hovered.position.y - ((size.height as f32 - 2.) / 2.).ceil() as i32,
            );

            let mut tiles = vec![];
            for x in initial_pos.x..initial_pos.x + size.width {
                for y in initial_pos.y..initial_pos.y + size.height {
                    let tile = map.get_tile(&Point::new(x, y));
                    if let Some(tile) = tile {
                        tiles.push(tile);
                    }
                }
            }

            Some(tiles)
        } else {
            None
        };
    }

    pub fn circle(&self, r: i32) -> Option<Vec<Point>> {
        return if let Some(hovered) = &self.hovered {
            Some(Self::draw_circle(&hovered.position, r))
        } else {
            None
        };
    }

    pub fn draw_circle(position: &Point, r: i32) -> Vec<Point> {
        let mut tiles = vec![];
        let mut i = 1;
        for y in (position.y - r)..=(position.y + r) {
            for x in (position.x - i)..=(position.x + i) {
                tiles.push(Point::new(x, y));
            }
            if y >= position.y + 1 {
                i -= 1;
            } else if y < position.y - 1 {
                i += 1;
            }
        }

        tiles
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Hovered {
    position: Point,
}

pub struct PlayerHoveredPlugin;

impl Plugin for PlayerHoveredPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(hovered_player_system);
    }
}

fn hovered_player_system(
    mut player_query: Query<&mut Cursor, With<Player>>,
    mouse_position: Res<CurrentMousePosition>,
) {
    let mut player_cursor = player_query.single_mut();
    player_cursor.hovered = mouse_position.position().map(|p| Hovered { position: p });
}
