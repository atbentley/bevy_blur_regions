use bevy::math::vec2;
use bevy::prelude::*;
use bevy_egui::egui;

use crate::BlurRegionsCamera;

pub trait EguiWindowBlurExt {
    fn show_with_blur<R, const N: usize>(
        self,
        blur_regions: &mut BlurRegionsCamera<N>,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<Option<R>>>;
}

impl<'open> EguiWindowBlurExt for egui::Window<'open> {
    fn show_with_blur<R, const N: usize>(
        self,
        blur_regions: &mut BlurRegionsCamera<N>,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<Option<R>>> {
        let Some(response) = self.show(ctx, add_contents) else {
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

        let blur_rect = Rect::from_corners(min, max);
        blur_regions.blur(blur_rect);

        Some(response)
    }
}
