// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_functions.rs
use macroquad::prelude::*;
use crate::acorn_kernel::acorn_tools::acorn_game_tools::agt_heart::Entity3DTransform;

pub fn acorn_generate_matrix(entity_3d_set: &Entity3DTransform) -> Mat4 {
    Mat4::from_translation(entity_3d_set.position)
    *Mat4::from_axis_angle(vec3(0.0, 1.0, 0.0), entity_3d_set.rotation)
    *Mat4::from_scale(entity_3d_set.scale)
}

pub fn get_gl_contex() -> &'static mut QuadGl {
    unsafe {
        let internal_gl = get_internal_gl();
        // Мы возвращаем ссылку на quad_gl. 
        // Это безопасно внутри одного кадра.
        std::mem::transmute(internal_gl.quad_gl)
    }
}