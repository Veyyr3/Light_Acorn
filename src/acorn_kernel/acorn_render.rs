// src/acorn_kernel/acorn_render/mod.rs
use macroquad::prelude::*;
// use bevy_ecs::prelude::World;

pub async fn acorn_loop() { // mut world: World
    loop {
        clear_background(BLACK);

        // 1. Получаем доступ к низкоуровневому контексту для матриц
        // let gl = unsafe { get_internal_gl().quad_gl };

        // 2. Здесь будет вызов систем рендеринга
        // render_system(&mut world, gl);

        next_frame().await;
    }
}