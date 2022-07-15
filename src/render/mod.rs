//! Rendering/drawing logic like lighting, map visibility.
//! TODO: Remove.

mod camera;

pub use camera::*;

use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use bevy_rapier2d::prelude::DebugRenderContext;

use crate::ui::AddConvar;

use self::camera::CameraPlugin;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin)
            .add_convar(DebugRenderConvar { on: false })
            .add_system(sync_debug_render_convar);
    }
}

/// Draws render debug information
#[derive(ConsoleCommand)]
#[console_command(name = "r_debug")]
struct DebugRenderConvar {
    /// Whether or not to draw debug information
    on: bool,
}

fn sync_debug_render_convar(
    convar: Res<DebugRenderConvar>,
    res: Option<ResMut<DebugRenderContext>>,
) {
    if let Some(mut res) = res {
        res.enabled = convar.on;
    }
}
