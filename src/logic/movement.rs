use crate::{level::Point, Blob, Level, Player, WalkPath};

use bevy::{core::FixedTimestep, prelude::*};

/// Handles player movement
/// Currently gets the players WalkPath and executes one move every 0.09 seconds.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage(
            "fixed_update",
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(0.09).with_label("movement"))
                .with_system(move_player),
        );
    }
}

/// Moves player one tile, if requested
fn move_player(
    levels: Res<Level>,
    mut query: QuerySet<(
        QueryState<(&mut WalkPath, &mut Transform, &mut Point), With<Player>>,
        QueryState<(&mut Transform, &mut Point), With<Blob>>,
    )>,
) {
    let mut q0 = query.q0();
    let (mut player_path, mut player_transform, mut player_position) = q0.single_mut();

    if !player_path.0.is_empty() {
        let blob_x = player_transform.translation.x;
        let blob_y = player_transform.translation.y;
        let new_blob_position = *player_position;

        let next_tile = player_path.0.remove(0);
        let new_translation = levels
            .get_current()
            .get_tile(next_tile)
            .unwrap()
            .screen_position();

        *player_position = next_tile;
        player_transform.translation =
            Vec3::from((new_translation, player_transform.translation.z));
        let mut q1 = query.q1();
        let (mut blob_transform, mut blob_position) = q1.single_mut();
        blob_transform.translation.x = blob_x;
        blob_transform.translation.y = blob_y;

        *blob_position = new_blob_position;
    }
}
