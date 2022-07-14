use bevy::prelude::*;
use bevy_inspector_egui::{widgets::InspectorQuery, Inspectable, InspectorPlugin};

use crate::entity::Player;

#[derive(Inspectable, Default)]
struct Data {
    #[inspectable(despawnable = false)]
    player: InspectorQuery<Entity, With<Player>>,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InspectorPlugin::<Data>::new());
    }
}
