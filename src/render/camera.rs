use std::time::Duration;

use bevy::{prelude::*, render::camera::Camera2d};
use bevy_console::ConsoleCommand;
use bevy_inspector_egui::Inspectable;

use crate::{components::Player, ui::AddConvar};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(update_camera)
            .add_convar(ScaleCommand { scale: 0.35 })
            .add_system(update_scale)
            .insert_resource(RequestedCameraPosition {
                previous_position: Vec3::new(0., 0., 5.),
                requested_position: Vec3::new(0., 0., 5.),
                timer: Timer::new(Duration::from_millis(50), false),
            });
    }
}

struct RequestedCameraPosition {
    pub previous_position: Vec3,
    pub requested_position: Vec3,
    pub timer: Timer,
}

fn update_camera(
    mut requested: ResMut<RequestedCameraPosition>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut cam_query: Query<&mut Transform, (Without<Player>, With<Camera2d>)>,
    time: Res<Time>,
) {
    let cam_pos = &mut cam_query.single_mut().translation;

    if let Ok(ply_pos) = player_query.get_single() {
        // *cam_pos = ply_pos.translation;
        requested.previous_position = *cam_pos;

        let trans = &ply_pos.translation;
        requested.requested_position = *trans;

        requested.timer.reset();
    }

    if !requested.timer.finished() {
        requested.timer.tick(time.delta());

        let percent_done = requested.timer.percent();

        let new = requested
            .previous_position
            .lerp(requested.requested_position, percent_done);

        *cam_pos = new;
    }
}

fn update_scale(
    mut cam_query: Query<&mut OrthographicProjection, With<Camera2d>>,
    scale: Res<ScaleCommand>,
) {
    if !scale.is_changed() {
        return;
    }

    let mut projection = cam_query.single_mut();

    projection.scale = scale.scale as f32;
}

fn setup(mut commands: Commands) {
    // let mut camera = OrthographicCameraBundle::new_2d();
    // camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));
    // commands.spawn_bundle(camera);

    commands.spawn_bundle(UiCameraBundle::default());
}

/// Adjust camera scale
#[derive(ConsoleCommand, Inspectable)]
#[console_command(name = "r_scale")]
pub struct ScaleCommand {
    /// The scale to set. Default is 1.
    scale: f64,
}
