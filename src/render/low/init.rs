use wgpu;
use crate::render::low::vertex::Vertex;

pub fn default_depth_texture(
    device: &wgpu::Device, 
    sc_desc: &wgpu::SwapChainDescriptor,
) -> (wgpu::Texture, wgpu::TextureView, wgpu::Sampler) {

    let desc = wgpu::TextureDescriptor {
        label: Some("depth texture"),
        size : wgpu::Extent3d {
            width: sc_desc.width,
            height: sc_desc.height,
            depth: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT // 3.
            | wgpu::TextureUsage::SAMPLED,
    };

    let texture = device.create_texture(&desc);
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let sampler = device.create_sampler(
        &wgpu::SamplerDescriptor { // 4.
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual), // 5.
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        }
    );
    
    (texture, view, sampler)
}

pub fn default_render_pipeline(
    device: &wgpu::Device, 
    vs_module: &wgpu::ShaderModule, 
    fs_module: &wgpu::ShaderModule,
    sc_desc: &wgpu::SwapChainDescriptor,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::RenderPipeline {

    let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: bind_group_layouts,
            push_constant_ranges: &[],
        });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        vertex: wgpu::VertexState {
            module: &vs_module,
            entry_point: "main", 
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState { 
            module: &fs_module,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: sc_desc.format,
                color_blend: wgpu::BlendState::REPLACE,
                alpha_blend: wgpu::BlendState::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::None,
            polygon_mode: wgpu::PolygonMode::Fill,
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
            clamp_depth: false,
        }),
    })
}