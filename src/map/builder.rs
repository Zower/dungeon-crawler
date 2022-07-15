//! The game is played on a 'level', consisting of a set number of randomly generated maps.

use bevy_ecs_tilemap::prelude::*;
use core::panic;
use rand::Rng;
use std::cmp::{max, min, Ordering};
use std::ops::Range;

use crate::map::{Floor, TilePaint, Wall};
use crate::util::{CHUNK_SIZE, TILE_SIZE};

use bevy::prelude::*;

use super::{Room, TileRect};

/// Create tilemaps.
pub struct MapBuilder {
    /// The size of the map to be generated, in chunks.
    map_size: MapSize,
    /// Number of rooms that can be generated.
    nr_rooms: u32,
    /// How big rooms can be.
    room_size_range_x: Range<u32>,
    /// How big rooms can be.
    room_size_range_y: Range<u32>,
    // /// The depth of this level.
    // depth: usize,
}

impl MapBuilder {
    // pub fn new() -> Self {
    //     Self::default()
    // }

    // pub fn square(size: u32) -> Self {
    //     Self {
    //         map_size: MapSize(size, size),
    //         ..Default::default()
    //     }
    // }

    // /// Set the map size
    // pub fn size(&mut self, size: MapSize) -> &mut Self {
    //     self.map_size = size;
    //     self
    // }

    // /// Sets depth of level.
    // pub fn depth(&mut self, depth: usize) -> &mut Self {
    //     self.depth = depth;
    //     self
    // }

    // pub fn nr_rooms(&mut self, amount: u32) -> &mut Self {
    //     self.nr_rooms = amount;
    //     self
    // }

    // /// Sets possible size of rooms.
    // pub fn room_size(&mut self, x: Range<u32>, y: Range<u32>) -> &mut Self {
    //     self.room_size_range_x = x;
    //     self.room_size_range_y = y;
    //     self
    // }

