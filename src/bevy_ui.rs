use bevy::prelude::*;
use bevy::render::camera::NormalizedRenderTarget;
use bevy::window::PrimaryWindow;

use crate::BlurRegion;
use crate::BlurRegionsCamera;

pub struct BlurRegionsBevyUiPlugin<const N: usize>;

impl<const N: usize> Plugin for BlurRegionsBevyUiPlugin<N> {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, crate::bevy_ui::compute_blur_regions::<N>);
    }
}

pub fn compute_blur_regions<const N: usize>(
    nodes: Query<(&Node, &GlobalTransform), With<BlurRegion>>,
    mut blur_regions_cameras: Query<(&Camera, &mut BlurRegionsCamera<N>)>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    windows: Query<&Window>,
) {
    for (camera, mut blur_regions) in &mut blur_regions_cameras {
        let Some(target) = camera.target.normalize(primary_window.get_single().ok()) else {
            continue;
        };

        let NormalizedRenderTarget::Window(window_entity) = target else {
            continue;
        };

        let Ok(window) = windows.get(window_entity.entity()) else {
            continue;
        };

        for (node, transform) in &nodes {
            let translation = transform.translation();
            let region = Rect::from_center_size(
                translation.xy() * window.scale_factor(),
                node.size() * window.scale_factor(),
            );
            blur_regions.blur(region);
        }
    }
}
