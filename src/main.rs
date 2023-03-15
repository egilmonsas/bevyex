use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_basic_camera::{CameraController, CameraControllerPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevyex::{
    camera::spawn_camera, injection::spawn_injection_drilling, jet_pile::spawn_jet_pile,
    lighting::*, sheet_pile::*, world::spawn_world,
};

pub const WINDOWWIDTH: i32 = 1280;
pub const WINDWHEIGHT: i32 = 1024;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.2)))
        // Startup Systems
        .add_startup_system(asset_loading)
        .add_startup_system(spawn_world)
        .add_startup_system(spawn_sheet_pile)
        .add_startup_system(spawn_sheet_pile2)
        .add_startup_system(spawn_sheet_pile3)
        .add_startup_system(spawn_jet_pile)
        .add_startup_system(spawn_sun)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_injection_drilling)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(CameraControllerPlugin)
        .register_type::<SheetPile>()
        .run();
}
#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}
fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    });
}
fn flicker(
    mut commands: Commands,
    mut sheet_piles: Query<(&Transform, &mut SheetPile)>,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (transform, mut sheet_pile) in &mut sheet_piles {
        sheet_pile.shooting_timer.tick(time.delta());
        if sheet_pile.shooting_timer.just_finished() {
            let mut spawn_transform = *transform;
            spawn_transform.translation += Vec3::Y;
            commands
                .spawn(SceneBundle {
                    scene: bullet_assets.bullet_scene.clone(),
                    transform: spawn_transform,
                    ..default()
                })
                .insert(LifeTime {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                })
                .insert(Name::new("Bullet"));
        }
    }
}
fn flicker_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut LifeTime)>,
    time: Res<Time>,
) {
    for (entity, mut bullet) in &mut bullets {
        bullet.timer.tick(time.delta());
        if bullet.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct LifeTime {
    pub timer: Timer,
}
