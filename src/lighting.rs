use bevy::prelude::*;
use std::f32::consts::PI;

pub fn spawn_sun(mut commands: Commands) {
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 100000.0,
                ..default()
            },
            transform: Transform::from_xyz(1000.0, 1000.0, 1000.0).with_rotation(
                Quat::from_axis_angle(Vec3::new(1.0, 0.1, 0.), 5.0 / 4.0 * PI),
            ),
            ..default()
        })
        .insert(Name::new("Light"));
}
