use bevy::prelude::*;

pub fn spawn_example_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((PbrBundle {
        mesh: meshes.add(Rectangle::new(10.0, 10.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    },));
    commands.spawn((PbrBundle {
        mesh: meshes.add(Capsule3d::new(0.5, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 1.01, 0.0),
        ..default()
    },));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

#[allow(dead_code)]
fn main() {
    // This is not an example, it is here to make rust-analyzer and clippy happy
}
