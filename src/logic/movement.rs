use std::time::Duration;

use crate::{level::Point, Blob, Level, Player, WalkPath};

use bevy::{core::FixedTimestep, prelude::*};

/// Handles player movement
/// Currently gets the players WalkPath and executes one move every 0.09 seconds.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_stage(
            //     "fixed_update",
            //     SystemStage::parallel()
            //         .with_run_criteria(FixedTimestep::step(0.09).with_label("movement"))
            // .with_system(move_player),
            // )
            .add_system(move_player)
            .insert_resource(MovePlayerTimer(Timer::new(
                Duration::from_millis(90),
                false,
            )));
    }
}

pub struct MovePlayerTimer(pub Timer);

/// Moves player one tile, if requested
fn move_player(
    levels: Res<Level>,
    mut query: QuerySet<(
        QueryState<(&mut WalkPath, &mut Transform, &mut Point), With<Player>>,
        QueryState<(&mut Transform, &mut Point), With<Blob>>,
    )>,
    mut timer: ResMut<MovePlayerTimer>,
    time: Res<Time>,
) {
    let mut q0 = query.q0();
    let (mut player_path, mut player_transform, mut player_position) = q0.single_mut();
    if !player_path.0.is_empty() {
        timer.0.tick(time.delta());
        let next_tile = player_path.0.get(0).unwrap();
        let tile_translation = levels
            .get_current()
            .get_tile(next_tile)
            .unwrap()
            .screen_position();

        let new = levels
            .get_current()
            .get_tile(&player_position)
            .unwrap()
            .screen_position()
            .lerp(tile_translation, timer.0.percent());

        player_transform.translation = Vec3::from((new, player_transform.translation.z));

        if timer.0.just_finished() {
            let blob_x = player_transform.translation.x;
            let blob_y = player_transform.translation.y;
            let new_blob_position = *player_position;

            let next_tile = player_path.0.remove(0);
            *player_position = next_tile;
            let mut q1 = query.q1();
            let (mut blob_transform, mut blob_position) = q1.single_mut();
            blob_transform.translation.x = blob_x;
            blob_transform.translation.y = blob_y;

            *blob_position = new_blob_position;
            timer.0.reset();
        }
    }
}
