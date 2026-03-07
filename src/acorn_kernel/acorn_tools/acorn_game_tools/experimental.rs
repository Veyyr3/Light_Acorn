use macroquad::prelude::*;
use crate::acorn_kernel::{
    acorn_settings::{AcornContext, AcornRenderBuffer}, 
    acorn_tools::acorn_game_tools::{agt_functions::acorn_generate_matrix, agt_heart::{Entity3DModel, Entity3DTransform}}
};
use bevy_ecs::world::World;

pub fn acorn_game_draw_3d_assets_3(world: &mut World, context: &mut AcornContext) {
    context.render_buffer.vertices.clear();
    context.render_buffer.indices.clear();

    let mut last_index_offset: usize = 0;
    
    // u16::MAX = 65535. Это жесткий лимит для вершин в одном draw_mesh
    const MAX_VERTICES_PER_BATCH: usize = 65000; 

    let assets = &context.assets_3d;
    let buffer = &mut context.render_buffer;

    let mut query = world.query::<(&Entity3DTransform, &Entity3DModel)>();

    for (transform, model_info) in query.iter(world) {
        let source_mesh = &assets.meshes[model_info.mesh_id];
        
        // ПРОВЕРКА: Если кол-во ВЕРШИН превысит 65535, сбрасываем батч
        if last_index_offset + source_mesh.vertices.len() > MAX_VERTICES_PER_BATCH {
            flush_batch(buffer);
            last_index_offset = 0;
        }

        let model_matrix = acorn_generate_matrix(&transform);

        // Трансформируем вершины
        for v in &source_mesh.vertices {
            let mut new_v = *v;
            // Используем mat4 * vec4 для скорости
            let pos = model_matrix * v.position.extend(1.0);
            new_v.position = pos.xyz();
            buffer.vertices.push(new_v);
        }

        // Добавляем индексы со смещением
        for i in &source_mesh.indices {
            buffer.indices.push((*i as usize + last_index_offset) as u16);
        }

        last_index_offset += source_mesh.vertices.len();
    }

    flush_batch(buffer);
}

fn flush_batch(buffer: &mut AcornRenderBuffer) {
    if !buffer.vertices.is_empty() {
        // Macroquad'овский draw_mesh внутри всё равно копирует данные в свой буфер.
        // Чтобы минимизировать тормоза, мы создаем Mesh только на момент отрисовки.
        draw_mesh(&Mesh {
            vertices: buffer.vertices.clone(),
            indices: buffer.indices.clone(),
            texture: None,
        });
        
        buffer.vertices.clear();
        buffer.indices.clear();
    }
}