use crate::{level::Path, Blob, Levels, Player};

use bevy::{core::FixedTimestep, prelude::*};
pub trait Moveable {
    fn path(&self) -> &Path;
}
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage(
            "fixed_update",
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(0.09).with_label("movement"))
                .with_system(move_player.system()),
        );
    }
}

/// Moves player one tile
fn move_player(
    levels: Res<Levels>,
    mut query: QuerySet<(
        Query<(&mut Player, &mut Transform)>,
        Query<(&mut Blob, &mut Transform)>,
    )>,
) {
    if let Ok((mut player, mut player_transform)) = query.q0_mut().single_mut() {
        let blob_x = player_transform.translation.x;
        let blob_y = player_transform.translation.y;
        let blob_position = player.current;

        if !player.path.0.is_empty() {
            if let Some(current_level) = levels.current {
                let next_tile = player.path.0.remove(0);
                let new_translation = levels.levels[current_level]
                    .get_tile(next_tile)
                    .unwrap()
                    .screen_position();

                player.current = next_tile;
                player_transform.translation.x = new_translation.0.x as f32;
                player_transform.translation.y = new_translation.0.y as f32;

                if let Ok((mut blob, mut blob_transform)) = query.q1_mut().single_mut() {
                    blob_transform.translation.x = blob_x;
                    blob_transform.translation.y = blob_y;

                    blob.current = blob_position;
                }
            }
        }
    }
}
