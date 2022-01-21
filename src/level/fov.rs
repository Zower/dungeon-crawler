use std::ops::Add;

use bevy::prelude::*;
use dungeon_crawler_derive::Convar;

use crate::{
    entity::Player,
    input::{AddConvar, Convar},
    Level,
};

use super::{Map, Point, TileComponent};

/// What something can currently see.
#[derive(Debug, Component)]
pub struct FieldOfView {
    range: i32,
    tiles: Vec<Point>,
}

impl FieldOfView {
    pub fn new(range: i32) -> Self {
        Self {
            range,
            tiles: vec![],
        }
    }
}

pub struct FovPlugin;

impl Plugin for FovPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_convar_default::<GlobalVision>()
            .add_system(player_fov);
    }
}

#[derive(Debug, Default, Convar)]
struct GlobalVision(bool);

fn player_fov(
    global: Res<GlobalVision>,
    mut level: ResMut<Level>,
    mut player_query: Query<(&Point, &mut FieldOfView), With<Player>>,
    mut map_sprites_query: Query<(&mut Sprite, &mut Visibility, &TileComponent)>,
) {
    let map = level.get_current_mut();

    let (player_pos, mut player_fov) = player_query.get_single_mut().unwrap();
    update_visible(map, *player_pos, &mut player_fov);

    for (mut sprite, mut visibility, pos) in map_sprites_query.iter_mut() {
        if global.0 {
            visibility.is_visible = true;
        } else {
            if player_fov.tiles.contains(&pos.0) || pos.0 == *player_pos {
                visibility.is_visible = true;
                sprite.color = Color::WHITE;
            } else if map.get_tile(&pos.0).unwrap().revealed {
                sprite.color = Color::GRAY;
            } else {
                visibility.is_visible = false;
            }
        }
    }

    map.update_visibility(&player_fov.tiles);
}

pub fn update_visible(map: &Map, init_position: Point, fov: &mut FieldOfView) {
    fov.tiles.clear();
    for octant in 0..=7 {
        update_visible_octant(octant, map, init_position, fov);
    }
}

fn update_visible_octant(octant: u8, map: &Map, init_position: Point, fov: &mut FieldOfView) {
    let mut blocked_fov = BlockedFov::new();
    for row in 1..fov.range {
        if !map.in_bounds(
            &(init_position + BlockedFov::rotate_for_octant(OctantPoint::new(row, 0), octant)),
        ) {
            break;
        }

        for col in 0..=row {
            let pos =
                init_position + BlockedFov::rotate_for_octant(OctantPoint::new(row, col), octant);

            if !map.in_bounds(&pos) {
                break;
            }

            if !blocked_fov.is_fully_blocked() {
                let blocker = ShadowBorder::new(OctantPoint::new(row, col));

                let visible = !blocked_fov.is_not_viewable(&blocker);

                if visible {
                    fov.tiles.push(pos);
                }

                if visible && map.get_tile(&pos).unwrap().is_wall() {
                    blocked_fov.add_blocker(blocker);
                }
            }
        }
    }
}

pub struct BlockedFov {
    pub borders: Vec<ShadowBorder>,
}

impl BlockedFov {
    pub fn new() -> Self {
        Self { borders: vec![] }
    }

    pub fn add_blocker(&mut self, new_border: ShadowBorder) {
        let index = self
            .borders
            .iter()
            .position(|b| b.start >= new_border.start)
            .unwrap_or(self.borders.len());

        let len = self.borders.len();
        let (first, second) = self.borders.split_at_mut(index);

        let overlapping_prev = (index > 0 && first.last().unwrap().end > new_border.start)
            .then(|| first.last_mut().unwrap());

        let overlapping_next =
            (index < len && second[0].start < new_border.end).then(|| &mut second[0]);

        if let Some(next) = overlapping_next {
            if let Some(prev) = overlapping_prev {
                prev.end = next.end;
                self.borders.remove(index);
            } else {
                next.start = new_border.start;
            }
        } else {
            if let Some(prev) = overlapping_prev {
                prev.end = new_border.end;
            } else {
                self.borders.insert(index, new_border);
            }
        }
    }

    pub fn is_not_viewable(&self, tile_border: &ShadowBorder) -> bool {
        self.borders.iter().any(|t_b| t_b.contains(tile_border))
    }

    pub fn is_fully_blocked(&self) -> bool {
        self.borders.len() == 1 && self.borders[0].start == 0. && self.borders[0].end == 1.
    }

    /// Panics if octant is not in 0..=7
    pub fn rotate_for_octant(point: OctantPoint, octant: u8) -> OctantPoint {
        let (x, y) = match octant {
            0 => (point.0.y, point.0.x),
            1 => (point.0.x, point.0.y),
            2 => (point.0.x, -point.0.y),
            3 => (point.0.y, -point.0.x),
            4 => (-point.0.y, -point.0.x),
            5 => (-point.0.x, -point.0.y),
            6 => (-point.0.x, point.0.y),
            7 => (-point.0.y, point.0.x),
            _ => panic!("Octants may only be rotated 360 degrees, octant should be in range 0..=7"),
        };

        OctantPoint::new(x, y)
    }
}

/// Where 0 < start < end <= 1,
#[derive(Debug)]
pub struct ShadowBorder {
    start: f32,
    end: f32,
}

impl ShadowBorder {
    pub fn new(point: OctantPoint) -> Self {
        Self {
            start: point.0.y as f32 / (point.0.x + 2) as f32,
            end: (point.0.y + 1) as f32 / (point.0.x + 1) as f32,
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

/// A point in an FOV octant. Origin is 0, 0 no matter where the entity being calculated on is.
pub struct OctantPoint(Point);

impl OctantPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self(Point::new(x, y))
    }
}

impl Add<OctantPoint> for Point {
    type Output = Self;

    fn add(self, rhs: OctantPoint) -> Self::Output {
        Self::new(self.x + rhs.0.x, self.y + rhs.0.y)
    }
}
