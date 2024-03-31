use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::camera::NormalizedRenderTarget;
use bevy::window::PrimaryWindow;

use crate::BlurRegion;
use crate::BlurRegionsCamera;

pub fn compute_blur_regions<const N: usize>(
    primary_windows: Query<Entity, With<PrimaryWindow>>,
    windows: Query<&Window>,
    nodes: Query<(&Node, &GlobalTransform), With<BlurRegion>>,
    mut blur_regions_cameras: Query<(&Camera, &mut BlurRegionsCamera<N>)>,
) {
    let primary_window = primary_windows.get_single().ok();

    for (camera, mut blur_regions) in &mut blur_regions_cameras {
        let Some(normalized_target) = camera.target.normalize(primary_window) else {
            continue;
        };
        let window = match normalized_target {
            NormalizedRenderTarget::Window(window_target) => {
                let Ok(window) = windows.get(window_target.entity()) else {
                    continue;
                };
                window
            }
            _ => {
                warn!("Unsupported blur region target {:?}", camera.target);
                continue;
            }
        };
        let window_size = vec2(window.resolution.width(), window.resolution.height());

        for (node, transform) in &nodes {
            let translation = transform.translation();
            let region = Rect::from_center_size(translation.xy() / window_size, node.size() / window_size);
            blur_regions.blur(region);
        }
    }
}
