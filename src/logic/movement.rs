use crate::{
    entity::PassiveTilePos,
    util::{tile_from_trans, PlayerQuery},
    ActiveState, GameState,
};

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::{plugin::RapierConfiguration, prelude::Velocity};
use iyes_loopless::prelude::*;
use leafwing_input_manager::{plugin::InputManagerPlugin, prelude::ActionState, Actionlike};
use strum::EnumString;

#[derive(Actionlike, Debug, Component, Clone, Copy, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum MovementAction {
    Up,
    Down,
    Left,
    Right,
}

/// Handles all movement
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<MovementAction>::default())
            // .add_system_to_stage(GameStage::Input, update_player_velocity)
            // .add_system(update_tilepos_transforms)
            .add_system(
                update_player_velocity
                    .run_in_state(ActiveState::Playing)
                    .run_in_state(GameState::FreeRoam),
            )
            // .add_enter_system(PauseState::Paused, set_vel_zero)
            .add_enter_system(ActiveState::Paused, pause_physics)
            .add_exit_system(ActiveState::Paused, resume_physics)
            // )
            // .add_system(move_ent_with_transform.after(update_player_velocity))
            //TODO after
            .add_system(update_player_tilepos);
    }
}

fn update_player_tilepos(mut query: Query<(&Transform, &mut PassiveTilePos), Changed<Transform>>) {
    for (transform, mut pos) in query.iter_mut() {
        **pos = tile_from_trans(&transform.translation.xy());
    }

    //     info!("Player tilepos: {:?}", pos);
}

fn update_player_velocity(
    mut player_query: PlayerQuery<(&mut Velocity, &ActionState<MovementAction>)>,
) {
    let (mut vel, action_state) = player_query.single_mut();

    let speed = 130.;

    let mut input = Vec2::ZERO;

    if action_state.pressed(MovementAction::Up) {
        // input.y += speed * TILE_SIZE * time.delta_seconds();
        input.y += 1.;
    }
    if action_state.pressed(MovementAction::Left) {
        // input.x -= speed * TILE_SIZE * time.delta_seconds();
        input.x -= 1.;
    }
    if action_state.pressed(MovementAction::Down) {
        // input.y -= speed * TILE_SIZE * time.delta_seconds();
        input.y -= 1.;
    }
    if action_state.pressed(MovementAction::Right) {
        // input.x += speed * TILE_SIZE * time.delta_seconds();
        input.x += 1.;
    }

    if input.length() != 0. {
        input /= input.length();
    }

    vel.linvel = input * speed;
}

fn pause_physics(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = false;
}

fn resume_physics(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = true;
}

// fn move_ent_with_transform(
//     mut vel_query: Query<(&mut Velocity, &mut Transform)>,
//     // map_query: Query<&TilePos, With<Wall>>,
// ) {
//     for (velocity, mut transform) in vel_query.iter_mut() {
//         // let potential = Vec3::new(
//         //     transform.translation.x + velocity.x,
//         //     transform.translation.y + velocity.y,
//         //     transform.translation.z,
//         // );

//         // let new_tile = tile_from_trans(&potential);

//         // if !map_query.iter().any(|&tile_pos| tile_pos == new_tile) {
//         transform.translation.x += velocity.x;
//         transform.translation.y += velocity.y;
//         // }

//         // transform.translation.x += velocity.x;
//         // transform.translation.y += velocity.y;
//     }
// }
