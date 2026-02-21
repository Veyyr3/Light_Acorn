// src/main.rs
mod acorn_kernel;

use acorn_kernel::acorn_render;

#[macroquad::main("Light Acorn test")]
async fn main() {
    println!("Hello, Light Acorn!");

    acorn_render::acorn_loop().await;
}
