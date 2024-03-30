mod bevy_ui;

use bevy::{
    asset::embedded_asset,
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{ComponentUniforms, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin},
        render_graph::{NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner},
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, CachedRenderPipelineId, ColorTargetState, ColorWrites,
            FragmentState, MultisampleState, Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderDefVal, ShaderStages, ShaderType,
            TextureFormat, TextureSampleType,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
        RenderApp,
    },
};

pub mod prelude {
    pub use super::{BlurRegion, BlurRegionsPlugin, DefaultBlurRegionsCamera as BlurRegionsCamera};
}

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
        embedded_asset!(app, "shader.wgsl");

        app.add_systems(PreUpdate, clear_blur_regions::<N>).add_systems(Last, bevy_ui::compute_blur_regions::<N>).add_plugins((
            ExtractComponentPlugin::<BlurRegionsCamera<N>>::default(),
            UniformComponentPlugin::<BlurRegionsCamera<N>>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<BlurRegionsNode<N>>>(Core3d, BlurRegionsLabel)
            .add_render_graph_edges(Core3d, (Node3d::Tonemapping, BlurRegionsLabel, Node3d::EndMainPassPostProcessing));
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<BlurRegionsPipeline<N>>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct BlurRegionsLabel;

#[derive(Default)]
struct BlurRegionsNode<const N: usize>;

impl<const N: usize> ViewNode for BlurRegionsNode<N> {
    type ViewQuery = (&'static ViewTarget, &'static BlurRegionsCamera<N>);

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, _blur_regions_camera): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let blur_regions_pipeline = world.resource::<BlurRegionsPipeline<N>>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let Some(pipeline) = pipeline_cache.get_render_pipeline(blur_regions_pipeline.pipeline_id) else {
            return Ok(());
        };

        let blur_regions = world.resource::<ComponentUniforms<BlurRegionsCamera<N>>>().uniforms();
        let Some(blur_regions_binding) = blur_regions.binding() else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "blur_regions_bind_group",
            &blur_regions_pipeline.layout,
            &BindGroupEntries::sequential((post_process.source, &blur_regions_pipeline.sampler, blur_regions_binding.clone())),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("blur_regions_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

#[derive(Resource)]
struct BlurRegionsPipeline<const N: usize> {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl<const N: usize> FromWorld for BlurRegionsPipeline<N> {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "blur_regions_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<BlurRegionsCamera<N>>(false),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world.resource::<AssetServer>().load("embedded://bevy_blur_regions/shader.wgsl");

        let pipeline_id = world.resource_mut::<PipelineCache>().queue_render_pipeline(RenderPipelineDescriptor {
            label: Some("blur_regions_pipeline".into()),
            layout: vec![layout.clone()],
            vertex: fullscreen_shader_vertex_state(),
            fragment: Some(FragmentState {
                shader,
                shader_defs: vec![ShaderDefVal::UInt("MAX_BLUR_REGIONS_COUNT".into(), N as u32)],
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
        });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}
