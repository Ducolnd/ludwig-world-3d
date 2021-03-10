use wgpu;
use wgpu::util::DeviceExt;
use winit::event::{WindowEvent, DeviceEvent, KeyboardInput, ElementState};
use std::collections::HashMap;

use crate::render::vertexarray::VertexArray;
use crate::render::shapes::shape::Shape;
use crate::render::low::renderer::Renderer;
use crate::render::low::buffer::DynamicBuffer;
use crate::render::camera::{Camera, CameraController};
use crate::render::low::uniforms::{CameraUniform, MultiUniform, ChunkPositionUniform, Uniform};
use crate::render::{
    low::{
        textures::{Texture, TextureManager},
        vertex::Vertex,
        init::default_depth_texture,
    },
};

use crate::world::{
    world::World,
    chunk::pos::ChunkPos,
};

/// The Master owns all low level items such as the device.
/// It also hands out Dynamic Buffers and other device/encoder
/// related things.
pub struct Master {
    // General gpu setup
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,

    pub shaders: HashMap<String, wgpu::ShaderModule>,

    // Rendering
    pub renderer: Renderer,
    // Camera
    pub camera: Camera,
    // Textures
    pub texture_manager: TextureManager,
    // Chunk stuff
    pub chunkpos_uniform: MultiUniform<ChunkPos, ChunkPositionUniform>,
}

impl Master {
    // Wgpu requiers async
    pub async fn new(window: &winit::window::Window) -> Self {

        // General setup
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: Some("Device descriptior"),
            },
            None,
        ).await.unwrap();

        let sc_desc = wgpu::SwapChainDescriptor { // How should the swap chain be used?
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT, // Texture usage
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let camera = Camera::new(&device, sc_desc.width, sc_desc.height, (0.0, 0.0, 0.0).into());

        // Chunkpos Uniform setup
        let chunkpos_uniform = MultiUniform::new(&device, 3, 1); // At binding 3 and index 1 in pipeline

        // Load a texture
        let mut texture_manager = TextureManager::new(&device);
        texture_manager.load("/home/duco/development/rust/gamedev/luwdigengine2d/assets/terrain.png", &device, &queue);

        let renderer = Renderer::new(
            &device,
            &sc_desc,
            &camera.uniform.uniform_bind_group_layout,
            &chunkpos_uniform.uniform_bind_group_layout,
            &texture_manager,
        );

        let shaders = Master::load_shader(&device);

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,

            shaders,

            renderer,
            camera,

            texture_manager,

            chunkpos_uniform,
        }
    }
}

impl Master {
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        self.renderer.render(
            &self.device, 
            &mut self.swap_chain, 
            &self.queue,
            &self.camera.uniform,
            &mut self.chunkpos_uniform,
            &self.texture_manager,
        )
    }

    /// Create a new buffer at pos
    pub fn new_buffer<T: Shape>(&mut self, vertex_array: &VertexArray<T>, pos: ChunkPos) {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let mut vertex_buffer = DynamicBuffer::new(
            8000,
            &self.device,
            wgpu::BufferUsage::VERTEX,
        );

        let mut index_buffer = DynamicBuffer::new(
            8000,
            &self.device,
            wgpu::BufferUsage::INDEX,
        );

        vertex_buffer.insert_back(
            &self.device, 
            &mut encoder,
            &vertex_array.to_vertices(),
        );

        index_buffer.insert_back(
            &self.device, 
            &mut encoder,
            &vertex_array.to_indices(),
        );

        self.queue.submit(vec![encoder.finish()]);

        self.renderer.vertex_buffer.insert(pos, vertex_buffer);
        self.renderer.index_buffer.insert(pos, index_buffer);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // Update swap chain
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);

        // Update depth texture
        let (_, depth_view, _) = default_depth_texture(&self.device, &self.sc_desc);
        self.renderer.depth_view = depth_view;

        // Update camera
        self.camera.projection.resize(new_size.width, new_size.height);

        println!("New screensize: {}x{}", new_size.width, new_size.height);
    }

    /// Panics if the given shader is not loaded
    pub fn default_pipeline(
        &mut self,
        vs_module: String,
        fs_module: String,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> wgpu::RenderPipeline {
    
        let render_pipeline_layout =
            self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: bind_group_layouts,
                push_constant_ranges: &[],
            });
    
            self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            vertex: wgpu::VertexState {
                module: &self.shaders.get(&vs_module).unwrap(),
                entry_point: "main", 
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState { 
                module: &self.shaders.get(&fs_module).unwrap(),
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: self.sc_desc.format,
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

    pub fn input(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::Key(
                KeyboardInput {
                    virtual_keycode: Some(key),
                    state,
                    ..
                }
            ) => self.camera.controller.process_keyboard(*key, *state),
            DeviceEvent::MouseWheel { delta, .. } => {
                self.camera.controller.process_scroll(delta);
                true
            }
            DeviceEvent::Button {
                button: 1, // Left Mouse Button
                state,
            } => {
                self.camera.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            DeviceEvent::MouseMotion { delta } => {
                if self.camera.mouse_pressed {
                    self.camera.controller.process_mouse(delta.0, delta.1);
                }
                true
            }
            _ => false,
        }
    }

    pub fn load_shader(device: &wgpu::Device) -> HashMap<String, wgpu::ShaderModule> {
        let map = HashMap::new();

        map.insert("fragment".to_string(), device.create_shader_module(wgpu::include_spirv!("shaders/shader.frag.spv")));
        map.insert("vertex".to_string(), device.create_shader_module(wgpu::include_spirv!("shaders/shader.vert.spv")));

        map
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.camera.controller.update_camera(&mut self.camera.view, dt);
        self.camera.update(&self.queue);
    }

    pub fn update_player(&self, world: &mut World) {
        world.player.position = self.camera.view.position;
    }
}