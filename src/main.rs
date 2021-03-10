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
mod helper;
mod game;

use render::low::master::Master;
use render::vertexarray::VertexArray;
use render::shapes::shapes::Quad;
use render::meshing::chunkmeshing::{ChunkMesh};
use render::meshing::meshing::{Mesh, MeshFace, *};

use world::block::blocks;
use world::block::blocks::get_block;
use world::chunk::pos::*;
use world::world::World;

use game::player::player::Player;

fn main() {    
    // Winit
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Ludwig Engine")
        .with_inner_size(winit::dpi::PhysicalSize::new(1200, 800))
        .build(&event_loop)
        .unwrap();
    
    // Master
    let mut master = block_on(Master::new(&window));

    // Game setup
    let player = Player::new((10.0, 5.0, 4.0).into());
    let mut world = World::new(1);

    world.place_player(player);

    println!("Average meshing time: {} Average loading time: {}", world.chunk_manager.meshing_time(), world.chunk_manager.loading_time());


    // Main loop
    let mut last_render_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {        
        match event {
            Event::DeviceEvent {
                ref event,
                .. // We're not using device_id currently
            } => {
                master.input(event);
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                
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
                let now = std::time::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                master.update(dt);
                master.update_player(&mut world);

                // println!("FPS: {}", 1.0 / dt.as_secs_f64());

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