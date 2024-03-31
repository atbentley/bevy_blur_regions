// Demonstrates how to tweak the settings of the blurring
//   cargo run --example settings --features egui

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

fn update(mut contexts: EguiContexts, mut blur_region_cameras: Query<&mut bevy_blur_regions::DefaultBlurRegionsCamera>) {
    let mut blur_regions = blur_region_cameras.single_mut();

    let frame = egui::Frame::window(&contexts.ctx_mut().style())
        .fill(egui::Color32::from_rgba_premultiplied(27, 27, 27, 225))
        .rounding(0.0)
        .inner_margin(egui::Margin::same(50.0))
        .shadow(egui::epaint::Shadow::NONE);

    let mut radius = blur_regions.radius;
    let mut linear_steps = blur_regions.linear_steps;
    let mut radial_steps = blur_regions.radial_steps;

    egui::Window::new("Hint")
        .frame(frame)
        .title_bar(false)
        .default_pos(egui::pos2(300.0 / 2.0, 720.0 / 2.0))
        .pivot(egui::Align2::CENTER_CENTER)
        .resizable(false)
        .show_with_blur(egui::Id::new("Hint"), &mut blur_regions, contexts.ctx_mut(), |ui| {
            ui.label("All blur regions on the same\ncamera share blur settings.");
        });

    egui::Window::new("Settings")
        .frame(frame)
        .title_bar(false)
        .default_pos(egui::pos2(1280.0 / 2.0, 720.0 / 2.0))
        .pivot(egui::Align2::CENTER_CENTER)
        .resizable(false)
        .show_with_blur(egui::Id::new("Settings"), &mut blur_regions, contexts.ctx_mut(), |ui| {
            ui.add_space(50.0);
            let radius_slider = egui::Slider::new(&mut radius, 0.0..=300.0).text("Radius").suffix("px");
            ui.add(radius_slider);
            let linear_steps_slider = egui::Slider::new(&mut linear_steps, 0..=32).text("Linear steps");
            ui.add(linear_steps_slider);
            let radial_steps_slider = egui::Slider::new(&mut radial_steps, 0..=32).text("Radial steps");
            ui.add(radial_steps_slider);
            ui.add_space(50.0);
        });

    if blur_regions.radius != radius {
        blur_regions.radius = radius;
    }
    if blur_regions.linear_steps != linear_steps {
        blur_regions.linear_steps = linear_steps;
    }
    if blur_regions.radial_steps != radial_steps {
        blur_regions.radial_steps = radial_steps;
    }
}
