mod diags;

use bevy::prelude::*;
use diags::DiagsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DiagsPlugin)
        .add_startup_system(setup)
        .add_system(rotate)
        .run();
}

#[derive(Component)]
struct Shape;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 3.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 10.0, 3.0),
        ..default()
    });

    let material = materials.add(StandardMaterial {
        base_color: Color::GREEN,
        ..default()
    });
    let mesh = meshes.add(shape::Cube::default().into());

    commands.spawn((
        PbrBundle {
            mesh,
            material,
            ..default()
        },
        Shape,
    ));
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(0.01);
    }
}
