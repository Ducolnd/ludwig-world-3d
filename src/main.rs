mod render;
mod world;
mod helper;
mod game;

use crate::render::low::context::Context;

fn main() {    
    let context = Context::new(String::from("Ludwig World 3D"), [1200, 800]);

    context.run();
}