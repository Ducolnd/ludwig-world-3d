use wgpu;
use winit::{
    event::DeviceEvent,
    dpi::PhysicalSize,
    window::Window,
};

use std::any::TypeId;
use std::collections::HashMap;

use crate::render::{
    low::{
        init::default_depth_texture,
        vertex::Vertex,
        textures::TextureManager,
        uniforms::{MultiUniform, ChunkPositionUniform},
    },
    camera::Camera,
    drawables::{Drawable, texture_vertex::TextureVertex, chunk::ChunkDrawable},
};
use crate::world::chunk::pos::ChunkPos;

pub struct Renderer {
    // General gpu setup
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub size: PhysicalSize<u32>,

    // Other
    pub camera: Camera,
    pub textures: TextureManager,
    pub chunkpos_uniform: MultiUniform<ChunkPos, ChunkPositionUniform>,

    // Used when rendering
    pub pipelines: HashMap<TypeId, wgpu::RenderPipeline>,
    pub depth_view: wgpu::TextureView,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {

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

        let (_, depth_view, _) = default_depth_texture(&device, &sc_desc);
        let camera = Camera::new(&device, sc_desc.width, sc_desc.height, cgmath::Point3 {x: 0.0, y: 0.0, z: 0.0});

        // Load a texture
        let mut textures = TextureManager::new(&device);
        textures.load("/home/duco/development/rust/gamedev/luwdigengine2d/assets/terrain.png", &device, &queue);

        let mut chunkpos_uniform = MultiUniform::new(&device, 3, 2);
        chunkpos_uniform.add(&queue, ChunkPos::new(0, 0, 0), ChunkPos::new(0, 0, 0).to_raw());

        let mut t = Self {
            size,
            surface,
            queue,
            sc_desc,
            swap_chain,
            device,

            camera,
            textures: textures,
            chunkpos_uniform,

            pipelines: HashMap::new(),
            depth_view,
        };

        t.register_pipeline::<TextureVertex>();
        t.register_pipeline::<ChunkDrawable>();

        t
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        // Update swap chain
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);

        // Update depth texture
        let (_, depth_view, _) = default_depth_texture(&self.device, &self.sc_desc);
        self.depth_view = depth_view;

        // Update camera
        self.camera.projection.resize(new_size.width, new_size.height);

        println!("New screensize: {}x{}", new_size.width, new_size.height);
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.camera.controller.update_camera(&mut self.camera.view, dt);
        self.camera.update(&self.queue);
    }

    pub fn input(&mut self, event: &DeviceEvent) -> bool {
        self.camera.input(event)
    }

    pub fn start_frame(&self) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default())
    }

    pub fn end_frame(&self, encoder: wgpu::CommandEncoder) {
        self.queue.submit(vec![encoder.finish()]);
    }

    pub fn render(
        &mut self,
        objs: Vec<&impl Drawable>,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::SwapChainFrame,
    ) {         
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass descriptor in renderer"),
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { // Clear color
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    }
                }
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: &self.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        for obj in objs {
            obj.draw(&mut render_pass, &self);
        }          
    }

    pub fn get_pipeline<T: 'static + Drawable>(&self) -> &wgpu::RenderPipeline {
        &self
            .pipelines
            .get(&std::any::TypeId::of::<T>())
            .expect("Pipeline was not registered in context")
    }

    pub fn register_pipeline<T: 'static + Drawable>(&mut self) {
        self.pipelines
            .insert(std::any::TypeId::of::<T>(), T::create_pipeline(self));
    }

    pub fn default_pipeline(
        &self,
        vertex: wgpu::ShaderModuleDescriptor,
        fragment: wgpu::ShaderModuleDescriptor,
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
                module: &self.device.create_shader_module(&vertex),
                entry_point: "main", 
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState { 
                module: &self.device.create_shader_module(&fragment),
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
}