#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::maths::PI

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> blur_regions: BlurRegionsCamera;

struct BlurRegionsCamera {
    radius: f32,
    linear_steps: u32,
    radial_steps: u32,
    current_regions_count: u32,
    regions: array<ComputedBlurRegion, #{MAX_BLUR_REGIONS_COUNT}>,
}

struct ComputedBlurRegion {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

fn is_blurred(position: vec2<f32>) -> bool {
    for (var i = 0; u32(i) < blur_regions.current_regions_count; i++ ) {
        if position.x > blur_regions.regions[i].min_x
            && position.x < blur_regions.regions[i].max_x
            && position.y > blur_regions.regions[i].min_y
            && position.y < blur_regions.regions[i].max_y {
            return true;
        }
    }
    return false;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(screen_texture, texture_sampler, in.uv);

    let screen_size = vec2<f32>(textureDimensions(screen_texture).xy);
    let position = in.uv * screen_size;

    if !is_blurred(position) {
        return color;
    }

    // If 16 pixels around the original pixel are sampled for blurring, each one should
    // contribute 1/17th of the final colour. The extra +1 comes from the original pixel.
    let blur_contribution = 1.0 / f32(blur_regions.radial_steps * blur_regions.linear_steps + 1);
    color = color * blur_contribution;

    let radius = vec2<f32>(blur_regions.radius / screen_size.x, blur_regions.radius / screen_size.y);
    let radial_step_size = 2.0 * PI / f32(blur_regions.radial_steps);
    let linear_step_size = 1.0 / f32(blur_regions.linear_steps);

    for(var radial_progress = 0.0; radial_progress <= 2.0 * PI; radial_progress += radial_step_size) {
		for(var liear_progress = linear_step_size; liear_progress <= 1.0; liear_progress += linear_step_size) {
            let blurring_pixel = vec2(cos(radial_progress), sin(radial_progress)) * radius * liear_progress;
            color += textureSample(screen_texture, texture_sampler, in.uv + blurring_pixel) * blur_contribution;
        }
    }

    return color;
}

