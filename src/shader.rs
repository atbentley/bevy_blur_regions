use bevy::asset::load_internal_asset;
use bevy::core_pipeline::core_2d::graph::Core2d;
use bevy::core_pipeline::core_2d::graph::Node2d;
use bevy::core_pipeline::core_3d::graph::Core3d;
use bevy::core_pipeline::core_3d::graph::Node3d;
use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::ecs::query::QueryItem;
use bevy::ecs::system::lifetimeless::Read;
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
use bevy::render::render_resource::SpecializedRenderPipeline;
use bevy::render::render_resource::SpecializedRenderPipelines;
use bevy::render::render_resource::TextureFormat;
use bevy::render::render_resource::TextureSampleType;
use bevy::render::renderer::RenderContext;
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::BevyDefault;
use bevy::render::view::ExtractedView;
use bevy::render::view::ViewTarget;
use bevy::render::Render;
use bevy::render::RenderApp;
use bevy::render::RenderSet;

use crate::BlurRegionsCamera;

const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(271147050642476932735403127655134602927);

pub struct BlurRegionsShaderPlugin<const N: usize>;

impl<const N: usize> Plugin for BlurRegionsShaderPlugin<N> {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER_HANDLE, "shader.wgsl", Shader::from_wgsl);

        app.add_plugins((
            ExtractComponentPlugin::<BlurRegionsCamera<N>>::default(),
            UniformComponentPlugin::<BlurRegionsCamera<N>>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<SpecializedRenderPipelines<BlurRegionsPipeline<N>>>()
            .add_systems(
                Render,
                (prepare_blur_regions_pipelines::<N>.in_set(RenderSet::Prepare),),
            )
            .add_render_graph_node::<ViewNodeRunner<BlurRegionsNode<N>>>(Core3d, BlurRegionsLabel)
            .add_render_graph_edges(Core3d, (Node3d::DepthOfField, BlurRegionsLabel, Node3d::Tonemapping))
            .add_render_graph_node::<ViewNodeRunner<BlurRegionsNode<N>>>(Core2d, BlurRegionsLabel)
            .add_render_graph_edges(Core2d, (Node2d::Bloom, BlurRegionsLabel, Node2d::Tonemapping));
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        let render_device = render_app.world().resource::<RenderDevice>().clone();
        render_app.insert_resource(BlurRegionsPipeline::<N>::new(&render_device));
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct BlurRegionsLabel;

#[derive(Default)]
pub struct BlurRegionsNode<const N: usize>;

impl<const N: usize> ViewNode for BlurRegionsNode<N> {
    type ViewQuery = (Read<ViewTarget>, Read<BlurRegionsPasses>);

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, passes): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let blur_regions_pipeline = world.resource::<BlurRegionsPipeline<N>>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let blur_regions = world.resource::<ComponentUniforms<BlurRegionsCamera<N>>>().uniforms();
        let Some(blur_regions_binding) = blur_regions.binding() else {
            return Ok(());
        };

        for pass in &passes.0 {
            let Some(pass_pipeline) = pipeline_cache.get_render_pipeline(pass.pipeline) else {
                return Ok(());
            };

            let post_process = view_target.post_process_write();

            let bind_group = render_context.render_device().create_bind_group(
                pass.bind_group_label,
                &blur_regions_pipeline.layout,
                &BindGroupEntries::sequential((
                    post_process.source,
                    &blur_regions_pipeline.sampler,
                    blur_regions_binding.clone(),
                )),
            );

            let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                label: Some(pass.pass_label),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: post_process.destination,
                    resolve_target: None,
                    ops: Operations::default(),
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_render_pipeline(pass_pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        Ok(())
    }
}

#[derive(Resource)]
pub struct BlurRegionsPipeline<const N: usize> {
    layout: BindGroupLayout,
    sampler: Sampler,
}

impl<const N: usize> BlurRegionsPipeline<N> {
    fn new(render_device: &RenderDevice) -> Self {
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

        Self { layout, sampler }
    }
}

#[derive(Component)]
pub struct BlurRegionsPasses([BlurRegionsPass; 2]);

pub struct BlurRegionsPass {
    pass_label: &'static str,
    bind_group_label: &'static str,
    pipeline: CachedRenderPipelineId,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BlurRegionsPipelineKey {
    pass: BlurRegionsPassKey,
    hdr: bool,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum BlurRegionsPassKey {
    Horizontal,
    Vertical,
}

fn prepare_blur_regions_pipelines<const N: usize>(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    mut pipelines: ResMut<SpecializedRenderPipelines<BlurRegionsPipeline<N>>>,
    pipeline: Res<BlurRegionsPipeline<N>>,
    views: Query<(Entity, &ExtractedView), With<BlurRegionsCamera<N>>>,
) {
    for (entity, view) in &views {
        let horizontal_pass = BlurRegionsPass {
            pass_label: "blur regions (horizontal pass)",
            bind_group_label: "blur regions bind group (horizontal pass)",
            pipeline: pipelines.specialize(
                &pipeline_cache,
                &pipeline,
                BlurRegionsPipelineKey {
                    pass: BlurRegionsPassKey::Horizontal,
                    hdr: view.hdr,
                },
            ),
        };

        let vertical_pass = BlurRegionsPass {
            pass_label: "blur regions (vertical pass)",
            bind_group_label: "blur regions bind group (vertical pass)",
            pipeline: pipelines.specialize(
                &pipeline_cache,
                &pipeline,
                BlurRegionsPipelineKey {
                    pass: BlurRegionsPassKey::Vertical,
                    hdr: view.hdr,
                },
            ),
        };

        commands.entity(entity).insert(BlurRegionsPasses([horizontal_pass, vertical_pass]));
    }
}

impl<const N: usize> SpecializedRenderPipeline for BlurRegionsPipeline<N> {
    type Key = BlurRegionsPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("blur_regions_pipeline".into()),
            layout: vec![self.layout.clone()],
            vertex: fullscreen_shader_vertex_state(),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
            fragment: Some(FragmentState {
                shader: SHADER_HANDLE,
                shader_defs: vec![ShaderDefVal::UInt("MAX_BLUR_REGIONS_COUNT".into(), N as u32)],
                entry_point: match key.pass {
                    BlurRegionsPassKey::Horizontal => "horizontal".into(),
                    BlurRegionsPassKey::Vertical => "vertical".into(),
                },
                targets: vec![Some(ColorTargetState {
                    format: if key.hdr {
                        ViewTarget::TEXTURE_FORMAT_HDR
                    } else {
                        TextureFormat::bevy_default()
                    },
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
        }
    }
}
