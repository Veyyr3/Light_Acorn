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
    acorn_settings::{AcornContext}, 
    acorn_tools::acorn_game_tools::agt_heart::{Entity3DModel, Entity3DTransform}
};
use bevy_ecs::world::World;

// ---------------------------- 3D transforming ----------------------------
pub fn acorn_generate_matrix(entity_3d_set: &Entity3DTransform) -> Mat4 {
    Mat4::from_translation(entity_3d_set.position)
    *Mat4::from_axis_angle(vec3(0.0, 1.0, 0.0), entity_3d_set.rotation)
    *Mat4::from_scale(entity_3d_set.scale)
}

/// new way to generate matrix.
/// it can add some perfomance.
/// but your models won't rotate by X and Z.
pub fn acorn_generate_matrix_y(entity_3d_set: &Entity3DTransform) -> Mat4 {
    Mat4::from_scale_rotation_translation(
        entity_3d_set.scale,
        Quat::from_rotation_y(entity_3d_set.rotation), 
        entity_3d_set.position,
    )
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

// -------------------------------------- NEW --------------------------------------
/// this function needs context.matrix_buffer
pub fn acorn_generate_transform(world: &mut World, context: &mut AcornContext) {
    context.matrix_buffer.clear();

    let mut query = world.query::<(&Entity3DTransform, &Entity3DModel)>();

    for (transform, model) in query.iter(world) {
        // Считаем матрицу (используя твой метод или оптимизированный)
        let model_matrix = acorn_generate_matrix_y(transform);
        
        // Записываем в конец вектора
        context.matrix_buffer.push((model.mesh_id, model_matrix));
    }
}

pub fn acorn_game_draw_3d(_world: &mut World, context: &mut AcornContext) {
    let gl = acorn_get_gl_contex();

    // Проходим по вектору как по конвейеру
    for (mesh_id, model_matrix) in &context.matrix_buffer {
        // Устанавливаем матрицу трансформации
        gl.push_model_matrix(*model_matrix);
        
        // Рисуем меш, беря его по индексу (usize)
        draw_mesh(&context.assets_3d.meshes[*mesh_id]);

        // Убираем матрицу
        gl.pop_model_matrix();
    }
}

/// This experimental function good for WebGL 1.0/OpenGL ES 2.0
/// You can use it to increase draw calls (more draw calls = more PC suffer)
/// 
/// I leave it because someone can't use higher version OpenGL.
/// But serious GPU instansing is impossible because vertex buffer in macroquad so few (96 kb).
/// 
/// Maybe this function need to refactor because ACORN_GL_100_BATCH_LIMIT depends on vertex number.
const ACORN_GL_100_BATCH_LIMIT: usize = 2379;
pub fn acorn_game_draw_3d_instanced(_world: &mut World, context: &mut AcornContext) {
    // 1 acorn = 62 vertex.
    // 2379 / 62 = 38 acorns on 1 draw call.
    // Vertex = 40 bytes.
    // 2379 * 40 = 95 160 bytes = 93 kb.
    // it's near with macroquad buffer 96 kb.
    
    let mut temp_mesh = Mesh {
        vertices: Vec::with_capacity(65535),
        indices: Vec::with_capacity(65535),
        texture: None,
    };

    for (mesh_id, model_matrix) in &context.matrix_buffer {
        let source_mesh = &context.assets_3d.meshes[*mesh_id];
        
        if temp_mesh.vertices.len() + source_mesh.vertices.len() > ACORN_GL_100_BATCH_LIMIT {
            draw_mesh(&temp_mesh);
            temp_mesh.vertices.clear();
            temp_mesh.indices.clear();
        }

        let offset = temp_mesh.vertices.len() as u16;

        // Копируем и трансформируем вершины
        for v in &source_mesh.vertices {
            let mut new_vertex = *v;
            let pos4 = model_matrix.mul_vec4(vec4(v.position.x, v.position.y, v.position.z, 1.0));
            new_vertex.position = vec3(pos4.x, pos4.y, pos4.z);
            temp_mesh.vertices.push(new_vertex);
        }

        // Копируем индексы со смещением
        for i in &source_mesh.indices {
            temp_mesh.indices.push(i + offset);
        }
    }

    if !temp_mesh.vertices.is_empty() {
        draw_mesh(&temp_mesh);
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