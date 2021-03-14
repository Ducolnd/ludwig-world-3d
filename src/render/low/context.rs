use winit::{
    event::*,
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    window::Window,
};
use futures::executor::block_on;

use crate::render::low::renderer::Renderer;
use crate::render::drawables::{texture_vertex::TextureVertex, chunk::ChunkDrawable};
use crate::render::shapes::shapes::{Quad, quad_builder};
use crate::render::vertexarray::VertexArray;
use crate::world::chunk::{chunk::Chunk, chunkmanager::ChunkManager, pos::ChunkPos};

pub struct Context {
    pub window: Window,
    pub event_loop: Option<EventLoop<()>>,
    pub renderer: Renderer,

    d: TextureVertex,
    dd: ChunkDrawable,
    manager: ChunkManager,
}

impl Context {
    pub fn new(window_title: String, window_size: [u32; 2]) -> Self {
        // Winit
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(window_title)
            .with_inner_size(PhysicalSize::new(window_size[0], window_size[1]))
            .build(&event_loop)
            .unwrap();

        let renderer = block_on(Renderer::new(&window));

        let d = TextureVertex::new(&renderer.device);
        let dd = ChunkDrawable::new(&renderer.device, ChunkPos::new(0, 0, 0));

        let mut manager = ChunkManager::new(1);
        manager.load_chunk(ChunkPos::new(0, 0, 0), [10; 16*16]);


        Self {
            event_loop: Some(event_loop),
            window,
            renderer,

            d,
            dd,
            manager,
        }
    }

    pub fn run(mut self) {

        
        let mut last_render_time = std::time::Instant::now();
        
        let mut updated = false;
        let mut frame: Option<wgpu::SwapChainFrame> = None;

        self.event_loop.take().unwrap().run(move |event, _, control_flow| {        
            match event {
                Event::DeviceEvent {
                    ref event,
                    .. // We're not using device_id currently
                } => {
                    self.renderer.input(event);
                }

                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        },
                        WindowEvent::Resized(physical_size) => {
                            self.renderer.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so w have to dereference it twice
                            self.renderer.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                    
                }
                Event::RedrawRequested(_) => {
                    let now = std::time::Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;
                    
                    self.renderer.update(dt);
                                  
                    
                    match frame.take() {
                        None => {
                            match self.renderer.swap_chain.get_current_frame() { 
                                Ok(swapchainframe) => {
                                    frame = Some(swapchainframe);
                                }
                                // Recreate the swap_chain if lost
                                Err(wgpu::SwapChainError::Lost) => self.renderer.resize(self.renderer.size),
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                                // All other errors (Outdated, Timeout) should be resolved by the next frame
                                Err(e) => println!("{:?}", e),
                            }
                        }
                        Some(swapchainframe) => {
                            let mut encoder = self.renderer.start_frame();
                            
                            if !updated {
                                updated = true;

                                let mut data = VertexArray::<Quad>::new();
                                data.push(quad_builder());
                                self.d.from_vertex_array(&data, &self.renderer.device, &mut encoder);

                                self.dd.from_chunk_mesh(self.manager.get_mesh(ChunkPos::new(0,0,0)), &self.renderer.device, &mut encoder);
                                
                                println!("updated sh it:");
                            }

                            self.renderer.render(
                                vec![&self.dd],
                                &mut encoder,
                                &swapchainframe,
                            );

                            self.renderer.end_frame(encoder);
                        }
                    }
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => {}
            }
        });
    }
}