// Demonstrates how to use the `BlurRegionsCamera::blur` immediate blurring api.
//   cargo run --example immediate

#[path = "./utils.rs"]
mod utils;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy_blur_regions::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BlurRegionsPlugin::default())
        .add_systems(Startup, (setup, utils::spawn_example_scene))
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DefaultBlurRegionsCamera::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}

fn update(mut blur_region_cameras: Query<&mut DefaultBlurRegionsCamera>) {
    let Ok(mut blur_regions) = blur_region_cameras.get_single_mut() else {
        return;
    };
    blur_regions.blur(Rect::from_center_size(vec2(0.25, 0.5), vec2(0.3, 0.5)));
    blur_regions.blur(Rect::from_center_size(vec2(0.75, 0.5), vec2(0.3, 0.5)));
}
