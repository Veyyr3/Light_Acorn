// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_functions.rs
use macroquad::prelude::*;
use tobj;

/// Load your 3d model and fill with macroquad color.
/// 
/// You can use this ONLY IF you don't have .mtl file.
/// 
/// Use [u8; 4] RGBA in second argument for control your own color. 
pub fn color_and_load_obj_to_mesh(path: &str, color: [u8; 4]) -> Mesh {
    // load_obj returns (models, materials)
    let (models, _materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            single_index: true, // Leave it for simple convertation
            triangulate: true,    // It's important! Macroquad use triangles to draw mesh
            ..Default::default()
        },
    ).expect("Failed to load OBJ file");

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // cycle for all mesh in .obj
    for m in models {
        let mesh = &m.mesh;

        // indeces in macroquad have u16 type
        for &i in &mesh.indices {
            indices.push(i as u16);
        }

        // creating vertices
        for i in 0..mesh.positions.len() / 3 {
            vertices.push(Vertex {
                position: vec3(
                    mesh.positions[i * 3],
                    mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2],
                ),
                // if .obj has UV-coordinates then use them
                uv: if !mesh.texcoords.is_empty() {
                    vec2(mesh.texcoords[i * 2], 1.0 - mesh.texcoords[i * 2 + 1])
                } else {
                    vec2(0.0, 0.0)
                },
                color,
                normal: vec4(0.0, 0.0, 0.0, 1.0)
            });
        }
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

/// Load your 3d model and fill with macroquad color.
/// 
/// You can use this if you have .mtl file.
/// 
/// Use [u8; 4] RGBA in second argument for control your own color. 
pub fn color_mtl_and_load_obj_to_mesh(path: &str, color: [u8; 4]) -> Mesh {
    let (models, _materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    ).expect("Failed to load OBJ file");

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for m in models {
        let mesh = &m.mesh;
        
        // how many vertices was in .obj
        let vertex_offset = vertices.len() as u16;

        // indeces in macroquad have u16 type
        for &i in &mesh.indices {
            indices.push(i as u16 + vertex_offset);
        }

        // creating vertices
        for i in 0..mesh.positions.len() / 3 {
            vertices.push(Vertex {
                position: vec3(
                    mesh.positions[i * 3],
                    mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2],
                ),
                uv: if !mesh.texcoords.is_empty() {
                    vec2(mesh.texcoords[i * 2], 1.0 - mesh.texcoords[i * 2 + 1])
                } else {
                    vec2(0.0, 0.0)
                },
                color: color,
                normal: vec4(0.0, 0.0, 0.0, 1.0)
            });
        }
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

/// Загружает OBJ и MTL, объединяя всё в один Mesh с цветами вершин для 1 draw call.
pub fn load_obj_with_materials_to_mesh(path: &str) -> Mesh {
    // Указываем tobj загружать материалы (.mtl)
    let (models, materials_result) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    ).expect("Failed to load OBJ file");

    // Если материалов нет или ошибка загрузки, используем пустой вектор
    let materials = materials_result.unwrap_or_default();

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for m in models {
        let mesh = &m.mesh;
        let vertex_offset = vertices.len() as u16;

        // 1. Определяем цвет для этой части меша
        let mat_color = if let Some(id) = mesh.material_id {
            // Распаковываем Option<[f32; 3]>, если там None - берем белый цвет
            let d = materials[id].diffuse.unwrap_or([1.0, 1.0, 1.0]);
            Color::new(d[0], d[1], d[2], 1.0)
        } else {
            WHITE // Если у меша вообще нет ID материала
        };

        // 2. Наполняем индексы с учетом смещения
        for &i in &mesh.indices {
            indices.push(i as u16 + vertex_offset);
        }

        // 3. Наполняем вершины
        for i in 0..mesh.positions.len() / 3 {
            // Конвертируем Color в [u8; 4], так как в версии 0.4.14 Vertex ждет массив байтов 
            let color_bytes = [
                (mat_color.r * 255.0) as u8,
                (mat_color.g * 255.0) as u8,
                (mat_color.b * 255.0) as u8,
                (mat_color.a * 255.0) as u8,
            ];

            vertices.push(Vertex {
                position: vec3(
                    mesh.positions[i * 3],
                    mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2],
                ),
                uv: if !mesh.texcoords.is_empty() {
                    vec2(mesh.texcoords[i * 2], 1.0 - mesh.texcoords[i * 2 + 1])
                } else {
                    vec2(0.0, 0.0)
                },
                color: color_bytes, 
                // В твоей версии Macroquad нормали это Vec4 
                normal: if !mesh.normals.is_empty() {
                    vec4(mesh.normals[i * 3], mesh.normals[i * 3 + 1], mesh.normals[i * 3 + 2], 1.0)
                } else {
                    vec4(0.0, 0.0, 0.0, 1.0)
                },
            });
        }
    }

    Mesh {
        vertices,
        indices,
        texture: None, // Для 1 draw call с цветами текстура не нужна
    }
}