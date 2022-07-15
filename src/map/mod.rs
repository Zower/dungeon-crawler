//! Modules relating to the levels of the game
mod builder;
mod common;
mod fov;
mod tile;

use std::cmp::Ordering;

use bevy_ecs_tilemap::{Map, MapQuery, Tile, TilePos};
use bevy_rapier2d::prelude::Collider;
pub use builder::*;
pub use common::*;
pub use fov::*;
pub use tile::*;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{PassiveTilePos, Player},
    util::trans_from_tile,
    ActiveState, GameState,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::GeneratingMap, setup_map)
            .add_system(
                spawn_colliders_for_tiles
                    .run_in_state(ActiveState::Playing)
                    .run_not_in_state(GameState::GeneratingMap),
            )
            .add_system(
                paint_map
                    .run_in_state(ActiveState::Playing)
                    .run_not_in_state(GameState::GeneratingMap)
                    .after(FovCalculationLabel),
            );
    }
}

fn paint_map(
    mut tiles: Query<(&mut TilePaint, &mut Tile, &TilePos)>,
    player_q: Query<Entity, Changed<PassiveTilePos>>,
    mut map: MapQuery,
) {
    if player_q.get_single().is_err() {
        return;
    }

    for (mut paint, mut tile, pos) in tiles.iter_mut() {
        tile.visible = true;
        match *paint {
            TilePaint::CursorDraw(color) => tile.color = color,
            TilePaint::Visible => tile.color = Color::WHITE,
            TilePaint::PreviouslySeen => tile.color = Color::GRAY,
            TilePaint::Invisible => tile.visible = false,
        }

        map.notify_chunk_for_tile(*pos, 0u16, 0u16);

        if *paint != TilePaint::Invisible {
            *paint = TilePaint::PreviouslySeen;
        }

        // commands.entity(entity).remove()
    }
    // commands.en

    // for pos in &fov.tiles {
    //     let ent = map.get_tile_entity(*pos, 0, 0).unwrap();
    //     if let Ok(mut tile) = floor_q.get_mut(ent) {
    //         tile.visible = true;
    //         map.notify_chunk_for_tile(*pos, 0u16, 0u16);
    //     } else if let Ok((_, mut tile)) = wall_q.get_mut(ent) {
    //         tile.visible = true;
    //         map.notify_chunk_for_tile(*pos, 0u16, 0u16);
    //     }
    // }
}

fn spawn_colliders_for_tiles(
    player: Query<&PassiveTilePos, (With<Player>, Changed<PassiveTilePos>)>,
    mut map: MapQuery,
    mut tiles: Query<(Entity, &TilePos), With<Wall>>,
    mut commands: Commands,
) {
    let player = player.get_single();

    if player.is_err() {
        return;
    }

    for (tile, _) in tiles.iter() {
        commands
            .entity(tile)
            .remove::<Collider>()
            .remove::<Transform>()
            .remove::<GlobalTransform>();
    }

    for tile in map.get_tile_neighbors(**player.unwrap(), 0, 0) {
        if let Ok(tile) = tile {
            if let Ok((_, pos)) = tiles.get_mut(tile) {
                let transform = TransformBundle::from_transform(Transform::from_translation(
                    Vec3::from((trans_from_tile(pos), 0.)),
                ));
                commands
                    .entity(tile)
                    .insert_bundle(transform)
                    .insert(Collider::cuboid(8., 8.));
            }
        }
    }
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    //TODO: should this be here?
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let tiles = asset_server.load("tiles/tiles.png");

    // Creates a new layer builder with a layer entity.
    let (layer_builder, room) = MapBuilder::default().build(&mut commands);

    // Builds the layer.
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, tiles);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, 0u16, layer_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(0., 0., 0.))
        .insert(GlobalTransform::default());

    commands.spawn().insert(room);

    commands.insert_resource(NextState(GameState::FreeRoam))
}

impl PartialOrd for TilePaint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (TilePaint::CursorDraw(_), TilePaint::CursorDraw(_)) => Some(Ordering::Equal),
            (TilePaint::CursorDraw(_), _) => Some(Ordering::Greater),
            // (PaintPriority::CursorDraw(_), PaintPriority::PreviouslySeen) => todo!(),
            // (PaintPriority::CursorDraw(_), PaintPriority::Invisible) => todo!(),
            (TilePaint::Visible, TilePaint::Visible) => Some(Ordering::Equal),
            (TilePaint::Visible, TilePaint::CursorDraw(_)) => Some(Ordering::Less),
            (TilePaint::Visible, _) => Some(Ordering::Greater),
            // (PaintPriority::Visible, PaintPriority::Invisible) => todo!(),
            (TilePaint::PreviouslySeen, TilePaint::PreviouslySeen) => Some(Ordering::Equal),
            (TilePaint::PreviouslySeen, TilePaint::CursorDraw(_)) => Some(Ordering::Less),
            (TilePaint::PreviouslySeen, TilePaint::Visible) => Some(Ordering::Less),
            (TilePaint::PreviouslySeen, TilePaint::Invisible) => Some(Ordering::Greater),
            (TilePaint::Invisible, TilePaint::Invisible) => Some(Ordering::Equal),
            (TilePaint::Invisible, _) => Some(Ordering::Less),
            // (PaintPriority::Invisible, PaintPriority::Visible) => todo!(),
            // (PaintPriority::Invisible, PaintPriority::PreviouslySeen) => todo!(),
        }
    }
}
