use wgpu;
use wgpu::util::DeviceExt;
use std::hash::Hash;

/// A wrapper around the wgpu::Buffer.
/// Holds multple objects.
/// Write only, not read.
pub struct DynamicBuffer <T: bytemuck::Pod + bytemuck::Zeroable> {
    buffer: wgpu::Buffer,
    usage: wgpu::BufferUsage,
    phantom: std::marker::PhantomData<T>,

    size: usize, // Reserved size
    pub len: usize, // Actual size
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> DynamicBuffer<T> {

    pub fn new (
        initial_size: usize, 
        device: &wgpu::Device,
        usage: wgpu::BufferUsage,
    ) -> Self {

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Dynamic Buffer"),
            size: (initial_size * std::mem::size_of::<T>()) as u64,
            usage: usage | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            usage,
            size: initial_size,
            len: 0,

            phantom: std::marker::PhantomData,
        }
    }

    // Will insert starting at the 'at'th type T
    fn insert_at(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        data: &[T],
        at: usize,
    ) {
        self.insert_at_byte(device, encoder, data, at * std::mem::size_of::<T>())
    }

    // Insert at byte. Will replace anything, also through
    // object borders
    fn insert_at_byte(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        data: &[T],
        at_byte: usize,
    ) {
        if data.len() + self.len > self.size {
            self.resize((data.len() + self.len) * 2, device, encoder); // Double size if needed
        }

        let to_add_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("To add Buffer"),
            usage: wgpu::BufferUsage::COPY_SRC | self.usage,
            contents: bytemuck::cast_slice(data),
        });

        encoder.copy_buffer_to_buffer(
            &to_add_buffer,
            0,
            &self.buffer,
            at_byte as u64,
            (data.len() * std::mem::size_of::<T>()) as u64,
        );

        self.len += data.len();
    }

    // Append to the buffer
    pub fn insert_back(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        data: &[T],
    ) {
        if data.len() + self.len > self.size {
            self.resize((data.len() + self.len) * 2, device, encoder); // Resize twice as much as is needed
        }

        let to_add_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("To add Buffer"),
            usage: wgpu::BufferUsage::COPY_SRC | self.usage,
            contents: bytemuck::cast_slice(data),
        });

        // Copy the new data into the buffer
        encoder.copy_buffer_to_buffer(
            &to_add_buffer,
            0,
            &self.buffer,
            (self.len * std::mem::size_of::<T>()) as u64,
            (data.len() * std::mem::size_of::<T>()) as u64,
        );

        self.len += data.len();
    }

    pub fn resize(
        &mut self, 
        new_size: usize,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let new_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            mapped_at_creation: false,
            size: (new_size * std::mem::size_of::<T>()) as u64,
            usage: self.usage | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC,
        });
        encoder.copy_buffer_to_buffer(
            &self.buffer,
            0,
            &new_buffer,
            0,
            (self.len * std::mem::size_of::<T>()) as u64,
        );
        self.buffer = new_buffer;

        self.size = new_size;
    }

    pub fn remove_at(
        &mut self, 
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
    ) {

    }

    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}

struct Segment {
    size: usize,
    free: bool,
}