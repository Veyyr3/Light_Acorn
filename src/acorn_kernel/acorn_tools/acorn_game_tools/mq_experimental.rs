// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

use macroquad::miniquad as mq;
use macroquad::prelude::*;
use crate::acorn_kernel::acorn_tools::acorn_game_tools::shaders::*;
use crate::acorn_kernel::{
    acorn_settings::{AcornContext}
};
use crate::get_look_dir;
use bevy_ecs::world::World;

// pub fn acorn_init_instancing(ctx: &mut dyn RenderingBackend) {
//     // 1. Создаем буфер для матриц. Используем Stream, так как данные меняются каждый кадр.
//     // Резервируем место под 10 000 матриц.
//     let instance_buffer = ctx.new_buffer(
//         BufferType::VertexBuffer,
//         BufferUsage::Stream,
//         BufferSource::empty::<Mat4>(10_000), // [cite: 352]
//     );

//     // 2. Описываем слои (Layouts)
//     let buffer_layouts = [
//         BufferLayout::default(), // Buffer 0: Вершины (PerVertex) 
//         BufferLayout {           // Buffer 1: Матрицы (PerInstance)
//             step_func: VertexStep::PerInstance,
//             ..BufferLayout::default()
//         },
//     ];

//     // 3. Описываем атрибуты (Attributes)
//     let attributes = [
//         // Из твоего Vertex (Buffer 0)
//         VertexAttribute::with_buffer("aPos", VertexFormat::Float3, 0),
//         VertexAttribute::with_buffer("aTex", VertexFormat::Float2, 0),
//         VertexAttribute::with_buffer("aCol", VertexFormat::Byte4, 0),
//         VertexAttribute::with_buffer("aNormal", VertexFormat::Float4, 0),
        
//         // Твоя Матрица (Buffer 1) [cite: 226]
//         VertexAttribute::with_buffer("aModelMat", VertexFormat::Mat4, 1),
//     ];

//     // 4. Создаем Pipeline (Шейдер загружается отдельно)
//     // let pipeline = ctx.new_pipeline(&buffer_layouts, &attributes, shader_id, params);
// }


// В твоем файле, где происходит настройка движка
pub fn acorn_setup_instancing(context: &mut AcornContext) {
    // 1. Получаем контекст через твой "козырь"
    let internal_gl = unsafe { get_internal_gl() };
    let ctx = internal_gl.quad_context;

    // 2. Тот самый блок создания шейдера
    let shader = ctx.new_shader(
        mq::ShaderSource::Glsl {
            vertex: VERTEX_SHADER_SRC, // твои строки с кодом #330
            fragment: FRAGMENT_SHADER_SRC,
        },
        mq::ShaderMeta {
            uniforms: mq::UniformBlockLayout {
                uniforms: vec![
                    mq::UniformDesc::new("uViewProjection", mq::UniformType::Mat4),
                ],
            },
            images: vec!["uTexture".to_string()],
        },
    ).expect("Шейдер не собрался!");

    // 3. Создаем Pipeline
    // Важно: здесь мы описываем, как Rust-структуры стыкуются с GLSL-слотами
    // Настройки поведения видеокарты (глубина, прозрачность)
    let params = mq::PipelineParams {
        depth_test: mq::Comparison::LessOrEqual,
        depth_write: true,
        primitive_type: mq::PrimitiveType::Triangles,
        ..Default::default()
    };

    let pipeline = ctx.new_pipeline(
        &[
            mq::BufferLayout::default(), 
            mq::BufferLayout {
                step_func: mq::VertexStep::PerInstance,
                ..Default::default()
            },
        ],
        &[
            mq::VertexAttribute::with_buffer("aPos", mq::VertexFormat::Float3, 0),
            mq::VertexAttribute::with_buffer("aTex", mq::VertexFormat::Float2, 0),
            mq::VertexAttribute::with_buffer("aCol", mq::VertexFormat::Byte4, 0),
            mq::VertexAttribute::with_buffer("aNormal", mq::VertexFormat::Float4, 0),
            mq::VertexAttribute::with_buffer("aModelMat", mq::VertexFormat::Mat4, 1),
        ],
        shader, // Тот ID, что мы получили из new_shader
        params, // Добавили 4-й аргумент!
    );

    // 4. Создаем пустой буфер для матриц (на 10 000 штук)
    let instance_buffer = ctx.new_buffer(
        mq::BufferType::VertexBuffer,
        mq::BufferUsage::Stream,
        mq::BufferSource::empty::<Mat4>(10_000),
    );

    // 5. Сохраняем "ключи" в наш контекст
    context.instancing_pipeline = pipeline;
    context.gpu_instance_buffer = instance_buffer;
}


