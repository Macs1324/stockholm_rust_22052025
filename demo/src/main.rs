use bevy::{
    color::palettes::tailwind::CYAN_800,
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

const GRID_X: f32 = 30.0;
const GRID_Y: f32 = 30.0;
const GRID_Z: f32 = 30.0;

const CELL_SIZE: f32 = 0.1;
const CELL_GAP: f32 = 0.01;

const SPAWN_RADIUS: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, render_cells)
        .add_systems(FixedUpdate, game_of_life)
        .add_systems(Update, add_cells_on_keypress)
        .run();
}

fn render_cells(mut query: Query<(&Cell, &mut Visibility), Changed<Cell>>) {
    for (cell, mut visibility) in &mut query.iter_mut() {
        match cell {
            Cell::Alive => *visibility = Visibility::Visible,
            Cell::Dead => {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

#[derive(Component)]
enum Cell {
    Alive,
    Dead,
}

#[derive(Component)]
struct GridPosition {
    x: i32,
    y: i32,
    z: i32,
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
                    Cell::Dead,
                    Visibility::Hidden,
                    GridPosition { x: i, y: j, z: k },
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

fn game_of_life(mut query: Query<(&mut Cell, &GridPosition)>) {
    let mut alive_positions: HashSet<(i32, i32, i32)> = HashSet::new();

    for (cell, transform) in query.iter() {
        match cell {
            Cell::Alive => {
                alive_positions.insert((transform.x, transform.y, transform.z));
            }
            Cell::Dead => {}
        }
    }

    for (mut cell, transform) in query.iter_mut() {
        let neighbors = neighbor_count(&alive_positions, transform.x, transform.y, transform.z);

        match *cell {
            Cell::Alive => {
                if neighbors < 12 || neighbors > 17 {
                    *cell = Cell::Dead;
                }
            }
            Cell::Dead => {
                if (12..=17).contains(&neighbors) || neighbors == 4 {
                    *cell = Cell::Alive;
                }
            }
        }
    }
}

fn neighbor_count(alive_positions: &HashSet<(i32, i32, i32)>, x: i32, y: i32, z: i32) -> usize {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                if alive_positions.contains(&(x + dx, y + dy, z + dz)) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn add_cells_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Cell, &Transform)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (mut cell, _transform) in query.iter_mut() {
            // if transform.translation.distance(Vec3::ZERO) < CELL_SIZE * SPAWN_RADIUS {
            if rand::random::<f32>() < 0.50 {
                *cell = Cell::Alive;
            } else {
                *cell = Cell::Dead;
            }
            // }
        }
    }
}
