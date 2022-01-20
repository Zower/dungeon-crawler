use std::time::Duration;

use bevy::prelude::*;

use crate::entity::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup)
            .add_system(update_camera)
            .insert_resource(RequestedCameraPosition {
                previous_position: Vec3::new(0., 0., 5.),
                requested_position: Vec3::new(0., 0., 5.),
                timer: Timer::new(Duration::from_millis(100), false),
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
    mut cam_query: Query<(&bevy::render::camera::Camera, &mut Transform), Without<Player>>,
    time: Res<Time>,
) {
    let cam_pos = &mut cam_query
        .iter_mut()
        .find(|(cam, _)| cam.name == Some(String::from("camera_2d")))
        .unwrap()
        .1
        .translation;

    if let Ok(ply_pos) = player_query.get_single() {
        *cam_pos = ply_pos.translation;
    //     requested.previous_position = *cam_pos;

    //     let trans = &ply_pos.translation;
    //     requested.requested_position = *trans;

    //     requested.timer.reset();
    }
    // else {
    //     if !requested.timer.finished() {
    //         requested.timer.tick(time.delta());

    //         let percent_done = requested.timer.percent();

    //         let new = requested
    //             .previous_position
    //             .lerp(requested.requested_position, percent_done);

    //         *cam_pos = new;
    //     }
    // }
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));
    commands.spawn_bundle(camera);

    commands.spawn_bundle(UiCameraBundle::default());
}
