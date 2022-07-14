//! Various UI functions
use crate::{
    entity::{Health, HealthText},
    util::PlayerQuery,
    ActiveState, GameState,
};
use bevy::prelude::*;
use iyes_loopless::prelude::*;
pub struct MiscPlugin;

impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            update_health
                .run_in_state(ActiveState::Playing)
                .run_not_in_state(GameState::GeneratingMap),
        );
    }
}

fn update_health(
    mut health_query: PlayerQuery<&Health>,
    mut text_query: Query<&mut Text, With<HealthText>>,
) {
    let player_health = health_query.single_mut();
    let mut text = text_query.single_mut();
    text.sections[0].value = player_health.0.to_string();
}
