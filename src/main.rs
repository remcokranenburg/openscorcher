use bevy::prelude::*;
use bevy::render::mesh::SphereKind;
use bevy::render::mesh::primitives::SphereMeshBuilder;
use bevy::window::WindowTheme;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                title: "openScorcher".into(),
                window_theme: Some(WindowTheme::Dark),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        Collider::cuboid(5.0, 0.05, 5.0),
        Mesh3d(meshes.add(Cuboid::new(20.0, 0.1, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(64, 38, 38))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Sphere
    commands
        .spawn((
            Collider::ball(1.0),
            Mesh3d(meshes.add(SphereMeshBuilder::new(
                1.0,
                SphereKind::Ico { subdivisions: 8 },
            ))),
            MeshMaterial3d(materials.add(Color::srgb_u8(51, 204, 51))),
            Transform::from_xyz(0.0, 5.0, 0.0),
            Restitution::coefficient(0.7),
            RigidBody::Dynamic,
        ))
        .with_child(PointLight {
            shadows_enabled: true,
            color: Color::srgb_u8(0, 64, 0),
            ..default()
        });

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
