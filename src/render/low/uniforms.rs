use wgpu::util::DeviceExt;
use std::collections::HashMap;
use std::hash::Hash;

/// A Uniform Buffer that can store multple things of T.
/// In the renderpass the offset should be set accordingly. K is the type for indexing, T the data.
pub struct MultiUniform<K: Hash + Eq + Copy, T: bytemuck::Pod + bytemuck::Zeroable> {
    pub buffer: wgpu::Buffer,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group: wgpu::BindGroup,
    pub offset: HashMap<K, u32>, // Array of offsets
    open_spots: Vec<u32>,

    pub index: u32,
    pub binding: u32,
    
    phantom: std::marker::PhantomData<T>,
}

impl<K: Hash + Eq + Copy, T: bytemuck::Pod + bytemuck::Zeroable> MultiUniform<K, T> {
    pub fn new(device: &wgpu::Device, binding: u32, index: u32) -> Self {
        let t_size = std::mem::size_of::<T>() as u64;

        assert!(t_size < wgpu::BIND_BUFFER_ALIGNMENT);

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: binding,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(t_size),
                    },
                    count: None,
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: wgpu::BIND_BUFFER_ALIGNMENT * 40,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: binding,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &buffer,
                        offset: 0,
                        size: wgpu::BufferSize::new(wgpu::BIND_BUFFER_ALIGNMENT), // Meaning that T cannot be bigger than 256 bytes
                    },
                }
            ],
            label: Some("uniform_bind_group multiuniform"),
        });

        let offset = HashMap::new();

        Self {
            buffer,
            uniform_bind_group,
            uniform_bind_group_layout,
            offset,
            open_spots: (0..40).collect(),

            index,
            binding,

            phantom: std::marker::PhantomData,
        }
    }

    pub fn add(&mut self, queue: &wgpu::Queue, at: K, data: T) {
        self.offset.insert(at, self.open_spots[0]);

        queue.write_buffer(
            &self.buffer, 
            self.open_spots[0] as u64 * wgpu::BIND_BUFFER_ALIGNMENT, // This goes with the assumption that T is never bigger than BIND_BUFFER_ALIGNMENT (256 bytes)
            bytemuck::cast_slice(&[data])
        );

        self.open_spots.remove(0);
    }

    pub fn remove(&mut self, at: &K) {
        self.open_spots.insert(0, *self.offset.get(at).unwrap());
        self.offset.remove(at);
    }
    
    #[allow(dead_code)]
    pub fn modify(&mut self, queue: &wgpu::Queue, at: K, data: T) {
        let offset = self.offset.get(&at).unwrap();

        queue.write_buffer(
            &self.buffer, 
            *offset as u64 * wgpu::BIND_BUFFER_ALIGNMENT, 
            bytemuck::cast_slice(&[data])
        );
    }
}

pub struct Uniform<T: bytemuck::Pod + bytemuck::Zeroable> {
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group: wgpu::BindGroup,

    pub index: u32,

    pub data: T,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> Uniform<T> {

    pub fn new(device: &wgpu::Device, data: T, binding: u32, index: u32) -> Self {
        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Unifor Buffer at binding"),
                contents: bytemuck::cast_slice(&[data]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        );
    
        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: binding,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });
    
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: binding,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buffer,
                        offset: 0,
                        size: None,
                    },
                }
            ],
            label: Some("uniform_bind_group simple uniform"),
        });
    
        Self { 
            uniform_buffer, 
            uniform_bind_group_layout, 
            uniform_bind_group,

            index,

            data,
        }
    }

    pub fn update(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.data]));
    }
}

// Uniform buffers are buffers that are available to every
// shader instance. 

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ChunkPositionUniform {
    pub location: [f32; 3]
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, data: cgmath::Matrix4<f32>) {
        self.view_proj = data.into();
    }
}