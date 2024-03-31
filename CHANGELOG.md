# Changelog

## [Unreleased]

### Added

### Changed

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
