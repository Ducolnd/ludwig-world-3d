use image::GenericImageView;
use anyhow::*;
use cgmath::Point2;
use image;

pub const TEXTURE_WIDTH: u32 = 16;
pub const TEXTURE_HEIGHT: u32 = 16;

pub const TEXTURE_IMAGE_HEIGHT: u32 = 256;
pub const TEXTURE_IMAGE_WIDTH: u32 = 256;


// Helper function
pub fn coords_to_float(coords: [u32; 2]) -> [f32; 2] {
    [
        coords[0] as f32 / TEXTURE_IMAGE_WIDTH as f32,
        coords[1] as f32 / TEXTURE_IMAGE_HEIGHT as f32,
    ]
}

pub struct TextureTile {
    pub coords: Point2<u32>, // Index coord of texture
}

impl TextureTile {
    pub fn to_usable(&self) -> [[f32; 2]; 4] {
        [
            coords_to_float([self.coords.x * TEXTURE_WIDTH, self.coords.y * TEXTURE_HEIGHT]),
            coords_to_float([self.coords.x * TEXTURE_WIDTH, self.coords.y * TEXTURE_HEIGHT + TEXTURE_HEIGHT]),
            coords_to_float([self.coords.x * TEXTURE_WIDTH + TEXTURE_WIDTH, self.coords.y * TEXTURE_HEIGHT + TEXTURE_HEIGHT]),
            coords_to_float([self.coords.x * TEXTURE_WIDTH + TEXTURE_WIDTH, self.coords.y * TEXTURE_HEIGHT]),
        ]
    }
}

pub struct TextureManager {
    pub textures: Vec<wgpu::BindGroup>, // Todo support multple textures

    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl TextureManager {
    pub fn new(device: &wgpu::Device) -> Self {
        let texture_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {filterable: false,},
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                            filtering: false,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            }
        );

        let textures = Vec::<wgpu::BindGroup>::new();
        
        Self {
            textures,
            texture_bind_group_layout,
        }
    }

    pub fn load(&mut self, path: &str, device: &wgpu::Device, queue: &wgpu::Queue) {
        let diffuse_texture = Texture::from_image(&device, &queue, path, Some("A texture")).unwrap();

        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &self.texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        self.textures.push(diffuse_bind_group);
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.textures[0]
    }
}


pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
        label: Option<&str>
    ) -> Result<Self> {
        let img = image::open(path).unwrap();

        let rgba = img.as_rgba8().unwrap();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            }
        );

        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );
        
        Ok(Self { texture, view, sampler })
    }
}