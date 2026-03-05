// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_functions.rs
use macroquad::prelude::*;
use tobj;

pub fn load_obj_to_mesh(path: &str) -> Mesh {
    // Загружаем .obj файл
    // load_obj возвращает (модели, материалы)
    let (models, _materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            single_index: true,   // Важно для простоты конвертации в макроквод
            triangulate: true,    // Всегда триангулируем для отрисовки
            ..Default::default()
        },
    ).expect("Failed to load OBJ file");

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Проходим по всем мешам в файле (обычно он один, но tobj поддерживает много)
    for m in models {
        let mesh = &m.mesh;

        // Наполняем индексы (u16 для экономии памяти на слабом ПК)
        for &i in &mesh.indices {
            indices.push(i as u16);
        }

        // Наполняем вершины
        for i in 0..mesh.positions.len() / 3 {
            vertices.push(Vertex {
                position: vec3(
                    mesh.positions[i * 3],
                    mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2],
                ),
                // Если есть UV-координаты (текстуры), берем их
                uv: if !mesh.texcoords.is_empty() {
                    vec2(mesh.texcoords[i * 2], 1.0 - mesh.texcoords[i * 2 + 1])
                } else {
                    vec2(0.0, 0.0)
                },
                color: WHITE.into(),
                normal: vec4(0.0, 0.0, 0.0, 1.0)
            });
        }
    }

    // Собираем итоговый Mesh для macroquad
    Mesh {
        vertices,
        indices,
        texture: None, // Текстуры добавим позже, если решим
    }
}