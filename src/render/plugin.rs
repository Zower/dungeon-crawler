use bevy::prelude::*;

use crate::input::AddConvar;

use super::{
    camera::CameraPlugin,
    map::{render_map, GlobalVision},
};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin)
            .add_convar_default::<GlobalVision>()
            .add_system(render_map);
    }
}
