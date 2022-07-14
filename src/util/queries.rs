use bevy::prelude::*;

use crate::entity::Player;

pub type PlayerQuery<'world, 'state, Q> = Query<'world, 'state, Q, With<Player>>;
