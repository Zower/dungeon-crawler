//! Various UI functions
use crate::entity::{Health, HealthText};
use bevy::prelude::*;
pub struct MiscPlugin;

impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_health);
    }
}

/// Updating all health text values
fn update_health(
    mut health_query: Query<(&Health, &Children)>,
    mut q_children: Query<&mut Text, With<HealthText>>,
) {
    for (health, children) in health_query.iter_mut() {
        for &child in children.iter() {
            if let Ok(mut text) = q_children.get_mut(child) {
                text.sections[0].value = health.0.to_string();
            }
        }
    }
}
