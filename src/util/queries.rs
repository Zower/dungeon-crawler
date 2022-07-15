use bevy::prelude::*;

use crate::components::Player;

pub type PlayerQuery<'world, 'state, Q> = Query<'world, 'state, Q, With<Player>>;
