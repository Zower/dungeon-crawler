use crate::{
    entity::{Enemy, Health, Position},
    Blob, Levels, Player, WalkPath,
};

use bevy::{core::FixedTimestep, prelude::*};

/// Handles player movement
/// Currently gets the players WalkPath and executes one move every 0.09 seconds.
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

/// Moves player one tile, if requested
fn move_player(
    levels: Res<Levels>,
    mut query: QuerySet<(
        Query<(&mut WalkPath, &mut Transform, &mut Position), With<Player>>,
        Query<(&mut Transform, &mut Position), With<Blob>>,
    )>,
) {
    if let Ok((mut player_path, mut player_transform, mut player_position)) =
        query.q0_mut().single_mut()
    {
        let blob_x = player_transform.translation.x;
        let blob_y = player_transform.translation.y;
        let new_blob_position = player_position.0;

        if !player_path.0.is_empty() {
            let next_tile = player_path.0.remove(0);
            let new_translation = levels
                .current()
                .get_tile(next_tile)
                .unwrap()
                .screen_position();

            player_position.0 = next_tile;
            player_transform.translation.x = new_translation.0.x as f32;
            player_transform.translation.y = new_translation.0.y as f32;

            if let Ok((mut blob_transform, mut blob_position)) = query.q1_mut().single_mut() {
                blob_transform.translation.x = blob_x;
                blob_transform.translation.y = blob_y;

                blob_position.0 = new_blob_position;
            }
        }
    }
}
