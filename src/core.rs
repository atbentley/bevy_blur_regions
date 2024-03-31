use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::ShaderType;

pub const DEFAULT_MAX_BLUR_REGIONS_COUNT: usize = 20;

/// Add this marker component to a UI Node to indicate that a blur region
/// should be created behind it.
#[derive(Component, Default, Clone, Copy)]
pub struct BlurRegion;

/// The final computed values of the blur region, in device coordinates (i.e. 0.0 - 1.0).
#[derive(Default, Debug, Clone, ShaderType)]
struct ComputedBlurRegion {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

impl ComputedBlurRegion {
    const OFFSCREEN: ComputedBlurRegion = ComputedBlurRegion {
        min_x: -1.0,
        max_x: -1.0,
        min_y: -1.0,
        max_y: -1.0,
    };
}

pub type DefaultBlurRegionsCamera = BlurRegionsCamera<DEFAULT_MAX_BLUR_REGIONS_COUNT>;

/// Indicates that this camera should render blur regions, as well as providing
/// settings for the blurring.
#[derive(Component, Debug, Clone, ExtractComponent, ShaderType)]
pub struct BlurRegionsCamera<const N: usize> {
    /// Increasing the radius increases the percieved blurriness.
    pub radius: f32,
    /// The number of steps to sample from the pixel being blurred to the edge of the radius.
    /// More steps leads to a higher quality blur, but at a performance cost.
    pub linear_steps: u32,
    /// The number of steps to sample radially around the pixel that is being blurred
    /// More steps leads to a higher quality blur, but at a performance cost.
    pub radial_steps: u32,
    current_regions_count: u32,
    regions: [ComputedBlurRegion; N],
}

impl Default for BlurRegionsCamera<DEFAULT_MAX_BLUR_REGIONS_COUNT> {
    fn default() -> Self {
        BlurRegionsCamera {
            radius: 0.025,
            linear_steps: 4,
            radial_steps: 16,
            current_regions_count: 0,
            regions: std::array::from_fn(|_| ComputedBlurRegion::OFFSCREEN),
        }
    }
}

impl<const N: usize> BlurRegionsCamera<N> {
    pub fn blur(&mut self, rect: Rect) {
        if self.current_regions_count == N as u32 {
            warn!("Blur region ignored as the max blur region count has already been reached");
            return;
        }

        self.regions[self.current_regions_count as usize] = ComputedBlurRegion {
            min_x: rect.min.x,
            max_x: rect.max.x,
            min_y: rect.min.y,
            max_y: rect.max.y,
        };
        self.current_regions_count += 1;
    }

    fn clear(&mut self) {
        self.current_regions_count = 0;
    }
}

fn clear_blur_regions<const N: usize>(mut blur_region_cameras: Query<&mut BlurRegionsCamera<N>>) {
    for mut blur_region in &mut blur_region_cameras {
        blur_region.clear();
    }
}

pub struct BlurRegionsPlugin<const N: usize>;

impl Default for BlurRegionsPlugin<DEFAULT_MAX_BLUR_REGIONS_COUNT> {
    fn default() -> Self {
        BlurRegionsPlugin::<DEFAULT_MAX_BLUR_REGIONS_COUNT>
    }
}

impl<const N: usize> Plugin for BlurRegionsPlugin<N> {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, clear_blur_regions::<N>)
            .add_systems(Last, crate::bevy_ui::compute_blur_regions::<N>)
            .add_plugins(crate::shader::BlurRegionsShaderPlugin::<N>);
    }
}
