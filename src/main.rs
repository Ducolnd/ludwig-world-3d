#![allow(unused_imports)]

use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder},
};
use rand::Rng;
use futures::executor::block_on;
use std::time::Instant;

mod render;
mod world;

use render::low::master::Master;
use render::vertexarray::VertexArray;
use render::shapes::shapes::Quad;
use render::meshing::chunkmeshing::{ChunkMesh};
use render::meshing::meshing::{Mesh, MeshFace, *};

use world::block::blocks;
use world::block::blocks::get_block;
use crate::world::chunk::pos::*;
use world::world::World;

fn main() {    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Ludwig Engine")
        .with_inner_size(winit::dpi::PhysicalSize::new(1200, 800))
        .build(&event_loop)
        .unwrap();
    
    let mut master = block_on(Master::new(&window));

    let mut world = World::new();

    let pos = ChunkPos {x: 0, y: 0, z: 0};
    world.load_chunk(pos, &mut master);
    world.update_chunk_buffer(&mut master, pos);

    let pos = ChunkPos {x: 0, y: 0, z: 1};
    world.load_chunk(pos, &mut master);
    world.update_chunk_buffer(&mut master, pos);
    
    let pos = ChunkPos {x: 1, y: 0, z: 1};
    world.load_chunk(pos, &mut master);
    world.update_chunk_buffer(&mut master, pos);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {

                // Handle input
                master.input(event);
                // Update
                master.update();

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
                        master.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &mut so w have to dereference it twice
                        master.resize(**new_inner_size);
                    }
                    _ => {}
                }
                
            }
            Event::RedrawRequested(_) => {
                match master.render() { // Render
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => master.resize(master.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
    
}