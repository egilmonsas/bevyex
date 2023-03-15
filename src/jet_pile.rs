use bevy::prelude::*;

const COLOR: Color = Color::rgb(0.2, 0.2, 0.8);

#[derive(Component)]
struct JetPile;

pub fn spawn_jet_pile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let read = raw
        .split('\n')
        .map(|line| {
            line.split(",")
                .map(|word| word.parse::<f32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<f32>>>();

    let c_ref = Vec3::new(-104017.675, 14.999, 1211459.201);
    for idx in 0..read.len() {
        let c = Vec3::new(-read[idx][0], read[idx][1], read[idx][2]) - c_ref;
        let height_200 = read[idx][3];
        let height_150 = read[idx][4];

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
                .insert(Name::new("JG Ø200"));
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
                .insert(Name::new("JG Ø150"));
        }
    }
}
const raw: &str = "103912.38,-3.90,1211435.05,0.00,13.60
103913.62,9.73,1211434.85,13.67,0.00
103915.33,9.70,1211435.49,14.50,0.00
103916.19,-3.45,1211436.51,0.00,13.15
103986.44,11.06,1211453.37,1.51,0.00
103985.04,11.11,1211453.10,1.26,0.00
103983.66,11.06,1211452.84,1.30,0.00
103982.22,11.08,1211452.54,1.37,0.00
103980.86,10.61,1211452.29,1.48,0.00
103979.46,10.56,1211451.95,1.57,0.00
103978.11,9.76,1211451.68,3.08,0.00
103976.74,7.06,1211451.40,2.49,1.00
103975.35,5.71,1211450.99,2.96,0.00
103973.95,4.07,1211450.74,1.32,4.06
103972.59,3.77,1211450.41,1.61,0.00
103971.20,4.03,1211450.11,1.03,4.18
103969.79,3.95,1211449.89,1.73,0.00
103968.43,2.70,1211449.50,1.51,5.60
103967.05,2.83,1211449.04,1.58,0.00
103965.52,2.87,1211448.86,1.40,5.52
103964.19,2.36,1211448.54,1.28,0.00
103962.80,2.19,1211448.32,1.59,6.34
103961.40,0.74,1211448.00,1.33,0.00
103959.96,0.37,1211447.75,1.58,8.07
103958.56,-0.06,1211447.42,1.57,0.00
103957.10,-0.20,1211447.16,1.47,8.91
103955.76,-0.33,1211446.88,1.36,0.00
103954.32,-0.62,1211446.55,1.15,9.35
103952.95,-0.75,1211446.27,1.25,0.00
103951.54,-0.59,1211445.97,1.17,9.29
103950.14,0.04,1211445.68,1.85,0.00
103948.77,0.17,1211445.42,1.73,8.55
103947.37,1.05,1211445.13,2.87,0.00
103945.94,0.18,1211444.85,1.50,8.67
103944.43,-0.77,1211444.47,1.03,0.00
103942.96,0.44,1211444.42,2.12,8.68
103941.48,-0.27,1211444.35,1.74,0.00
103940.01,-0.21,1211444.19,1.46,9.35
103938.48,-0.18,1211444.10,1.67,0.00
103936.99,-0.19,1211443.76,1.66,9.33
103935.63,-0.35,1211443.41,1.59,0.00
103934.18,-0.82,1211443.03,1.04,10.00
103932.75,-0.88,1211442.57,1.16,0.00
103931.33,-0.95,1211442.17,1.79,10.24
103929.87,-1.07,1211441.74,1.52,0.00
103928.47,-1.27,1211441.33,1.57,10.52
103927.03,-0.71,1211440.92,3.01,0.00
103925.65,-1.67,1211440.55,2.47,11.08
103924.25,-2.31,1211440.30,1.69,0.00
103922.69,-2.67,1211440.17,2.05,12.10
103921.29,-2.80,1211439.93,1.27,0.00
103919.89,-2.76,1211439.45,1.24,12.19
103918.50,-2.49,1211439.02,1.63,0.00
103917.12,-2.58,1211438.56,1.40,12.12
103915.82,14.20,1211438.31,18.30,0.00
103910.71,-0.18,1211436.63,3.87,0.00
103909.26,-0.35,1211436.31,4.61,10.04
103907.85,-2.85,1211435.95,1.74,0.00
103906.42,-2.76,1211435.62,2.21,12.42
103905.03,-2.34,1211435.35,2.21,0.00
103903.62,-1.75,1211435.09,1.41,10.69
103902.19,-1.59,1211434.81,1.59,0.00
103900.74,-1.35,1211434.49,1.55,10.23
103899.27,-0.81,1211434.28,1.33,0.00
103897.97,-0.71,1211434.08,1.42,9.73
103898.80,-0.69,1211432.75,2.04,0.00
103900.24,-1.08,1211432.95,1.72,0.00
103901.70,-1.58,1211433.25,1.34,0.00
103903.17,-1.67,1211433.52,1.43,0.00
103904.58,-1.97,1211433.75,2.09,0.00
103905.92,-4.80,1211434.00,1.01,0.00
103907.37,-2.78,1211434.35,1.35,0.00
103908.85,-2.79,1211434.69,1.60,0.00
103910.25,-0.10,1211435.09,1.97,0.00
103911.69,-2.78,1211435.40,1.29,0.00
103916.73,-2.81,1211436.96,1.24,0.00
103918.14,-2.48,1211437.36,1.22,0.00
103919.49,-2.59,1211437.81,1.84,0.00
103920.88,-2.76,1211438.28,1.15,0.00
103922.28,-2.73,1211438.56,1.38,0.00
103923.69,-2.56,1211438.95,1.37,0.00
103925.22,-2.22,1211438.98,1.93,0.00
103926.63,-0.73,1211439.27,2.15,0.00
103928.03,-1.02,1211439.72,1.73,0.00
103929.43,-1.11,1211440.17,1.40,0.00
103930.89,-1.01,1211440.60,1.13,0.00
103932.33,-0.93,1211440.95,1.14,0.00
103933.77,-0.52,1211441.44,1.06,0.00
103935.18,-0.17,1211441.81,1.02,0.00
103936.60,-0.53,1211442.11,1.10,0.00
103937.99,-0.16,1211442.62,1.28,0.00
103939.47,-0.24,1211442.77,1.13,0.00
103940.99,-0.25,1211442.88,1.60,0.00
103942.44,-0.27,1211442.97,1.71,0.00
103943.92,1.19,1211443.01,2.32,0.00
103945.44,0.14,1211443.16,1.65,0.00
103946.90,1.08,1211443.54,2.01,0.00
103948.33,1.09,1211443.81,2.17,0.00
103949.71,0.17,1211444.07,1.47,0.00
103951.10,0.29,1211444.37,1.79,0.00
103952.50,-0.75,1211444.66,1.39,0.00
103953.89,-0.75,1211444.94,1.04,0.00
103955.28,-0.34,1211445.29,1.57,0.00
103956.68,-0.32,1211445.51,1.00,0.00
103958.05,-0.17,1211445.85,1.29,0.00
103959.52,-0.03,1211446.13,1.13,0.00
103960.91,0.66,1211446.47,1.25,0.00
103962.34,2.37,1211446.70,3.18,0.00
103963.73,2.30,1211446.93,1.27,0.00
103965.12,2.87,1211447.18,1.49,0.00
103966.48,2.89,1211447.62,1.35,0.00
103968.02,2.83,1211447.84,1.66,0.00
103969.38,3.62,1211448.23,2.73,0.00
103970.72,3.95,1211448.52,1.30,0.00
103972.14,4.01,1211448.80,1.27,0.00
103973.52,3.81,1211449.09,1.50,0.00
103974.89,5.13,1211449.40,2.38,0.00
103976.31,5.73,1211449.77,1.61,0.00
103977.66,8.08,1211450.05,3.20,0.00
103979.03,9.76,1211450.31,2.72,0.00
103980.42,10.56,1211450.67,1.55,0.00
103981.76,10.62,1211450.92,1.11,0.00
103983.17,11.08,1211451.26,1.40,0.00
103984.60,11.06,1211451.50,1.38,0.00
103985.98,11.15,1211451.79,1.54,0.00";
