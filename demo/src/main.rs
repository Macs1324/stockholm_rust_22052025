use bevy::{color::palettes::tailwind::CYAN_800, prelude::*};

const GRID_X: f32 = 30.0;
const GRID_Y: f32 = 30.0;
const GRID_Z: f32 = 30.0;

const CELL_SIZE: f32 = 0.1;
const CELL_GAP: f32 = 0.01;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
enum Cell {
    Alive,
    Dead,
}

pub fn setup(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    clear_color.0 = CYAN_800.into();
    commands.spawn((
        Name::new("Camera"),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
    ));

    commands.spawn((
        Name::new("Light"),
        PointLight::default(),
        Transform::from_xyz(3.0, 8.0, 5.0),
    ));

    for i in 0..GRID_X as i32 {
        for j in 0..GRID_Y as i32 {
            for k in 0..GRID_Z as i32 {
                commands.spawn((
                    Name::new("Cell"),
                    Cell::Alive,
                    Visibility::Visible,
                    Mesh3d(meshes.add(Cuboid::new(
                        CELL_SIZE - CELL_GAP,
                        CELL_SIZE - CELL_GAP,
                        CELL_SIZE - CELL_GAP,
                    ))),
                    Transform::from_xyz(
                        i as f32 * CELL_SIZE,
                        j as f32 * CELL_SIZE,
                        k as f32 * CELL_SIZE,
                    ),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.5, 1.0, 0.6),
                        ..default()
                    })),
                ));
            }
        }
    }
}
