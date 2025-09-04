use bevy::app::AppExit;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy::render::mesh::SphereKind;
use bevy::render::mesh::primitives::SphereMeshBuilder;
use bevy::window::{MonitorSelection, VideoModeSelection, WindowMode, WindowTheme};
use bevy_rapier3d::prelude::*;

const BALL_MAX_SPEED: f32 = 20.0;
const BALL_MAX_REVERSE_SPEED: f32 = -10.0;
const BALL_FORCE: f32 = 30.0;
const BALL_STRAFE_FORCE: f32 = 20.0;
const BALL_ANGULAR_SPEED: f32 = 1.0;
const BALL_FRICTION: f32 = 0.98;
const BALL_ANGULAR_FRICTION: f32 = 0.95;

#[derive(Component)]
struct RaceBall(usize);

#[derive(Component, Default)]
struct BallOrientation(f32); // Yaw in radians

fn spawn_ball(
    id: usize,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        RaceBall(id),
        BallOrientation::default(),
        Collider::ball(1.0),
        Mesh3d(meshes.add(SphereMeshBuilder::new(
            1.0,
            SphereKind::Ico { subdivisions: 8 },
        ))),
        MeshMaterial3d(materials.add(Color::srgb_u8(51, 204, 51))),
        Transform::from_xyz(0.0, 5.0, 0.0),
        PointLight {
            shadows_enabled: true,
            color: Color::srgb_u8(0, 64, 0),
            ..default()
        },
        Restitution::coefficient(0.7),
        RigidBody::Dynamic,
        ExternalForce::default(),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                title: "openScorcher".into(),
                window_theme: Some(WindowTheme::Dark),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_input,
                apply_friction,
                respawn_when_off_track,
                move_camera,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        Collider::cuboid(10.0, 0.1, 10.0),
        Mesh3d(meshes.add(Cuboid::new(20.0, 0.2, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(64, 38, 38))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    spawn_ball(0, &mut commands, &mut meshes, &mut materials);

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

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    mut ball_query: Query<(
        &mut ExternalForce,
        &mut Transform,
        &RaceBall,
        &mut BallOrientation,
    )>,
    time: Res<Time>,
) {
    if keyboard.any_pressed([KeyCode::KeyW, KeyCode::KeyQ])
        && keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
    {
        exit.write(AppExit::Success);
    }
    let dt = time.delta_secs();
    if let Ok((mut ext_force, mut transform, race_ball, mut orientation)) = ball_query.single_mut()
    {
        let mut force = Vec3::ZERO;
        let mut angular = 0.0;
        // Forward/Backward
        if keyboard.pressed(KeyCode::ArrowUp) {
            force += Vec3::Z * -BALL_FORCE;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            force += Vec3::Z * BALL_FORCE;
        }
        // Left/Right (strafe)
        if keyboard.pressed(KeyCode::ArrowLeft) {
            force += Vec3::X * -BALL_STRAFE_FORCE;
            angular += BALL_ANGULAR_SPEED * dt;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            force += Vec3::X * BALL_STRAFE_FORCE;
            angular -= BALL_ANGULAR_SPEED * dt;
        }
        // Apply orientation
        let yaw = orientation.0;
        let rot = Quat::from_rotation_y(yaw);
        let force = rot * force;
        ext_force.force = force;
        // Angular momentum
        orientation.0 += angular;
        // Update transform rotation for visual orientation
        transform.rotation = Quat::from_rotation_y(orientation.0);
    }
}

fn apply_friction(mut ball_query: Query<&mut Velocity, With<RaceBall>>) {
    if let Ok(mut velocity) = ball_query.single_mut() {
        velocity.linvel *= BALL_FRICTION;
        velocity.angvel *= BALL_ANGULAR_FRICTION;
    }
}

fn respawn_when_off_track(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &Transform, &mut BallOrientation, &RaceBall)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((entity, transform, mut orientation, race_ball)) = ball_query.single_mut() {
        if transform.translation.y < -5.0 {
            commands.entity(entity).despawn();
            spawn_ball(race_ball.0, &mut commands, &mut meshes, &mut materials);
            orientation.0 = 0.0;
        }
    }
}

fn move_camera(
    ball_query: Query<(&Transform, &RaceBall)>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<RaceBall>)>,
    time: Res<Time>,
) {
    if let (Ok((ball_transform, _)), Ok(mut cam_transform)) =
        (ball_query.single(), camera_query.single_mut())
    {
        let ball_pos = ball_transform.translation;
        let ball_rot = ball_transform.rotation;
        // Desired camera position: 12 units behind, 8 units above
        let back = ball_rot * Vec3::Z * 12.0;
        let up = Vec3::Y * 8.0;
        let target_pos = ball_pos + back + up;
        // Interpolate position
        let lerp_factor = time.delta().as_secs_f32() / 0.5;
        cam_transform.translation = cam_transform.translation.lerp(target_pos, lerp_factor);
        // Desired camera rotation: looking at the ball
        let target_rot = Transform::from_translation(cam_transform.translation)
            .looking_at(ball_pos, Vec3::Y)
            .rotation;
        cam_transform.rotation = cam_transform.rotation.slerp(target_rot, lerp_factor);
    }
}
