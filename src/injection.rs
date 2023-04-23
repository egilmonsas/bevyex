use crate::LOCAL_ZERO;
use bevy::prelude::*;
use serde::Deserialize;
use std::error::Error;

const COLORS: [Color; 4] = [
    Color::rgba(0.2, 0.8, 0.2, 0.25),
    Color::rgba(0.3, 0.5, 0.2, 0.25),
    Color::rgba(0.5, 0.3, 0.2, 0.25),
    Color::rgba(0.8, 0.2, 0.2, 0.25),
];
const COLOR: Color = Color::rgba(0.1, 0.1, 0.1, 1.0);
#[derive(Debug, Deserialize)]
struct InjectionEntry {
    #[allow(unused)]
    line: String,
    id: String,
    x: f32,
    #[serde(rename = "z")]
    y: f32,
    #[serde(rename = "y")]
    z: f32,
    z_bot: f32,
    p50: f32,
    p50_vol: f32,
    p5: f32,
    p5_vol: f32,
}

fn get_color(usage: f32) -> Color {
    if usage > 500.0 {
        COLORS[3]
    } else if usage > 300.0 {
        COLORS[2]
    } else if usage > 100.0 {
        COLORS[1]
    } else {
        COLORS[0]
    }
}
fn read_data() -> Result<Vec<InjectionEntry>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("data/injectiondata.csv")?;

    let iter = rdr.deserialize();
    let mut out: Vec<InjectionEntry> = vec![];
    for line in iter {
        let record: Result<InjectionEntry, csv::Error> = line;
        let entry: Option<InjectionEntry> = match record {
            Ok(person) => Some(person),
            Err(err) => {
                eprintln!("Error parsing record: {err}");
                None
            }
        };
        if let Some(entry) = entry {
            out.push(entry);
        }
    }
    Ok(out)
}
#[derive(Component)]
struct Injection;

pub fn spawn_injection_drilling(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let data = read_data().unwrap();

    for entry in data {
        let height = entry.y - entry.z_bot;

        let c1 = Vec3::new(-entry.x, entry.y, entry.z) - LOCAL_ZERO;
        let c2 = Vec3::new(-entry.x, entry.p5, entry.z) - LOCAL_ZERO;
        let c3 = Vec3::new(-entry.x, entry.p50, entry.z) - LOCAL_ZERO;

        if height > 0.0 {
            // Draw borehole
            commands
                .spawn((PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cylinder {
                        radius: 0.2,
                        height,
                        resolution: 16,
                        segments: 1,
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: COLOR,
                        alpha_mode: AlphaMode::Opaque,
                        ..default()
                    }),
                    transform: Transform::from_translation(c1 - height / 2.0 * Vec3::Y),
                    ..default()
                },))
                .insert(Name::new(format!("{}-Berghull", entry.id)));
            // Draw injection result top
            commands
                .spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cylinder {
                            radius: 0.5,
                            height: 4.5,
                            resolution: 16,
                            segments: 1,
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: get_color(entry.p5_vol),
                            alpha_mode: AlphaMode::Add,
                            ..default()
                        }),
                        transform: Transform::from_translation(c2 - 4.5 / 2.0 * Vec3::Y),
                        ..default()
                    },
                    Injection,
                ))
                .insert(Name::new(format!("{}-Pakker 0.5m", entry.id)));
            // Draw injection result bottom
            let p50_h = (entry.p50 - entry.z_bot).max(0.1);
            commands
                .spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cylinder {
                            radius: 0.5,
                            height: p50_h,
                            resolution: 16,
                            segments: 1,
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: get_color(entry.p50_vol),
                            alpha_mode: AlphaMode::Add,
                            ..default()
                        }),
                        transform: Transform::from_translation(c3 - p50_h / 2.0 * Vec3::Y),
                        ..default()
                    },
                    Injection,
                ))
                .insert(Name::new(format!("{}-Pakker 5m", entry.id)));
        }
    }
}
