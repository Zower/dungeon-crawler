use bevy::prelude::Plugin;

pub struct TurnTicked;

struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<TurnTicked>();
    }
}
