use std::ops::Add;

use bevy::prelude::*;
use bevy_ecs_tilemap::{MapQuery, Tile, TilePos};

use crate::{
    components::{PassiveTilePos, Player},
    ActiveState, GameState,
};

use iyes_loopless::prelude::*;

use super::{TilePaint, Wall};

/// What something can currently see.
#[derive(Debug, Component)]
pub struct FieldOfView {
    pub range: u32,
    pub tiles: Vec<TilePos>,
}

impl FieldOfView {
    pub fn new(range: u32) -> Self {
        Self {
            range,
            tiles: vec![],
        }
    }
}

#[derive(SystemLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub struct FovCalculationLabel;

pub struct FovPlugin;

impl Plugin for FovPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            player_fov
                .run_in_state(ActiveState::Playing)
                .run_not_in_state(GameState::GeneratingMap)
                .label(FovCalculationLabel),
        )
        .add_system(
            update_tile_paint
                .run_in_state(ActiveState::Playing)
                .run_not_in_state(GameState::GeneratingMap)
                .after(FovCalculationLabel),
        );
    }
}

fn update_tile_paint(
    mut player_query: Query<&FieldOfView, (With<Player>, Changed<PassiveTilePos>)>,
    mut map: MapQuery,
    mut tile_query: Query<&mut TilePaint>,
) {
    if let Ok(fov) = player_query.get_single_mut() {
        // for mut tile in tile_query.iter_mut() {
        //     if *tile == TilePaint::Visible {
        //         *tile = TilePaint::PreviouslySeen;
        //     }
        // }

        for tile in &fov.tiles {
            if let Ok(ent) = map.get_tile_entity(*tile, 0, 0) {
                let mut current = tile_query.get_mut(ent).unwrap();
                *current = current.greater_of(TilePaint::Visible)
            }
        }
    }
}

// TODO: Improve this and all subsequent functions greatly. Performance can certainly be improved.
fn player_fov(
    map: MapQuery,
    wall_q: Query<(Entity, &mut Tile), With<Wall>>,
    // mut level: ResMut<Level>,
    mut player_query: Query<
        (&PassiveTilePos, &mut FieldOfView),
        (With<Player>, Changed<PassiveTilePos>),
    >,
) {
    if let Ok((player_pos, mut player_fov)) = player_query.get_single_mut() {
        update_visible(map, **player_pos, &mut player_fov, wall_q);
    }
}

pub fn update_visible(
    mut map: MapQuery,
    init_position: TilePos,
    fov: &mut FieldOfView,
    wall_q: Query<(Entity, &mut Tile), With<Wall>>,
) {
    fov.tiles.clear();
    for octant in 0..=7 {
        update_visible_octant(octant, &mut map, init_position, fov, &wall_q);
    }
    fov.tiles.push(init_position);
}

fn update_visible_octant(
    octant: u8,
    map: &mut MapQuery,
    init_position: TilePos,
    fov: &mut FieldOfView,
    wall_q: &Query<(Entity, &mut Tile), With<Wall>>,
) {
    let mut blocked_fov = BlockedFov::new();
    for row in 1..fov.range as i32 {
        if map
            .get_tile_entity(
                init_position + BlockedFov::rotate_for_octant(OctantTilePos::new(row, 0), octant),
                0,
                0,
            )
            .is_err()
        {
            break;
        }

        for col in 0..=row {
            let pos =
                init_position + BlockedFov::rotate_for_octant(OctantTilePos::new(row, col), octant);

            if map.get_tile_entity(pos, 0, 0).is_err() {
                break;
            }

            if !blocked_fov.is_fully_blocked() {
                let blocker = ShadowBorder::new(OctantTilePos::new(row, col));

                let visible = !blocked_fov.is_not_viewable(&blocker);

                if visible {
                    fov.tiles.push(pos);
                }

                if visible && wall_q.contains(map.get_tile_entity(pos, 0, 0).unwrap()) {
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
    pub fn rotate_for_octant(pos: OctantTilePos, octant: u8) -> OctantTilePos {
        let (x, y) = match octant {
            0 => (pos.1, pos.0),
            1 => (pos.0, pos.1),
            2 => (pos.0, -pos.1),
            3 => (pos.1, -pos.0),
            4 => (-pos.1, -pos.0),
            5 => (-pos.0, -pos.1),
            6 => (-pos.0, pos.1),
            7 => (-pos.1, pos.0),
            _ => panic!("Octants may only be rotated 360 degrees, octant should be in range 0..=7"),
        };

        OctantTilePos::new(x, y)
    }
}

/// Where 0 < start < end <= 1,
#[derive(Debug)]
pub struct ShadowBorder {
    start: f32,
    end: f32,
}

impl ShadowBorder {
    pub fn new(pos: OctantTilePos) -> Self {
        Self {
            start: pos.1 as f32 / (pos.1 + 2) as f32,
            end: (pos.1 + 1) as f32 / (pos.1 + 1) as f32,
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

/// A TilePos in an FOV octant. Origin is 0, 0 no matter where the entity being calculated on is.
#[derive(Debug)]
pub struct OctantTilePos(pub i32, pub i32);

impl OctantTilePos {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
}

impl Add<OctantTilePos> for TilePos {
    type Output = Self;

    fn add(self, rhs: OctantTilePos) -> Self::Output {
        let x = if rhs.0.is_negative() {
            self.0.saturating_sub(rhs.0.wrapping_abs() as u32)
        } else {
            self.0.saturating_add(rhs.0 as u32)
        };

        let y = if rhs.1.is_negative() {
            self.1.saturating_sub(rhs.1.wrapping_abs() as u32)
        } else {
            self.1.saturating_add(rhs.1 as u32)
        };

        Self(x, y)
    }
}
