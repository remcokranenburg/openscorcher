use bevy::prelude::*;
use bevy::render::mesh::primitives::SphereMeshBuilder;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane (10x10)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(10.0, 0.1, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(77, 128, 77))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Sphere (radius 1)
    commands.spawn((
        Mesh3d(meshes.add(SphereMeshBuilder::new(1.0, Default::default()))),
        MeshMaterial3d(materials.add(Color::srgb_u8(204, 51, 51))),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