    /// Build a random map.
    pub fn build(&self, commands: &mut Commands) -> (LayerBuilder<TileBundle>, Room) {
        debug!(
            "Generating map with size: x: {:?}, y: {:?}",
            self.map_size.0, self.map_size.1
        );

        let (mut layer_builder, _) = LayerBuilder::new(
            commands,
            LayerSettings::new(
                self.map_size,
                ChunkSize(CHUNK_SIZE, CHUNK_SIZE),
                TileSize(TILE_SIZE, TILE_SIZE),
                TextureSize(160., 160.),
            ),
            0u16,
            0u16,
        );

        layer_builder.set_all(TileBundle {
            tile: Tile {
                texture_index: 10,
                visible: false,
                ..Default::default()
            },
            ..Default::default()
        });

        let mut rng = rand::thread_rng();
        let mut rooms: Vec<TileRect> = Vec::new();

        for _ in 1..=self.nr_rooms {
            let w = rng.gen_range(self.room_size_range_x.clone());
            let h = rng.gen_range(self.room_size_range_y.clone());
            let x = rng.gen_range(1..self.map_size.0 * CHUNK_SIZE - w - 1);
            let y = rng.gen_range(1..self.map_size.1 * CHUNK_SIZE - h - 1);

            let new_room = TileRect::new(x, y, w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }

            if ok {
                debug!("Creating room: {new_room:?}");

                layer_builder.fill(
                    TilePos(new_room.x1, new_room.y1),
                    TilePos(new_room.x2, new_room.y2),
                    TileBundle {
                        tile: Tile {
                            texture_index: 6,
                            visible: false,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                )
            }

            if !rooms.is_empty() {
                // let new_pos = tile_from_trans(&new_room.center());
                // let prev_pos = tile_from_trans(&rooms[rooms.len() - 1].center());
                let new_pos = TilePos(new_room.center().x as u32, new_room.center().y as u32);
                let prev_pos = TilePos(
                    rooms[rooms.len() - 1].center().x as u32,
                    rooms[rooms.len() - 1].center().y as u32,
                );
                // let prev_pos = rooms[rooms.len() - 1].center();
                let mut action = |t| {
                    layer_builder
                        .set_tile(
                            t,
                            TileBundle {
                                tile: Tile {
                                    texture_index: 6,
                                    visible: false,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        )
                        .unwrap();
                };

                if rng.gen_range(0..2) == 1i32 {
                    apply_horizontal_tunnel(prev_pos.0, new_pos.1, prev_pos.1, &mut action);
                    apply_vertical_tunnel(prev_pos.1, new_pos.1, new_pos.0, &mut action);
                } else {
                    apply_vertical_tunnel(prev_pos.1, new_pos.1, new_pos.0, &mut action);
                    apply_horizontal_tunnel(prev_pos.0, new_pos.1, prev_pos.1, &mut action);
                }

                fn apply_horizontal_tunnel(
                    x1: u32,
                    x2: u32,
                    y: u32,
                    mut action: impl FnMut(TilePos),
                ) {
                    for x in min(x1, x2)..=max(x1, x2) {
                        action(TilePos(x, y))
                    }
                }

                fn apply_vertical_tunnel(
                    y1: u32,
                    y2: u32,
                    x: u32,
                    mut action: impl FnMut(TilePos),
                ) {
                    for y in min(y1, y2)..=max(y1, y2) {
                        action(TilePos(x, y))
                    }
                }
            }

            rooms.push(new_room);
        }

        layer_builder.for_each_tiles_mut(|ent, data| {
            if ent.is_none() {
                *ent = Some(commands.spawn().id())
            }

            let stolen = data.take().unwrap();

            match stolen.tile.texture_index {
                10 => commands
                    .entity(ent.unwrap())
                    .insert(Wall)
                    .insert(TilePaint::Invisible),
                // .insert(Collider::cuboid(16., 16.)),
                // ,
                // .insert(RigidBody::Fixed),
                6 => commands
                    .entity(ent.unwrap())
                    .insert(Floor)
                    .insert(TilePaint::Invisible),
                _ => panic!(),
            };

            *data = Some(stolen);
        });

        (layer_builder, Room(rooms.remove(0)))
    }
}

// impl Map {

//     pub fn update_visibility(&mut self, visible_tiles: &Vec<TilePos>) {
//         for TilePos in visible_tiles {
//             self.get_tile_mut(TilePos).unwrap().revealed = true;
//         }
//     }
// }

// More utility focused implementations
// impl Map {
//     pub fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
//         for x in min(x1, x2)..=max(x1, x2) {
//             let idx = Map::xy_idx(&TilePos::new(x, y), self.size.height);
//             if idx > 0 && idx < 80 * 50 {
//                 self.grid[idx as usize].surface = Surface::Floor;
//             }
//         }
//     }

//     pub fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
//         for y in min(y1, y2)..=max(y1, y2) {
//             let idx = Map::xy_idx(&TilePos::new(x, y), self.size.height);
//             if idx > 0 && idx < 80 * 50 {
//                 self.grid[idx as usize].surface = Surface::Floor;
//             }
//         }
//     }

//     /// A* implemented from <https://www.redblobgames.com/pathfinding/a-star/introduction.html>
//     pub fn a_star(&self, start: TilePos, goal: TilePos) -> Vec<TilePos> {
//         let mut frontier = BinaryHeap::new();
//         let start_TilePos = self.get_tile(&start).unwrap().position;
//         frontier.push(Reverse(TilePriority {
//             TilePos: &start_TilePos,
//             priority: 0,
//         }));

//         let mut came_from = HashMap::new();
//         came_from.insert(&start_TilePos, &start_TilePos); // First just TilePoss to itself

//         let mut cost_so_far = HashMap::new();
//         cost_so_far.insert(&start_TilePos, 0);

//         while let Some(current) = frontier.pop() {
//             // Pop returns PiecePriority, need the tile itself.
//             let current = current.0.TilePos;

//             if *current == goal {
//                 break;
//             }

//             for neighbour in self.get_neighbours(current) {
//                 // Neighbour is not in came_from
//                 let new_cost = cost_so_far.get(current).unwrap() + neighbour.cost;
//                 // if !came_from.contains_key(neighbour) {
//                 if !cost_so_far.contains_key(&neighbour.position)
//                     || new_cost < *cost_so_far.get(&neighbour.position).unwrap()
//                 {
//                     // Make sure its not a wall, etc.
//                     if neighbour.is_safe() {
//                         cost_so_far.insert(&neighbour.position, new_cost);
//                         let priority = new_cost + Map::heuristic(goal, neighbour.position);
//                         frontier.push(Reverse(TilePriority {
//                             TilePos: &neighbour.position,
//                             priority,
//                         }));
//                         came_from.insert(&neighbour.position, current);
//                     }
//                 }
//             }
//         }

//         // Came_from now includes the correct path, that can be traced back from came_from[goal]
//         let mut path = Vec::new();

//         let mut current = self.get_tile(&goal).unwrap().position;

//         while current != start_TilePos {
//             path.push(current);
//             current = *came_from.remove(&current).unwrap();
//         }

//         path.reverse();

//         path
//     }

//     fn heuristic(a: TilePos, b: TilePos) -> i32 {
//         i32::abs(a.x - b.x) + i32::abs(a.y - b.y)
//     }
// }

impl Default for MapBuilder {
    fn default() -> Self {
        Self {
            map_size: MapSize(2, 2),
            nr_rooms: 40,
            room_size_range_x: 4..8,
            room_size_range_y: 4..8,
            // depth: 0,
        }
    }
}

/// A wrapper struct that can be put into a priorityqueue, prioritized by cost, so that I dont have to implement ordering on Tile.
#[derive(Debug, PartialEq, Eq)]
struct TilePriority<'a> {
    tile_pos: &'a TilePos,
    priority: i32,
}
impl<'a> Ord for TilePriority<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<'a> PartialOrd for TilePriority<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
