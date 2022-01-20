use std::time::Duration;

use crate::{level::Point, Blob, Level, Player, WalkPath};

use bevy::{core::FixedTimestep, prelude::*};

/// Handles player movement
/// Currently gets the players WalkPath and executes one move every 0.09 seconds.
pub struct MovementPlugin;

pub const MOVEMENT_STEP: u64 = 90;

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
            .insert_resource(PlayerMovement {
                last_position: Point::new(1,1),
                timer: Timer::new(
                Duration::from_millis(MOVEMENT_STEP),
                false),
                locked: false,
            }
            );
    }
}

pub struct PlayerMovement {
    last_position: Point,
    timer: Timer,
    locked: bool,
} 

/// Moves player one tile, if requested
fn move_player(
    levels: Res<Level>,
    mut query: QuerySet<(
        QueryState<(&mut WalkPath, &mut Transform, &mut Point), With<Player>>,
        QueryState<(&mut Transform, &mut Point), With<Blob>>,
    )>,
    mut movement: ResMut<PlayerMovement>,
    time: Res<Time>,
) {
    let mut q0 = query.q0();
    let (mut player_path, mut player_transform, mut player_position) = q0.single_mut();
    if !player_path.0.is_empty() && !movement.locked {
        let next_tile = player_path.0.remove(0);
        movement.last_position = *player_position;
        *player_position = next_tile;

        movement.timer.reset();
        movement.locked = true;
    } else if movement.locked {
        movement.timer.tick(time.delta());

        let tile_translation = levels
            .get_current()
            .get_tile(&player_position)
            .unwrap()
            .screen_position();

        let new = levels
            .get_current()
            .get_tile(&movement.last_position)
            .unwrap()
            .screen_position();

        *player_transform.translation = *Vec3::from((new.lerp(tile_translation, movement.timer.percent()), player_transform.translation.z));

        if movement.timer.finished() {
            movement.locked = false;

            let last = movement.last_position;
            movement.last_position = *player_position;

            let mut q1 = query.q1();
            let (mut blob_transform, mut blob_position) = q1.single_mut();
            blob_transform.translation.x = new.x;
            blob_transform.translation.y = new.y;

            *blob_position = last;
        }
    }
}
