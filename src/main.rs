use bevy::{prelude::*, window::close_on_esc};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                close_on_esc,
                update_rotate_round_and_self,
                update_rotate_round_only,
            ),
        )
        .run();
}

#[derive(Debug, Component)]
struct RotateCenter;

#[derive(Debug, Component)]
struct RotateRoundAndSelf;

#[derive(Debug, Component)]
struct RotateRoundOnly;

fn setup(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 15.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(5.0, 50.0, 5.0).looking_at(Vec3::Z, Vec3::Y),
        ..default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: meshs.add(shape::Plane::default().into()),
        material: materials.add(Color::DARK_GREEN.into()),
        transform: Transform::default().with_scale(Vec3::new(20.0, 1.0, 20.0)),
        ..default()
    });

    // center ball
    commands.spawn((
        PbrBundle {
            mesh: meshs.add(shape::UVSphere::default().into()),
            material: materials.add(Color::YELLOW.into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0).with_scale(Vec3::splat(0.1)),
            ..default()
        },
        RotateCenter,
    ));

    // rotate round and self box
    commands.spawn((
        PbrBundle {
            mesh: meshs.add(shape::Cube::default().into()),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(5.0, 1.0, 0.0),
            ..default()
        },
        RotateRoundAndSelf,
    ));

    // rotate round only
    commands.spawn((
        PbrBundle {
            mesh: meshs.add(shape::Cube::default().into()),
            material: materials.add(Color::GRAY.into()),
            transform: Transform::from_xyz(-5.0, 1.0, 0.0),
            ..default()
        },
        RotateRoundOnly,
    ));
}

fn update_rotate_round_and_self(
    center_transform: Query<&Transform, (With<RotateCenter>, Without<RotateRoundAndSelf>)>,
    mut rotate_transform: Query<&mut Transform, (With<RotateRoundAndSelf>, Without<RotateCenter>)>,
    time: Res<Time>,
) {
    let center_transform = center_transform.single();
    let mut rotate_transform = rotate_transform.single_mut();
    // get distence

    rotate_transform.rotate_around(
        center_transform.translation,
        Quat::from_rotation_y(1.0 * time.delta_seconds()),
    );
}

fn update_rotate_round_only(
    center_transform: Query<&Transform, (With<RotateCenter>, Without<RotateRoundOnly>)>,
    mut rotate_transform: Query<&mut Transform, (With<RotateRoundOnly>, Without<RotateCenter>)>,
    time: Res<Time>,
) {
    let center_transform = center_transform.single();
    let mut rotate_transform = rotate_transform.single_mut();
    // get distence

    let rotation = rotate_transform.rotation;
    rotate_transform.rotate_around(
        center_transform.translation,
        Quat::from_rotation_y(1.0 * time.delta_seconds()),
    );
    rotate_transform.rotation = rotation;
}
