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