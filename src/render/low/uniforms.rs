use crate::render::camera::Camera;
use wgpu::util::DeviceExt;
use std::collections::HashMap;
use std::hash::Hash;

use crate::render::low::buffer::DynamicBuffer;

/// A Uniform Buffer that can store multple things of T.
/// In the renderpass the offset should be set accordingly
pub struct MultiUniform<K: Hash + Eq + Copy, T: bytemuck::Pod + bytemuck::Zeroable> {
    pub buffer: DynamicBuffer<T>,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group: wgpu::BindGroup,
    pub offset: HashMap<K, u32>,

    pub index: u32,
    pub binding: u32,
    
    size: u32, // How many items of T
}

impl<K: Hash + Eq + Copy, T: bytemuck::Pod + bytemuck::Zeroable> MultiUniform<K, T> {
    pub fn new(device: &wgpu::Device, binding: u32, index: u32) -> Self {
        let t_size = std::mem::size_of::<T>() as u64;

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: binding,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: true,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<T>() as u64),
                    },
                    count: None,
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });

        let buffer = DynamicBuffer::new(2000, device, wgpu::BufferUsage::UNIFORM);

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: binding,
                    resource: wgpu::BindingResource::Buffer(buffer.get_buffer().slice(..t_size)),
                }
            ],
            label: Some("uniform_bind_group"),
        });

        let offset = HashMap::new();

        Self {
            buffer,
            uniform_bind_group,
            uniform_bind_group_layout,
            offset,

            index,
            binding,

            size: 0,
        }
    }

    pub fn add(&mut self, queue: &wgpu::Queue, at: K, data: T) {
        self.offset.insert(at, self.size);

        queue.write_buffer(
            &self.buffer.get_buffer(), 
            self.size as u64 * wgpu::BIND_BUFFER_ALIGNMENT, 
            bytemuck::cast_slice(&[data])
        );

        self.size += 1;
    }

    pub fn modify(&mut self, queue: &wgpu::Queue, at: K, data: T) {
        let offset = self.offset.get(&at).unwrap();

        queue.write_buffer(
            &self.buffer.get_buffer(), 
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
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
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
                    resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..))
                }
            ],
            label: Some("uniform_bind_group"),
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

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}