use bevy::prelude::*;
use bevy_blur_regions::prelude::*;

#[path = "./utils.rs"]
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BlurRegionsPlugin::default())
        .add_systems(Startup, (setup, utils::spawn_example_scene))
        .add_systems(Update, move_node)
        .run();
}

fn setup(mut commands: Commands) {
    // 3D camera
    commands.spawn((
        BlurRegionsCamera::default(),
        Camera3dBundle {
            camera: Camera { order: 1, ..default() },
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    // UI camera
    commands.spawn(Camera2dBundle {
        camera: Camera { order: 2, ..default() },
        ..default()
    });

    // UI node with blur region
    commands.spawn((
        BlurRegion,
        NodeBundle {
            border_color: Color::BLACK.into(),
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(50.0),
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            ..default()
        },
    ));
}

fn move_node(time: Res<Time>, mut nodes: Query<&mut Style>) {
    for mut style in &mut nodes {
        style.left = Val::Percent((time.elapsed_seconds().cos() + 1.0) / 2.0 * 50.0);
        style.top = Val::Percent((time.elapsed_seconds().sin() + 1.0) / 2.0 * 50.0);
    }
}
