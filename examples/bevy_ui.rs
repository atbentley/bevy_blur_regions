// Demonstrates how to use Bevy UI integration to blur nodes.
//   cargo run --example bevy_ui

use bevy::prelude::*;
use bevy_blur_regions::prelude::*;

#[path = "./utils.rs"]
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BlurRegionsPlugin::default())
        .add_systems(Startup, (setup, utils::spawn_example_scene_3d))
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
            border_radius: BorderRadius::new(Val::ZERO, Val::Percent(5.0), Val::Percent(10.0), Val::Percent(15.0)),
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

fn move_node(time: Res<Time>, mut nodes: Query<(&mut Style, &mut Visibility)>) {
    for (mut style, mut visibility) in &mut nodes {
        style.left = Val::Percent((time.elapsed_seconds().cos() + 1.0) / 2.0 * 50.0);
        style.top = Val::Percent((time.elapsed_seconds().sin() + 1.0) / 2.0 * 50.0);

        *visibility = if time.elapsed_seconds() % 2. < 1. {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
