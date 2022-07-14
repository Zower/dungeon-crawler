use bevy::prelude::{App, Plugin};

use super::console::ConsolePlugin;
// use super::console::ConsolePlugin;
use super::fps::FPSPlugin;
use super::misc::MiscPlugin;

/// Default UI Plugin, imports everything
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(ConsolePlugin)
            .add_plugin(FPSPlugin)
            .add_plugin(MiscPlugin)
            .add_plugin(ConsolePlugin);
    }
}
