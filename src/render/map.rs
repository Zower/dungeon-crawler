use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{
    entity::{PassiveTilePos, Player},
    level::Wall,
    tilemap::{MapBuilder, Rect2},
    util::trans_from_tile,
    ActiveState, GameState,
};

use iyes_loopless::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_map)
            .add_system(
                spawn_colliders_for_tiles
                    .run_in_state(ActiveState::Playing)
                    .run_not_in_state(GameState::GeneratingMap),
            )
            .add_system(set_texture_filters_to_nearest);
    }
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

// Required by ecs_tilemap. Idk why TBH.
pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}
