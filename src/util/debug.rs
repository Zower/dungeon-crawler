use bevy::prelude::*;
use bevy_inspector_egui::{
    widgets::{InspectorQuery, ResourceInspector},
    Inspectable, InspectorPlugin,
};

use crate::components::Player;

use crate::render::ScaleCommand;

#[derive(Inspectable, Default)]
struct Data {
    #[inspectable(despawnable = false)]
    player: InspectorQuery<Entity, With<Player>>,
    clear_color: ResourceInspector<ClearColor>,
    r_scale: ResourceInspector<ScaleCommand>,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InspectorPlugin::<Data>::new());
    }
}
