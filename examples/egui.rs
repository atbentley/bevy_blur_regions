// Demonstrates how to add blurring to egui windows.
//   cargo run --example egui --features egui

#[path = "./utils.rs"]
mod utils;

use bevy::prelude::*;
use bevy_blur_regions::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
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

fn update(
    mut contexts: EguiContexts,
    mut blur_region_cameras: Query<Entity, With<bevy_blur_regions::DefaultBlurRegionsCamera>>,
) {
    let entity = blur_region_cameras.single_mut();

    let frame = egui::Frame::window(&contexts.ctx_mut().style())
        .fill(egui::Color32::from_rgba_premultiplied(27, 27, 27, 100))
        .rounding(0.0)
        .shadow(egui::epaint::Shadow::NONE);

    egui::Window::new("Blur").frame(frame).show_with_blur(contexts.ctx_mut(), |ui| {
        ui.allocate_space(egui::vec2(300.0, 150.0));
    });

    egui::Window::new("Blur2").frame(frame).show_with_blur_on_camera(entity, contexts.ctx_mut(), |ui| {
        ui.allocate_space(egui::vec2(300.0, 150.0));
    });
}
