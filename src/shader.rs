use bevy::asset::embedded_asset;
use bevy::core_pipeline::core_3d::graph::Core3d;
use bevy::core_pipeline::core_3d::graph::Node3d;
use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::extract_component::ComponentUniforms;
use bevy::render::extract_component::ExtractComponentPlugin;
use bevy::render::extract_component::UniformComponentPlugin;
use bevy::render::render_graph::NodeRunError;
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::render_graph::RenderGraphContext;
use bevy::render::render_graph::RenderLabel;
use bevy::render::render_graph::ViewNode;
use bevy::render::render_graph::ViewNodeRunner;
use bevy::render::render_resource::binding_types::sampler;
use bevy::render::render_resource::binding_types::texture_2d;
use bevy::render::render_resource::binding_types::uniform_buffer;
use bevy::render::render_resource::BindGroupEntries;
use bevy::render::render_resource::BindGroupLayout;
use bevy::render::render_resource::BindGroupLayoutEntries;
use bevy::render::render_resource::CachedRenderPipelineId;
use bevy::render::render_resource::ColorTargetState;
use bevy::render::render_resource::ColorWrites;
use bevy::render::render_resource::FragmentState;
use bevy::render::render_resource::MultisampleState;
use bevy::render::render_resource::Operations;
use bevy::render::render_resource::PipelineCache;
use bevy::render::render_resource::PrimitiveState;
use bevy::render::render_resource::RenderPassColorAttachment;
use bevy::render::render_resource::RenderPassDescriptor;
use bevy::render::render_resource::RenderPipelineDescriptor;
use bevy::render::render_resource::Sampler;
use bevy::render::render_resource::SamplerBindingType;
use bevy::render::render_resource::SamplerDescriptor;
use bevy::render::render_resource::ShaderDefVal;
use bevy::render::render_resource::ShaderStages;
use bevy::render::render_resource::TextureFormat;
use bevy::render::render_resource::TextureSampleType;
use bevy::render::renderer::RenderContext;
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::BevyDefault;
use bevy::render::view::ViewTarget;
use bevy::render::RenderApp;

use crate::BlurRegionsCamera;

pub struct BlurRegionsShaderPlugin<const N: usize>;

impl<const N: usize> Plugin for BlurRegionsShaderPlugin<N> {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "shader.wgsl");

        app.add_plugins((
            ExtractComponentPlugin::<BlurRegionsCamera<N>>::default(),
            UniformComponentPlugin::<BlurRegionsCamera<N>>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<BlurRegionsNode<N>>>(Core3d, BlurRegionsLabel)
            .add_render_graph_edges(
                Core3d,
                (Node3d::Tonemapping, BlurRegionsLabel, Node3d::EndMainPassPostProcessing),
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<BlurRegionsPipeline<N>>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct BlurRegionsLabel;

#[derive(Default)]
pub struct BlurRegionsNode<const N: usize>;

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
            &BindGroupEntries::sequential((
                post_process.source,
                &blur_regions_pipeline.sampler,
                blur_regions_binding.clone(),
            )),
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
pub struct BlurRegionsPipeline<const N: usize> {
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
