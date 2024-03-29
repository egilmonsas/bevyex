use bevy::prelude::*;

use crate::LOCAL_ZERO;

const HEIGHT: f32 = 0.3;
const WIDTH: f32 = 0.175;
const THICKNESS: f32 = 0.01;
const COLOR: Color = Color::rgb(0.8, 0.2, 0.2);

pub struct SheetPilePlugin;

impl Plugin for SheetPilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_sheet_pile)
            .add_startup_system(spawn_sheet_pile2)
            .add_startup_system(spawn_sheet_pile3)
            .register_type::<SheetPile>();
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SheetPile {}

struct SheetPileGeom {
    c_start: Vec3,
    c_end: Vec3,
    length: f32,
    leading: bool,
}
impl SheetPileGeom {
    fn leading_flip(&self) -> f32 {
        if self.leading {
            1.0
        } else {
            -1.0
        }
    }
    pub fn wall_ang(&self) -> f32 {
        (self.c_end.z - self.c_start.z + HEIGHT * self.leading_flip())
            .atan2(self.c_end.x - self.c_start.x)
    }
    pub fn vertices(&self) -> [Vec3; 4] {
        let c_2 = self.c_start
            + WIDTH
                * Vec3 {
                    x: self.wall_ang().cos(),
                    y: 0.0,
                    z: self.wall_ang().sin(),
                };
        let c_3 = self.c_end
            - WIDTH
                * Vec3 {
                    x: self.wall_ang().cos(),
                    y: 0.0,
                    z: self.wall_ang().sin(),
                };

        [self.c_start, c_2, c_3, self.c_end]
    }
}
pub fn spawn_sheet_pile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let read = RAW
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|word| word.parse::<f32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<f32>>>();

    let mut leading = false;
    for idx in 0..read.len() - 1 {
        let c_start = Vec3::new(-read[idx][0], read[idx][1], read[idx][2]) - LOCAL_ZERO;
        let c_end = Vec3::new(-read[idx + 1][0], read[idx + 1][1], read[idx + 1][2]) - LOCAL_ZERO;

        leading = !leading;
        let sheet_pile = SheetPileGeom {
            c_start,
            c_end,
            length: read[idx][3],
            leading,
        };
        let vertices = sheet_pile.vertices();
        for idxx in 0..vertices.len() - 1 {
            let c_prev = vertices[idxx];
            let c_next = vertices[idxx + 1];
            let coord_ang = (c_next.z - c_prev.z).atan2(c_next.x - c_prev.x);
            let diff = c_next - c_prev;
            let width = diff.length();

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box {
                        min_x: 0.0,
                        max_x: width,
                        min_y: -sheet_pile.length,
                        max_y: 0.0,
                        min_z: -THICKNESS / 2.0,
                        max_z: THICKNESS / 2.0,
                    })),
                    material: materials.add(COLOR.into()),
                    transform: Transform {
                        translation: c_prev,
                        rotation: Quat::from_rotation_y(-coord_ang),
                        scale: Vec3::ONE,
                    },
                    ..default()
                })
                .insert(Name::new("Spuntnål"));
        }
    }
}
pub fn spawn_sheet_pile2(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let read = RAW2
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|word| word.parse::<f32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<f32>>>();

    let mut leading = true;
    for idx in 0..read.len() - 1 {
        let c_start = Vec3::new(-read[idx][0], read[idx][1], read[idx][2]) - LOCAL_ZERO;
        let c_end = Vec3::new(-read[idx + 1][0], read[idx + 1][1], read[idx + 1][2]) - LOCAL_ZERO;

        leading = !leading;
        let sheet_pile = SheetPileGeom {
            c_start,
            c_end,
            length: read[idx][3],
            leading,
        };
        let vertices = sheet_pile.vertices();
        for idxx in 0..vertices.len() - 1 {
            let c_prev = vertices[idxx];
            let c_next = vertices[idxx + 1];
            let coord_ang = (c_next.z - c_prev.z).atan2(c_next.x - c_prev.x);
            let diff = c_next - c_prev;
            let width = diff.length();

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box {
                        min_x: 0.0,
                        max_x: width,
                        min_y: -sheet_pile.length,
                        max_y: 0.0,
                        min_z: -THICKNESS / 2.0,
                        max_z: THICKNESS / 2.0,
                    })),
                    material: materials.add(COLOR.into()),
                    transform: Transform {
                        translation: c_prev,
                        rotation: Quat::from_rotation_y(-coord_ang),
                        scale: Vec3::ONE,
                    },
                    ..default()
                })
                .insert(Name::new("Spuntnål"));
        }
    }
}
pub fn spawn_sheet_pile3(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let read = RAW3
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|word| word.parse::<f32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<f32>>>();

    let mut leading = false;
    for idx in 0..read.len() - 1 {
        let c_start = Vec3::new(-read[idx][0], read[idx][1], read[idx][2]) - LOCAL_ZERO;
        let c_end = Vec3::new(-read[idx + 1][0], read[idx + 1][1], read[idx + 1][2]) - LOCAL_ZERO;

        leading = !leading;
        let sheet_pile = SheetPileGeom {
            c_start,
            c_end,
            length: read[idx][3],
            leading,
        };
        let vertices = sheet_pile.vertices();
        for idxx in 0..vertices.len() - 1 {
            let c_prev = vertices[idxx];
            let c_next = vertices[idxx + 1];
            let coord_ang = (c_next.z - c_prev.z).atan2(c_next.x - c_prev.x);
            let diff = c_next - c_prev;
            let width = diff.length();

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box {
                        min_x: 0.0,
                        max_x: width,
                        min_y: -sheet_pile.length,
                        max_y: 0.0,
                        min_z: -THICKNESS / 2.0,
                        max_z: THICKNESS / 2.0,
                    })),
                    material: materials.add(COLOR.into()),
                    transform: Transform {
                        translation: c_prev,
                        rotation: Quat::from_rotation_y(-coord_ang),
                        scale: Vec3::ONE,
                    },
                    ..default()
                })
                .insert(Name::new("Spuntnål"));
        }
    }
}
const RAW: &str = "104017.675,14.999,1211459.201,2.8
104016.985,15.061,1211458.756,3.4
104016.183,14.996,1211458.929,4.28
104015.433,15.006,1211458.557,4.94
104014.605,14.986,1211458.643,4.34
104013.894,15.009,1211458.205,4.2
104013.089,14.982,1211458.361,4.21
104012.404,15.002,1211457.889,4.25
104011.584,14.983,1211458.005,4.13
104010.867,15.007,1211457.577,3.95
104010.055,14.986,1211457.716,4.25
104009.324,14.999,1211457.323,4.27
104008.504,14.974,1211457.433,3.85
104007.78,14.992,1211457.026,3.85
104006.965,14.987,1211457.12,4.09
104006.274,14.996,1211456.655,4.09
104005.445,15.01,1211456.724,4.55
104004.725,15.004,1211456.317,4.51
104003.915,14.998,1211456.48,5.22
104003.209,15.004,1211456.014,5.22
104002.409,15.008,1211456.132,4.98
104001.683,14.997,1211455.697,4.95
104000.887,14.979,1211455.812,4.82
104000.105,14.99,1211455.448,4.78
103999.311,14.979,1211455.559,5.17
103998.573,14.982,1211455.113,5.16
103997.787,14.984,1211455.223,5.37
103997.049,14.98,1211454.801,5.35
103996.25,14.971,1211454.93,5.52
103995.512,14.985,1211454.537,5.52
103994.691,14.986,1211454.587,4.95
103994.016,14.991,1211454.065,4.94
103993.243,14.991,1211454.256,4.99
103992.504,14.963,1211453.83,4.89
103991.712,14.948,1211453.944,4.92
103990.963,14.954,1211453.529,4.91
103990.159,14.958,1211453.609,4.98
103989.445,14.962,1211453.13,5.17
103988.686,14.949,1211453.284,5.31
103988.009,14.949,1211452.799,5.3
103987.256,14.953,1211452.963,5.4
103986.592,14.967,1211452.462,5.33
103985.843,14.952,1211452.666,5.09
103985.177,14.955,1211452.19,5.02
103984.44,14.966,1211452.402,5.07
103983.795,14.936,1211451.899,5.14
103983.056,14.952,1211452.139,5.07
103982.369,14.943,1211451.659,5.07
103981.62,14.957,1211451.839,5.21
103980.963,14.956,1211451.318,5.53
103980.257,14.942,1211451.594,5.58
103979.619,14.968,1211451.072,5.62
103978.864,14.946,1211451.247,5.9
103978.23,14.945,1211450.713,6.4
103977.513,14.947,1211450.976,8.1
103976.863,14.965,1211450.446,9.1
103976.141,14.947,1211450.697,10.42
103975.508,14.936,1211450.166,10.4
103974.747,14.903,1211450.293,11.15
103974.086,14.917,1211449.802,12
103973.35,14.86,1211450.037,12.1
103972.723,14.764,1211449.49,12.19
103971.991,14.789,1211449.711,12.15
103971.337,14.757,1211449.199,11.95
103970.599,14.717,1211449.405,11.9
103969.916,14.73,1211448.924,11.96
103969.194,14.691,1211449.192,12.28
103968.584,14.691,1211448.628,13.31
103967.833,14.655,1211448.801,13.18
103967.215,14.64,1211448.235,13.18
103966.452,14.618,1211448.342,13.02
103965.675,14.617,1211448.019,13.02
103964.919,14.585,1211448.157,12.95
103964.317,14.569,1211447.58,13.4
103963.587,14.594,1211447.838,13.52
103962.925,14.632,1211447.329,13.71
103962.202,14.607,1211447.615,13.88
103961.544,14.544,1211447.103,15
103960.804,14.487,1211447.299,14.95
103960.107,14.408,1211446.868,15.11
103959.357,14.357,1211447.046,15.38
103958.718,14.297,1211446.527,15.53
103957.958,14.266,1211446.715,15.55
103957.25,14.167,1211446.248,15.78
103956.502,14.145,1211446.462,15.66
103955.882,14.068,1211445.913,15.62
103955.157,14.054,1211446.182,15.6
103954.484,14.055,1211445.687,16
103953.716,14.116,1211445.847,16
103953.091,14.094,1211445.335,16.05
103952.346,14.085,1211445.57,16.09
103951.701,14.062,1211445.055,16
103950.942,14.101,1211445.269,15.85
103950.302,14.12,1211444.771,15.44
103949.538,14.151,1211444.978,15.31
103948.911,14.139,1211444.469,15.18
103948.17,14.175,1211444.722,15.29
103947.531,14.202,1211444.206,15.35
103946.773,14.17,1211444.427,14.33
103946.097,14.21,1211443.938,15.24
103945.337,14.207,1211444.149,15.25
103944.64,14.143,1211443.555,16.31
103943.832,14.049,1211443.766,16.3
103943.124,14.036,1211443.413,15";

