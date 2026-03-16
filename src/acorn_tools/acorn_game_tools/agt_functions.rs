// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_functions.rs
use macroquad::prelude::*;
use crate::acorn_settings::AcornContext;
use crate::acorn_tools::acorn_game_tools::agt_heart::{Entity3DModel, Entity3DTransform};
use bevy_ecs::world::World;

// ---------------------------- 3D transforming ----------------------------
fn acorn_generate_matrix(entity_3d_set: &Entity3DTransform) -> Mat4 {
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