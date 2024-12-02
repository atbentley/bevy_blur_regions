use bevy::prelude::*;

#[allow(dead_code)]
pub fn spawn_example_scene_3d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.5, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 1.01, 0.0),
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

#[allow(dead_code)]
pub fn spawn_example_scene_2d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shapes = [
        Mesh2d(meshes.add(Circle { radius: 50.0 })),
        Mesh2d(meshes.add(CircularSector::new(50.0, 1.0))),
        Mesh2d(meshes.add(CircularSegment::new(50.0, 1.25))),
        Mesh2d(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2d(meshes.add(Annulus::new(25.0, 50.0))),
        Mesh2d(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2d(meshes.add(Rhombus::new(75.0, 100.0))),
        Mesh2d(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2d(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];
    let num_shapes = shapes.len();

    let extent: f32 = 900.;
    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((
            shape,
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(-(extent / 2. + i as f32 / (num_shapes - 1) as f32 * extent), 0.0, 0.0),
        ));
    }
}

#[allow(dead_code)]
fn main() {
    // This is not an example, it is here to make rust-analyzer and clippy happy
}
