# Bevy Blur Regions

A [Bevy](https://github.com/bevyengine/bevy) plugin to selectively blur regions of the screen.

![screenshot](content/screenshot.png)

## Usage

Add the plugin:

```rust
use bevy::prelude::*;
use bevy_blur_regions::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BlurRegionsPlugin::default())
        .run();
}
```

Add the `BlurRegionsCamera` component to the 3D camera whose output should be blurred:

```rust
commands.spawn((
    BlurRegionsCamera::default(),
    Camera3dBundle::default(),
));
```

When using Bevy UI, add the `BlurRegion` marker component to any Bevy UI nodes which should have a blurry background:

```rust
commands.spawn((
    BlurRegion,
    NodeBundle {
        style: Style {
            width: Val::Percent(50.0),
            height: Val::Percent(50.0),
            left: Val::Percent(25.0),
            top: Val::Percent(25.0),
            ..default()
        },
        ..default()
    },
));
```

For non Bevy UI use cases the immediate mode blurring api can be used:

```rust
// Immediate mode blurring must be called each frame
fn update(mut blur_region_cameras: Query<&mut BlurRegionsCamera>) {
    let Ok(mut blur_regions) = blur_region_cameras.get_single_mut() else {
        return;
    };
    blur_regions.blur(Rect::from_center_size(vec2(0.5, 0.5), vec2(0.5, 0.5)));
}
```

## Caveats

### The number of blur regions is limited

The number of blur regions that can be present on the screen at the same time is limited to 20.

## License

All code in this repository is dual-licensed under either:

    MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
    Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.
