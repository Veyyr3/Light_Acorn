// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_functions.rs
use macroquad::prelude::*;
use crate::acorn_kernel::{
    acorn_settings::AcornContext, 
    acorn_tools::acorn_game_tools::agt_heart::{Entity3DModel, Entity3DTransform}
};
use bevy_ecs::world::World;

fn acorn_generate_matrix(entity_3d_set: &Entity3DTransform) -> Mat4 {
    Mat4::from_translation(entity_3d_set.position)
    *Mat4::from_axis_angle(vec3(0.0, 1.0, 0.0), entity_3d_set.rotation)
    *Mat4::from_scale(entity_3d_set.scale)
}

fn acorn_get_gl_contex() -> &'static mut QuadGl {
    unsafe {
        let internal_gl = get_internal_gl();
        // Мы возвращаем ссылку на quad_gl. 
        // Это безопасно внутри одного кадра.
        std::mem::transmute(internal_gl.quad_gl)
    }
}

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

        let model3d = &context.assets_3d.meshes[mesh.mesh_id];

        draw_mesh(model3d);

        gl.pop_model_matrix();
    }
}