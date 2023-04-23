use bevy::prelude::*;
use bevy_basic_camera::CameraControllerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevyex::{
    camera::spawn_camera, injection::spawn_injection_drilling, jet_pile::spawn_jet_pile,
    lighting::*, sheet_pile::*, world::spawn_world,
};

pub const WINDOWWIDTH: i32 = 1280;
pub const WINDWHEIGHT: i32 = 1024;
pub const BACKGROUNDCOLOR: Color = Color::rgb(0.1, 0.1, 0.2);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUNDCOLOR))
        // Startup Systems
        .add_startup_systems((
            spawn_camera,
            spawn_world,
            spawn_sun,
            spawn_jet_pile,
            spawn_injection_drilling,
        ))
        .add_startup_system(modify_window)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(SheetPilePlugin)
        .add_plugin(CameraControllerPlugin)
        .register_type::<SheetPile>()
        .run();
}
fn modify_window(mut windows: Query<&mut Window>) {
    let mut window = windows.get_single_mut().unwrap();

    window.set_maximized(true);
    window.title = "E18-Ramstadsletta, Eksempel".to_string();
}
