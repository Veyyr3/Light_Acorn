// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_functions.rs
use macroquad::prelude::*;
use crate::acorn_kernel::{
    acorn_settings::{AcornContext, AcornRenderBuffer}, 
    acorn_tools::acorn_game_tools::agt_heart::{Entity3DModel, Entity3DTransform}
};
use bevy_ecs::world::World;

// ---------------------------- 3D transforming ----------------------------
pub fn acorn_generate_matrix(entity_3d_set: &Entity3DTransform) -> Mat4 {
    Mat4::from_translation(entity_3d_set.position)
    *Mat4::from_axis_angle(vec3(0.0, 1.0, 0.0), entity_3d_set.rotation)
    *Mat4::from_scale(entity_3d_set.scale)
}

fn acorn_get_gl_contex() -> &'static mut QuadGl {
    unsafe {
        let internal_gl = get_internal_gl();
        // returns link on quad_gl. 
        // it is safe in one frame.
        std::mem::transmute(internal_gl.quad_gl)
    }
}

/// Use this function to draw all your entities with 3D models.
pub fn acorn_game_draw_3d_assets(world: &mut World, context: &mut AcornContext) {
    let gl = acorn_get_gl_contex();

    let mut query = 
        world.query::<(&Entity3DTransform, &Entity3DModel)>();

    for (transform, mesh) in query.iter(world) {
        let model_matrix = acorn_generate_matrix(&transform);

        gl.push_model_matrix(model_matrix);
        
        /*
        You may change to if/else branching for safety
        But I use perfomance mode

        if let Some(mesh) = context.assets_3d.meshes.get(mesh.mesh_id) {
            draw_mesh(mesh);
        } else {
            println!("oops...")
        }
        */

        draw_mesh(&context.assets_3d.meshes[mesh.mesh_id]);

        gl.pop_model_matrix();
    }
}

pub fn acorn_game_draw_3d_assets_batched(world: &mut World, context: &mut AcornContext) {
    // 1. Создаем временные буферы для сбора всей геометрии кадра
    // В идеале их стоит хранить в AcornContext, чтобы не пересоздавать Vec каждый кадр (zero allocation)
    let mut batched_vertices: Vec<Vertex> = Vec::with_capacity(10000 * 24); // Примерно
    let mut batched_indices: Vec<u16> = Vec::with_capacity(10000 * 36);
    let mut last_index_offset = 0;

    let mut query = world.query::<(&Entity3DTransform, &Entity3DModel)>();

    for (transform, model_info) in query.iter(world) {
        let model_matrix = acorn_generate_matrix(&transform);
        let source_mesh = &context.assets_3d.meshes[model_info.mesh_id];

        // 2. "Впекаем" трансформацию в вершины прямо на CPU
        for v in &source_mesh.vertices {
            let mut new_v = *v;
            // Умножаем позицию вершины на матрицу сущности
            let pos_vec4 = model_matrix * vec4(v.position.x, v.position.y, v.position.z, 1.0);
            new_v.position = vec3(pos_vec4.x, pos_vec4.y, pos_vec4.z);
            
            // Если есть нормали, их тоже нужно трансформировать (вращать)
            // new_v.normal = ...
            
            batched_vertices.push(new_v);
        }

        // 3. Копируем индексы с учетом смещения
        for i in &source_mesh.indices {
            batched_indices.push(*i + last_index_offset as u16);
        }

        last_index_offset += source_mesh.vertices.len();
    }

    // 4. ОДИН ВЫЗОВ НА ВСЕ 10 000 ЖЕЛУДЕЙ
    if !batched_vertices.is_empty() {
        let big_mesh = Mesh {
            vertices: batched_vertices,
            indices: batched_indices,
            texture: None, // Берем текстуру первого желудя
        };
        draw_mesh(&big_mesh);
    }
}

pub fn acorn_game_draw_3d_assets2(world: &mut World, context: &mut AcornContext) {
    // 1. Сначала очистим буфер
    context.render_buffer.vertices.clear();
    context.render_buffer.indices.clear();

    let mut last_index_offset = 0;
    const MAX_INDICES: usize = 12000; 

    // 2. Разделяем контекст на части, чтобы Borrow Checker был спокоен
    // Достаем ссылки на поля отдельно
    let assets = &context.assets_3d;
    let buffer = &mut context.render_buffer;

    let mut query = world.query::<(&Entity3DTransform, &Entity3DModel)>();

    for (transform, model_info) in query.iter(world) {
        let model_matrix = acorn_generate_matrix(&transform);
        
        // Берем меш из ассетов
        let source_mesh = &assets.meshes[model_info.mesh_id];

        // Проверка лимита: если не влезает, рисуем накопленное
        if buffer.indices.len() + source_mesh.indices.len() > MAX_INDICES {
            // Передаем ТОЛЬКО буфер, а не весь контекст!
            flush_batch(buffer); 
            last_index_offset = 0;
        }

        // Трансформация на CPU
        for v in &source_mesh.vertices {
            let mut new_v = *v;
            new_v.position = (model_matrix * v.position.extend(1.0)).xyz();
            buffer.vertices.push(new_v);
        }

        for i in &source_mesh.indices {
            buffer.indices.push(*i + last_index_offset as u16);
        }

        last_index_offset += source_mesh.vertices.len();
    }

    // 3. Рисуем финальный остаток
    flush_batch(buffer);
}

fn flush_batch(buffer: &mut AcornRenderBuffer) {
    if !buffer.vertices.is_empty() {
        let batch_mesh = Mesh {
            vertices: buffer.vertices.clone(), 
            indices: buffer.indices.clone(),
            texture: None, 
        };
        draw_mesh(&batch_mesh);
        
        buffer.vertices.clear();
        buffer.indices.clear();
    }
}
// ---------------------------- Debug Functions ----------------------------
/// Functions for inspect number of functions in Zones and Locations.
pub fn acorn_debug_inspector(_world: &mut World, context: &mut AcornContext) {
    let mut y_offset = 20.0;
    let x_start = 20.0;
    let font_size = 20.0;

    draw_text("--- LIGHT ACORN RUNTIME INSPECTOR ---", x_start, y_offset, font_size, YELLOW);
    y_offset += 30.0;

    let zones = [
        ("BEFORE_2D_ZONE", &context.before_2d_zone),
        ("AFTER_2D_ZONE", &context.after_2d_zone),
    ];

    for (z_name, zone) in zones.iter() {
        draw_text(&format!("ZONE: {}", z_name), x_start, y_offset, font_size, ORANGE);
        y_offset += 25.0;

        for (l_idx, location) in zone.locations.iter().enumerate() {
            let func_count = location.functions.len();
            
            let color = if func_count > 0 { GREEN } else { GRAY };

            draw_text(
                &format!("  |_ Location [{}]: {} functions", l_idx, func_count), 
                x_start + 20.0, 
                y_offset, 
                font_size - 2.0, 
                color
            );
            y_offset += 22.0;
        }
        y_offset += 10.0;
    }
    let total_funcs = context.before_2d_zone.locations.iter().map(|l| l.functions.len()).sum::<usize>() 
                    + context.after_2d_zone.locations.iter().map(|l| l.functions.len()).sum::<usize>();
    
    draw_text(&format!("TOTAL ACTIVE FUNCTIONS: {}", total_funcs), x_start, y_offset + 10.0, font_size, SKYBLUE);
}