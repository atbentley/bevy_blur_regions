use bevy::math::vec2;
use bevy::prelude::*;
use bevy_egui::egui;

use crate::BlurRegionsCamera;

pub trait EguiWindowBlurExt {
    fn show_with_blur<R, const N: usize>(
        self,
        id: egui::Id,
        window: &Window,
        blur_regions: &mut BlurRegionsCamera<N>,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<Option<R>>>;
}

impl<'open> EguiWindowBlurExt for egui::Window<'open> {
    fn show_with_blur<R, const N: usize>(
        self,
        id: egui::Id,
        window: &Window,
        blur_regions: &mut BlurRegionsCamera<N>,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<Option<R>>> {
        let self_with_id = self.id(id);

        let Some(response) = self_with_id.show(ctx, add_contents) else {
            return None;
        };

        // egui appears to be painting one frame before bevy, so in order to ensure that the blur
        // is positioned exactly behind the window we need to ideally look at where the window was
        // one frame ago.
        let egui_rect = ctx.memory(|memory| memory.area_rect(id)).unwrap_or(response.response.rect);

        let screen_size = vec2(window.resolution.width(), window.resolution.height());
        let min = vec2(egui_rect.min.x, egui_rect.min.y) / screen_size;
        let max = vec2(egui_rect.max.x, egui_rect.max.y) / screen_size;
        let blur_rect = Rect::from_corners(min, max);
        blur_regions.blur(blur_rect);

        Some(response)
    }
}