const RAW2: &str = "103943.124,14.036,1211443.413,15
103942.359,14.025,1211443.72,15.5
103941.643,14.055,1211443.365,15.5
103940.876,14.032,1211443.646,15.5
103940.186,14.049,1211443.279,15.5
103939.408,14.046,1211443.49,15.59
103938.672,14.042,1211443.17,15.47
103937.884,14.044,1211443.401,15.42
103937.189,14.058,1211443.016,15.5
103936.391,14.07,1211443.058,15.79
103935.795,14.069,1211442.506,15.83
103935.029,14.036,1211442.709,15.58
103934.376,13.981,1211442.21,16.05
103933.575,14.063,1211442.333,16.1
103932.969,14.088,1211441.838,16.15
103932.149,14.065,1211441.874,16.2
103931.534,14.079,1211441.354,16.25
103930.729,14.078,1211441.467,16.3
103930.091,14.095,1211441.002,16.35
103929.274,14.091,1211441.041,16.4
103928.626,14.093,1211440.567,16.45
103927.871,14.058,1211440.631,16.5
103927.23,14.089,1211440.119,16.3
103926.425,14.099,1211440.216,16
103925.834,14.09,1211439.667,17
103925.054,14.1,1211439.847,17.5
103924.418,14.094,1211439.381,17.6
103923.651,14.11,1211439.598,17.9
103922.893,14.11,1211439.353,17.95
103922.09,14.085,1211439.47,18
103921.482,14.062,1211438.96,18.12
103920.691,14.068,1211439.231,18.08
103920.078,14.031,1211438.679,18.03
103919.286,14.049,1211438.751,17.95
103918.691,14.001,1211438.206,17.8
103917.899,14.014,1211438.319,17.7
103917.341,14.031,1211437.76,17.8
103916.517,14.016,1211437.864,18.1
103915.93,14.031,1211437.364,18.05";

const RAW3: &str = "103911.577,14.046,1211436.293,18.03
103910.892,14.054,1211435.797,18.18
103910.112,14.055,1211435.926,15.36
103909.448,14.055,1211435.488,15.6
103908.661,14.031,1211435.614,18.5
103908.045,14.042,1211435.087,18.03
103907.246,13.946,1211435.245,18
103906.571,14.023,1211434.75,18
103905.815,14.025,1211434.921,20.57
103905.124,14.015,1211434.399,20.55
103904.429,14.026,1211434.65,17.55
103903.776,14.023,1211434.152,17.2
103903.017,14.021,1211434.388,17
103902.367,14.021,1211433.918,16.89
103901.587,14.011,1211434.106,16.81
103900.901,14.028,1211433.653,16.96
103900.141,14.006,1211433.786,16.46
103899.44,14.006,1211433.354,16.36
103898.667,14.011,1211433.578,16.03
103897.995,14.009,1211433.146,15.9";
