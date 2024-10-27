# Changelog

## 0.5.0 - 2024-10-27

### Support for the Visibility component in Bevy UI

Blur regions now respect the Visibility component of a node in Bevy UI. When a node is invisible, the blur region is not rendered. No additional config in application code is required to enable Visiblity support.

Thanks to Tam for contributing support for this.

### Support for 2D cameras

2D cameras are now supported. There are no additional configuration steps required to make use of blurring with 2D cameras, add the `BlurRegionsCamera` component to the camera whose output should be blurred, and then add create blur regions as normal.

### Added

- Support for Visibility on bevy ui nodes.
- Support for 2D cameras

## Changed

- egui: Upgrade to bevy_egui 0.30.

## 0.4.0 - 2024-09-19

### Support for rounded blur regions

Blur regions now support border radius! Use border radius as normal in Bevy UI or egui and the property will automatically be picked up by the shader, no additional config in application code is needed.

Thanks to rlofc for contributing support for this.

### Added

- Support for rounded blur regions

### Fixed

- Fix an issue where blur regions from removed egui windows would remain on screen.

## 0.3.0 - 2024-07-06

This release upgrades to Bevy 0.14 as well as introduces a number of changes.

#### Upgrade to Bevy 0.14

Bevy Blur Regions now supports Bevy 0.14!

### Improved blurring algorithm

The blurring is now a proper gaussian blur.

As apart of this improvement, the blur settings are now simplified. Applications that were previously making use of the blur settings will need to be updated. The `settings` example provides an interactive approach to gaining an understanding in how the settings influence the appearance of the blur.

```rust
// Before
blur_region_camera.radius = some_radius;
blur_region_camera.linear_steps = some_linear_steps;
blur_region_camera.radial_steps = some_radial_steps;

// After
blur_region_camera.circle_of_confusion = some_circle_of_confusion;
```

Thanks to pcwalton for the depth of field contribution to the bevy engine, which this feature is now based off.

#### Simplified egui integration

The `show_with_blur` interface that enables blurring of egui windows has been simplified.

This simplification is a breaking change. When upgrading from 0.2.0 to 0.3.0, calls to the `show_with_blur` function will need to be modified to remove the `id` and `blur_regions` parameters.

```rust
// Before
egui::Window::new("Blur").frame(frame).show_with_blur(
    egui::Id::new("Blur"),
    &mut blur_regions,
    contexts.ctx_mut(),
    |ui| ui.label("Blurry"),

// After
egui::Window::new("Blur").frame(frame).show_with_blur(
    contexts.ctx_mut(),
    |ui| ui.label("Blurry"),
);
```

### Added

- Add support for HDR render targets.
- Add support for deband dithering during tonemapping.

### Changed

- Improve the blurring algorithm. The new implementation utilises the gaussian blur implementation originally contributed to bevy for the depth of field feature.
- Change the settings on `BlurRegionsCamera` to only expose a new setting `circle_of_confusion` (previously it was `radius`, `linear_steps`, and `radial_steps`).
- egui: Removed the need to pass in the egui's window id into the `egui::Window::show_with_blur` function. The window id is now automatically detected.
- egui: Removed the need to pass in the `BlurRegionsCamera` into the `egui::Window::show_with_blur` function when there is only one `BlurRegionsCamera`.
- bevy: Upgrade to Bevy 0.14.
- egui: Upgrade to egui 0.28.

### Removed

## 0.2.0 - 2024-03-31

This release introduces out of the box integration with egui. Currently only egui windows are supported, check out the egui example to see how it works.

### Added

- Integration with egui windows.

### Changed

- Switched all units from device coordinates to physical pixels. This has the side effect of removing the hard-coded scale factor that was in the shader.
- Moved the Bevy UI integration behind the `bevy_ui` feature.

## 0.1.0 - 2024-03-30

Initial release of bevy_blur_regions

### Added

- Automatic blurring of Bevy UI nodes using the `BlurRegion` marker component.
- Immediate mode blurring API using `BlurRegionsCamera::blur`.
