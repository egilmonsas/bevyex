use std::error::Error;

use bevy::prelude::*;
use serde::Deserialize;

use crate::LOCAL_ZERO;

const COLOR: Color = Color::rgb(0.2, 0.2, 0.8);

#[derive(Component)]
struct JetPile;

#[derive(Debug, Deserialize)]
struct JetPileEntry {
    #[allow(unused)]
    nmr: i32,
    id: String,
    x: f32,
    #[serde(rename = "y")]
    z: f32,
    length_200: f32,
    length_150: f32,
    #[serde(rename = "z_ok_prod")]
    y_ok: f32,
    #[serde(rename = "z_uk_prod")]
    y_uk: f32,
}
fn read_data() -> Result<Vec<JetPileEntry>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("data/jetgroutdata.csv")?;

    let iter = rdr.deserialize();
    let mut out: Vec<JetPileEntry> = vec![];
    for line in iter {
        let record: Result<JetPileEntry, csv::Error> = line;
        let entry: Option<JetPileEntry> = match record {
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
pub fn spawn_jet_pile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let data = read_data().unwrap();

    for entry in data {
        let c = Vec3::new(-entry.x, entry.y_ok, entry.z) - LOCAL_ZERO;
        let height_200 = entry.length_200;
        let height_150 = entry.length_150;

        if height_200 > 0.0 {
            commands
                .spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cylinder {
                            radius: 1.0,
                            height: height_200,
                            resolution: 16,
                            segments: 1,
                        })),
                        material: materials.add(COLOR.into()),
                        transform: Transform::from_translation(c - height_200 / 2.0 * Vec3::Y),
                        ..default()
                    },
                    JetPile,
                ))
                .insert(Name::new(format!("{}-Ø2000", entry.id)));
        }
        if height_150 > 0.0 {
            commands
                .spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cylinder {
                            radius: 0.75,
                            height: height_150,
                            resolution: 16,
                            segments: 1,
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: COLOR,
                            ..default()
                        }),
                        transform: Transform::from_translation(c + height_150 / 2.0 * Vec3::Y),
                        ..default()
                    },
                    JetPile,
                ))
                .insert(Name::new(format!("{}-Ø1500", entry.id)));
        }
    }
}
