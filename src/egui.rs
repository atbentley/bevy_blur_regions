use bevy::math::vec2;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::Rounding;
use bevy_egui::EguiContext;

use crate::core::DEFAULT_MAX_BLUR_REGIONS_COUNT;
use crate::BlurRegionsCamera;

pub struct BlurRegionsEguiPlugin<const N: usize>;

impl<const N: usize> Plugin for BlurRegionsEguiPlugin<N> {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, extract_egui_blurs::<N>);
    }
}

#[derive(Clone)]
enum EguiBlurTarget {
    DefaultCamera,
    Entity(Entity),
}

#[derive(Clone)]
struct EguiBlurRegions<const N: usize> {
    target: EguiBlurTarget,
    current_regions_count: u32,
    regions: [(Rect, Vec4); N],
}

impl<const N: usize> Default for EguiBlurRegions<N> {
    fn default() -> Self {
        EguiBlurRegions {
            target: EguiBlurTarget::DefaultCamera,
            current_regions_count: 0,
            regions: std::array::from_fn(|_| (Rect::new(-1.0, -1.0, -1.0, -1.0), Vec4::ZERO)),
        }
    }
}

impl<const N: usize> EguiBlurRegions<N> {
    pub fn blur(&mut self, rect: Rect, rounding: Vec4) {
        if self.current_regions_count == N as u32 {
            warn!("Blur region ignored as the max blur region count has already been reached");
            return;
        }

        self.regions[self.current_regions_count as usize] = (rect, rounding);
        self.current_regions_count += 1;
    }

    fn clear(&mut self) {
        self.current_regions_count = 0;
    }
}

pub trait EguiWindowBlurExt {
    fn show_with_blur<R>(
        self,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<Option<R>>>;

    fn show_with_blur_n<R, const N: usize>(
        self,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<Option<R>>>;

    fn show_with_blur_on_camera<R>(
        self,
        camera_entity: Entity,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<Option<R>>>;

    fn show_with_blur_on_camera_n<R, const N: usize>(
        self,
        camera_entity: Entity,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<Option<R>>>;
}

fn get_egui_blur_rect<R>(
    window: egui::Window<'_>,
    ctx: &egui::Context,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> Option<(egui::InnerResponse<Option<R>>, Rect, egui::Rounding)> {
    let mut rounding = egui::Rounding::ZERO;

    let Some(response) = window.show(ctx, |ui| {
        // When drawing a window, the frame for that window can be found on the UiStack of the grandparent of the current UiStack
        rounding = ui
            .stack()
            .parent
            .as_ref()
            .and_then(|s| s.parent.as_ref())
            .map(|s| s.frame().rounding)
            .unwrap_or(Rounding::ZERO);
        add_contents(ui)
    }) else {
        return None;
    };

    // egui appears to be painting one frame before bevy, so in order to ensure that the blur
    // is positioned exactly behind the window we need to ideally look at where the window was
    // one frame ago.
    let egui_rect =
        ctx.memory(|memory| memory.area_rect(response.response.layer_id.id)).unwrap_or(response.response.rect);

    let scale_factor = ctx.options(|op| op.zoom_factor);
    let min = vec2(egui_rect.min.x, egui_rect.min.y) * scale_factor;
    let max = vec2(egui_rect.max.x, egui_rect.max.y) * scale_factor;

    rounding = rounding * scale_factor;

    Some((response, Rect::from_corners(min, max), rounding))
}

impl<'open> EguiWindowBlurExt for egui::Window<'open> {
    fn show_with_blur<R>(
        self,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<Option<R>>> {
        self.show_with_blur_n::<R, DEFAULT_MAX_BLUR_REGIONS_COUNT>(ctx, add_contents)
    }

    fn show_with_blur_n<R, const N: usize>(
        self,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<Option<R>>> {
        let (response, blur_rect, rounding) = get_egui_blur_rect(self, ctx, add_contents)?;

        ctx.memory_mut(|mem| {
            let egui_blur_regions: &mut EguiBlurRegions<N> = mem.data.get_temp_mut_or_default(egui::Id::NULL);
            egui_blur_regions.target = EguiBlurTarget::DefaultCamera;
            egui_blur_regions.blur(blur_rect, Vec4::new(rounding.nw, rounding.ne, rounding.se, rounding.sw));
        });

        Some(response)
    }

    fn show_with_blur_on_camera<R>(
        self,
        camera_entity: Entity,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<Option<R>>> {
        self.show_with_blur_on_camera_n::<R, DEFAULT_MAX_BLUR_REGIONS_COUNT>(camera_entity, ctx, add_contents)
    }

    fn show_with_blur_on_camera_n<R, const N: usize>(
        self,
        camera_entity: Entity,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<Option<R>>> {
        let (response, blur_rect, rounding) = get_egui_blur_rect(self, ctx, add_contents)?;

        ctx.memory_mut(|mem| {
            let egui_blur_regions: &mut EguiBlurRegions<N> = mem.data.get_temp_mut_or_default(egui::Id::NULL);
            egui_blur_regions.target = EguiBlurTarget::Entity(camera_entity);
            egui_blur_regions.blur(blur_rect, Vec4::new(rounding.nw, rounding.ne, rounding.se, rounding.sw));
        });

        Some(response)
    }
}

pub fn extract_egui_blurs<const N: usize>(
    mut contexts: Query<&'static mut EguiContext>,
    mut blur_region_cameras: Query<&mut BlurRegionsCamera<N>>,
) {
    for mut context in &mut contexts {
        let ctx = context.get_mut();

        ctx.memory_mut(|mem| {
            let egui_blur_regions: &mut EguiBlurRegions<N> = mem.data.get_temp_mut_or_default(egui::Id::NULL);

            match egui_blur_regions.target {
                EguiBlurTarget::DefaultCamera => {
                    if let Ok(mut blur_regions) = blur_region_cameras.get_single_mut() {
                        blur_regions.rounded_blur_all(
                            &egui_blur_regions.regions[0..(egui_blur_regions.current_regions_count as usize)],
                        );
                    } else {
                        debug!("No default BlurRegionsCamera exists, skipping blurring.");
                    }
                }
                EguiBlurTarget::Entity(entity) => {
                    if let Ok(mut blur_regions) = blur_region_cameras.get_mut(entity) {
                        blur_regions.rounded_blur_all(
                            &egui_blur_regions.regions[0..(egui_blur_regions.current_regions_count as usize)],
                        );
                    } else {
                        debug!("No BlurRegionsCamera exists for entity {entity:?}, skipping blurring.");
                    }
                }
            };

            egui_blur_regions.clear();
        });
    }
}
