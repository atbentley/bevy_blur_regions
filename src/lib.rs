#[cfg(feature = "bevy_ui")]
mod bevy_ui;
mod core;
#[cfg(feature = "egui")]
mod egui;
mod shader;

pub mod prelude {
    pub use super::BlurRegion;
    pub use super::BlurRegionsCamera;
    pub use super::BlurRegionsPlugin;
    pub use super::DefaultBlurRegionsCamera;

    #[cfg(feature = "egui")]
    pub use super::EguiWindowBlurExt;
}

pub use core::BlurRegion;
pub use core::BlurRegionsCamera;
pub use core::BlurRegionsPlugin;
pub use core::DefaultBlurRegionsCamera;
pub use shader::BlurRegionsLabel;

#[cfg(feature = "egui")]
pub use crate::egui::EguiWindowBlurExt;
