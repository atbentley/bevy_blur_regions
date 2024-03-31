mod bevy_ui;
mod core;
mod shader;

pub mod prelude {
    pub use super::BlurRegion;
    pub use super::BlurRegionsCamera;
    pub use super::BlurRegionsPlugin;
    pub use super::DefaultBlurRegionsCamera;
}

pub use core::BlurRegion;
pub use core::BlurRegionsCamera;
pub use core::BlurRegionsPlugin;
pub use core::DefaultBlurRegionsCamera;
pub use shader::BlurRegionsLabel;
