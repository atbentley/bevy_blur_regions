/// This example demonstrates that a scene without any blur regions functions correctly.

#[path = "./utils.rs"]
mod utils;

use bevy::prelude::*;
use bevy_blur_regions::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BlurRegionsPlugin::default())
        .add_systems(Startup, (setup, utils::spawn_example_scene))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        BlurRegionsCamera::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}
