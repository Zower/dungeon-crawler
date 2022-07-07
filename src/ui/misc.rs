//! Various UI functions
use crate::entity::{Health, HealthText, Player};
use bevy::prelude::*;
pub struct MiscPlugin;

impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_health);
    }
}

fn update_health(
    mut health_query: Query<&Health, With<Player>>,
    mut text_query: Query<&mut Text, With<HealthText>>,
) {
    let player_health = health_query.single_mut();
    let mut text = text_query.single_mut();
    text.sections[0].value = player_health.0.to_string();
}
