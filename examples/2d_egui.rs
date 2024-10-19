// Demonstrates using a 2d camera with egui.
//   cargo run --example 2d_egui --features egui

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
        .add_systems(Startup, (setup, utils::spawn_example_scene_2d))
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), DefaultBlurRegionsCamera::default()));
}

fn update(
    mut contexts: EguiContexts,
    mut blur_region_cameras: Query<Entity, With<bevy_blur_regions::DefaultBlurRegionsCamera>>,
) {
    let entity = blur_region_cameras.single_mut();

    let frame = egui::Frame::window(&contexts.ctx_mut().style())
        .fill(egui::Color32::from_white_alpha(10))
        .rounding(10.0)
        .shadow(egui::epaint::Shadow::NONE);

    egui::Window::new("Blur").frame(frame).title_bar(false).resizable(false).show_with_blur(contexts.ctx_mut(), |ui| {
        ui.allocate_space(egui::vec2(300.0, 150.0));
    });

    egui::Window::new("Blur2").frame(frame).title_bar(false).resizable(false).show_with_blur_on_camera(
        entity,
        contexts.ctx_mut(),
        |ui| {
            ui.allocate_space(egui::vec2(300.0, 150.0));
        },
    );
}
