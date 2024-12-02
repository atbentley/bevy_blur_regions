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
        Camera3d::default(),
        Camera { order: 1, ..default() },
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // UI camera
    commands.spawn((
        Camera2d::default(),
        Camera { order: 2, ..default() }
        ));

    // UI node with blur region
    commands.spawn((
        BlurRegion,
        Node {
            width: Val::Percent(50.0),
            height: Val::Percent(50.0),
            border: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BorderColor(Color::BLACK),
        BorderRadius::new(Val::ZERO, Val::Percent(5.0), Val::Percent(10.0), Val::Percent(15.0)),
    ));
}

fn move_node(time: Res<Time>, mut nodes: Query<(&mut Node, &mut Visibility)>) {
    for (mut node, mut visibility) in &mut nodes {
        node.left = Val::Percent((time.elapsed_secs().cos() + 1.0) / 2.0 * 50.0);
        node.top = Val::Percent((time.elapsed_secs().sin() + 1.0) / 2.0 * 50.0);

        *visibility = if time.elapsed_secs() % 2. < 1. {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
