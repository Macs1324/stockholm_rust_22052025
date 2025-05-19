use bevy::{color::palettes::tailwind::CYAN_800, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
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

    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 1.0, 0.6),
            ..default()
        })),
    ));
}
