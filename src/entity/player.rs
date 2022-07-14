use bevy::prelude::{Component, Deref, DerefMut};
use bevy_ecs_tilemap::TilePos;

/// The player entity
#[derive(Debug, Component, Default)]
pub struct Player;

#[derive(Debug, Component)]
pub struct Blob;

#[derive(Debug, Component, Clone, Copy, Deref, DerefMut, Default)]
pub struct PassiveTilePos(pub TilePos);