// ВАЖНО: Тебе нужно добавить эти ID в структуру ассета, 
// чтобы не создавать их каждый кадр!
// pub struct AcornMeshGPU {
//     pub v_buffer: mq::BufferId,
//     pub i_buffer: mq::BufferId,
//     pub user_mesh: Mesh, 
// }
pub struct AcornPreparedMesh {
    pub v_buffer: mq::BufferId,
    pub i_buffer: mq::BufferId,
    pub index_count: i32,
    pub texture: mq::TextureId,
}
pub struct AcornMeshInstanceDB {
    // Теперь это просто список "ключей" к видеокарте. 
    // Очень легкая структура, можно хранить тысячи таких.
    pub gpu_meshes: Vec<AcornPreparedMesh>,
}

pub fn acorn_draw_instanced(_world: &mut World, context: &mut AcornContext) {
    let mut internal_gl = unsafe { get_internal_gl() };

    // clear to fix z-buffer
    internal_gl.quad_context.clear(None, Some(1.0), None);
    // ПОЛУЧАЕМ ГОТОВУЮ МАТРИЦУ ИЗ MACROQUAD
    let target = context.pos + get_look_dir(context.yaw, context.pitch);
    let aspect = screen_width() / screen_height();
    
    // ВАЖНО: fovy должен совпадать с тем, что в Camera3D (по умолчанию 45 градусов)
    let fovy = 45.0_f32.to_radians(); 
    
    // Математика из источника camera.txt 
    let view_proj = Mat4::perspective_rh_gl(fovy, aspect, 0.01, 10000.0)
                  * Mat4::look_at_rh(context.pos, target, Vec3::Y);


    let ctx = internal_gl.quad_context;
    
    // 1. Считаем View (Камера)
    // let yaw = context.yaw;
    // let pitch = context.pitch;
    // let x = yaw.cos() * pitch.cos();
    // let y = pitch.sin();
    // let z = yaw.sin() * pitch.cos();
    // let forward = vec3(x, y, z).normalize();
    // let view = Mat4::look_at_rh(context.pos, context.pos + forward, Vec3::Y);

    // // 2. Считаем Projection (Линза)
    // let projection = Mat4::perspective_rh_gl(
    //     60.0f32.to_radians(), 
    //     screen_width() / screen_height(), 
    //     0.01, 
    //     1000.0
    // );

    // Итоговая матрица для шейдера
    // let view_proj = projection * view;

    // 3. Цикл отрисовки
    for (m_id, gpu_mesh) in context.instance_assets.gpu_meshes.iter().enumerate() {
        let current_matrices: Vec<Mat4> = context.matrix_buffer.iter()
            .filter(|(id, _)| *id == m_id)
            .map(|(_, mat)| *mat)
            .collect();

        if current_matrices.is_empty() { continue; }

        ctx.buffer_update(context.gpu_instance_buffer, mq::BufferSource::slice(&current_matrices));

        let bindings = mq::Bindings {
            vertex_buffers: vec![gpu_mesh.v_buffer, context.gpu_instance_buffer], 
            index_buffer: gpu_mesh.i_buffer,
            images: vec![gpu_mesh.texture], 
        };

        ctx.apply_pipeline(&context.instancing_pipeline);
        ctx.apply_bindings(&bindings);
        
        // ПЕРЕДАЕМ МАТРИЦУ В ШЕЙДЕР
        ctx.apply_uniforms(mq::UniformsSource::table(&view_proj.to_cols_array()));

        ctx.draw(0, gpu_mesh.index_count, current_matrices.len() as i32);
    }
}

// функция, которая берет обычный Mesh и создает его GPU-копию.
pub fn acorn_register_for_instancing(
    ctx: &mut dyn mq::RenderingBackend, 
    mesh: &Mesh,
    default_tex: mq::TextureId // Добавляем новый аргумент
) -> AcornPreparedMesh {
    let v_buffer = ctx.new_buffer(
        mq::BufferType::VertexBuffer,
        mq::BufferUsage::Immutable,
        mq::BufferSource::slice(&mesh.vertices),
    );

    let i_buffer = ctx.new_buffer(
        mq::BufferType::IndexBuffer,
        mq::BufferUsage::Immutable,
        mq::BufferSource::slice(&mesh.indices),
    );

    // Вместо .expect() используем .unwrap_or()
    let texture = mesh.texture.as_ref()
        .map(|t| t.raw_miniquad_id())
        .unwrap_or(default_tex); // Если текстуры нет, берем белую

    AcornPreparedMesh {
        v_buffer,
        i_buffer,
        index_count: mesh.indices.len() as i32,
        texture,
    }
}