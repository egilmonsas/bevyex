use bevy::prelude::*;
use bevy_basic_camera::CameraController;

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(10.0, 100.0, 5.0),
            ..default()
        })
        .insert(CameraController::default())
        .insert(PointLightBundle {
            point_light: PointLight {
                intensity: 10000.0,
                range: 100.0,
                ..default()
            },
            ..default()
        });
}
