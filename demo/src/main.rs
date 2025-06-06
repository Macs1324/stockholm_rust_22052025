use bevy::{input::mouse::AccumulatedMouseMotion, platform::collections::HashSet, prelude::*};

const GRID_X: f32 = 45.0;
const GRID_Y: f32 = 45.0;
const GRID_Z: f32 = 45.0;

const CELL_SIZE: f32 = 0.1;
const CELL_GAP: f32 = 0.01;

const CAMERA_ORBIT_RADIUS: f32 = 12.5;
const CAMERA_ORBIT_SPEED: f32 = 0.1;
const PAN_SPEED: f32 = 0.2;

const TICKER_INTERVAL: f32 = 0.1;

#[derive(Event)]
pub struct RandomizeGridEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(LifeTicker {
            timer: Timer::from_seconds(TICKER_INTERVAL, TimerMode::Repeating),
        })
        .add_event::<RandomizeGridEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, render_cells)
        .add_systems(FixedUpdate, game_of_life)
        .add_systems(Update, randomize_on_keypress)
        .add_systems(FixedUpdate, orbit)
        .add_systems(Update, randomize_grid)
        .run();
}

#[derive(Resource)]
struct LifeTicker {
    timer: Timer,
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
struct Orbit {
    radius: f32,
    speed: f32,
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
    mut event: EventWriter<RandomizeGridEvent>,
) {
    clear_color.0 = Color::srgb(0.07, 0.1, 0.14);
    commands.spawn((
        Name::new("Camera"),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Orbit {
            radius: CAMERA_ORBIT_RADIUS,
            speed: CAMERA_ORBIT_SPEED,
        },
        Camera3d::default(),
    ));

    commands.spawn((
        Name::new("Light"),
        PointLight::default(),
        Transform::from_xyz(3.0, 8.0, 5.0),
    ));

    commands.spawn((
        Name::new("Light"),
        PointLight::default(),
        Transform::from_xyz(-3.0, 8.0, -6.0),
    ));

    for i in 0..GRID_X as i32 {
        for j in 0..GRID_Y as i32 {
            for k in 0..GRID_Z as i32 {
                let distance = ((((i as f32 - GRID_X / 2.0).powi(2)
                    + (j as f32 - GRID_Y / 2.0).powi(2)
                    + (k as f32 - GRID_Z / 2.0).powi(2))
                .sqrt()
                    / (GRID_X.powi(2) + GRID_Y.powi(2) + GRID_Z.powi(2)).sqrt())
                    * 3.0)
                    .powf(-1.2);

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
                        (i as f32 - GRID_X / 2.0) * CELL_SIZE,
                        (j as f32 - GRID_Y / 2.0) * CELL_SIZE,
                        (k as f32 - GRID_Z / 2.0) * CELL_SIZE,
                    ),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.0 * distance, 0.67 * distance, 0.88 * distance),
                        ..default()
                    })),
                ));
            }
        }
    }

    event.write(RandomizeGridEvent);
}

fn game_of_life(
    mut query: Query<(&mut Cell, &GridPosition)>,
    time: Res<Time>,
    mut ticker: ResMut<LifeTicker>,
) {
    ticker.timer.tick(time.delta());
    if ticker.timer.just_finished() {
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
                    if !(12..=18).contains(&neighbors) {
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

fn randomize_grid(
    mut query: Query<(&mut Cell, &Transform)>,
    mut event: EventReader<RandomizeGridEvent>,
) {
    for _ in event.read() {
        for (mut cell, _transform) in query.iter_mut() {
            if rand::random::<f32>() < 0.15 {
                *cell = Cell::Alive;
            } else {
                *cell = Cell::Dead;
            }
        }
    }
}

fn randomize_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event: EventWriter<RandomizeGridEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        event.write(RandomizeGridEvent);
    }
}
fn orbit(
    mut query: Query<(&mut Transform, &Orbit)>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut touch_state: Local<TouchOrbitState>,
    time: Res<Time>,
) {
    for (mut transform, orbit) in &mut query.iter_mut() {
        let current_pos = transform.translation;
        let current_radius = orbit.radius;

        // Convert current position to spherical angles
        let mut theta = current_pos.z.atan2(current_pos.x);
        let mut phi = (current_pos.y / current_radius).acos();

        // Check if we're interacting with either mouse or touch
        let is_mouse_active = mouse_buttons.pressed(MouseButton::Left);
        let is_touch_active = touches.iter().next().is_some();

        if is_mouse_active {
            // Use mouse motion directly - it's already accumulated properly
            let delta = mouse_motion.delta;

            // Calculate rotation amounts
            let rotation_y = -delta.x * PAN_SPEED * time.delta_secs();
            let rotation_x = -delta.y * PAN_SPEED * time.delta_secs();

            // Update angles based on mouse movement
            theta -= rotation_y;
            phi = (phi + rotation_x).clamp(0.1, std::f32::consts::PI - 0.1);

            // Reset touch state when using mouse
            touch_state.last_position = None;
        } else if is_touch_active {
            // Handle single touch for orbit
            if let Some(touch) = touches.iter().next() {
                let current_touch_pos = touch.position();

                // Calculate delta from last frame, not just from previous position
                // This matches how mouse_motion accumulates over time
                if let Some(last_pos) = touch_state.last_position {
                    let delta = Vec2::new(
                        current_touch_pos.x - last_pos.x,
                        current_touch_pos.y - last_pos.y,
                    );

                    // Use the same math as for mouse
                    let rotation_y = -delta.x * PAN_SPEED * time.delta_secs();
                    let rotation_x = -delta.y * PAN_SPEED * time.delta_secs();

                    // Update angles based on touch movement
                    theta -= rotation_y;
                    phi = (phi + rotation_x).clamp(0.1, std::f32::consts::PI - 0.1);
                }

                // Save current position for next frame
                touch_state.last_position = Some(current_touch_pos);
            }
        } else {
            // Auto-rotate when there's no input
            theta += time.delta_secs() * orbit.speed;

            // Reset touch state when not touching
            touch_state.last_position = None;
        }

        // Convert back to Cartesian coordinates
        transform.translation.x = orbit.radius * phi.sin() * theta.cos();
        transform.translation.y = orbit.radius * phi.cos();
        transform.translation.z = orbit.radius * phi.sin() * theta.sin();

        // Make camera always look at the origin
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

// Add this struct to track touch state between frames
#[derive(Default)]
struct TouchOrbitState {
    last_position: Option<Vec2>,
}
