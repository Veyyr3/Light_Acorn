// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_functions.rs

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use crate::acorn_tools::acorn_game_tools::prelude::*;
// from settings
use crate::acorn_settings::{
    AcornZoneContext,
    AcornGlobalContext,
};
// for debug
use std::fs::File;
use std::io::{BufReader, BufRead};

/*
Here are different functions.
*/

// ---------------------------- Functions 3D transforming ----------------------------

fn acorn_generate_matrix(entity_3d_set: &AcornEntity3DTransform) -> Mat4 {
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

// ---------------------------- Acorn Before 2D Zone Functions ----------------------------

#[allow(dead_code)]
/// ## Description
/// Use this function to draw all your entities with 3D models.
/// 
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub assets_3d: Acorn3DAssetDatabase,`
/// 
/// ## Example: 
/// ```
/// let before_2d_zone = zone! {
///    location! {
///        agt_draw_3d_assets,
///    }
/// };
/// ```
pub fn agt_draw_3d_assets(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    let gl = acorn_get_gl_contex();

    let db_assets = &context.game_base_preset.assets_3d.meshes;

    let mut query = 
        world.query::<(&AcornEntity3DTransform, &AcornEntity3DModel)>();

    for (transform, mesh) in query.iter(world) {
        let model_matrix = acorn_generate_matrix(&transform);

        gl.push_model_matrix(model_matrix);
        
        /*
        You may change to if/else branching for safety
        But I use perfomance mode

        if let Some(mesh) = db_assets.get(mesh.mesh_id) {
            draw_mesh(mesh);
        } else {
            println!("oops...")
        }
        */

        draw_mesh(&db_assets[mesh.mesh_id]);

        gl.pop_model_matrix();
    }
}

#[allow(dead_code)]
/// ## Description
/// Add gravity for your entities.
/// 
/// Cheaper in perfomance than `agt_gravity_no_under_ground`.
/// 
/// ## Required entity components:
/// * `Acorn3DSpeed`
/// * `AcornHasGravity` 
pub fn agt_gravity(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    let mut query = 
        world.query_filtered::<&mut Acorn3DSpeed, With<AcornHasGravity>>();

    let gravity_force = context.game_base_preset.gravity_force;

    let dt = context.frame_delta;

    for mut speed in query.iter_mut(world) {
        speed.speed_value.y -= gravity_force * dt;
    }
}

#[allow(dead_code)]
/// ## Description
/// Add gravity for your entities. Entities do not fall under 0.0 on Y axis.
/// 
/// ## Required entity components:
/// * [`Acorn3DSpeed`]
/// * [`AcornEntity3DTransform`]
/// * [`AcornHasGravity`]
/// 
/// ## WARNING:
/// **Put only AFTER function** `agt_do_entities_move` **or Functions Sets with Collision like** 'AGT_SIMPLE_COLLISION'. 
/// 
/// **Example:**
/// ```
/// let before_2d_zone = zone! {
///     AGT_SLIDE_COLLISION_INCLUDE_TRIGGERS, // A Functions Set with Collisions
///     location! {
///         agt_gravity_no_under_ground, // <-
///     },
/// }
/// ```
pub fn agt_gravity_no_under_ground(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // get context
    let gravity_force = context.game_base_preset.gravity_force;
    let dt = context.frame_delta;

    // create query
    let mut query = 
        world.query_filtered::<(&mut AcornEntity3DTransform, &mut Acorn3DSpeed), With<AcornHasGravity>>();

    // pull out everything that fell
    for (mut e_pos, mut e_speed) in query.iter_mut(world) {
        if e_pos.position.y < 0.0 {
            e_pos.position.y = 0.0;
        } else {
            e_speed.speed_value.y -= gravity_force * dt;
        }
    }
}

// ---------------------------- Debug Functions ----------------------------

#[allow(dead_code)]
/// Function to inspect number of functions in Zones and Locations.
/// 
/// Add to `after_2d_zone`
pub fn acorn_debug_inspector(
    _world: &mut World, 
    zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    let mut y_offset = 20.0;
    let x_start = 20.0;
    let font_size = 20.0;

    draw_text("--- LIGHT ACORN RUNTIME INSPECTOR ---", x_start, y_offset, font_size, YELLOW);
    y_offset += 30.0;

    // add here your zone
    let all_zones = [
        ("UI_INPUT_ZONE", &zones.ui_input_zone),
        ("BEFORE_2D_ZONE", &zones.before_2d_zone),
        ("AFTER_2D_ZONE", &zones.after_2d_zone),
    ];

    for (z_name, zone) in all_zones.iter() {
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
    let total_funcs = zones.before_2d_zone.locations.iter().map(|l| l.functions.len()).sum::<usize>() 
                    + zones.after_2d_zone.locations.iter().map(|l| l.functions.len()).sum::<usize>();
    
    draw_text(&format!("TOTAL ACTIVE FUNCTIONS: {}", total_funcs), x_start, y_offset + 10.0, font_size, SKYBLUE);
}

#[allow(dead_code)]
/// Displays your program's memory consumption in KB (Linux) to the console.
pub fn acorn_linux_memory_inspector(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
	if let Ok(file) = File::open("/proc/self/statm") {
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        if reader.read_line(&mut line).is_ok() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                if let Ok(pages) = parts[1].parse::<usize>() {
                    let memory_kb = (pages * 4096) / 1024;
                    println!("Memory consumption (Linux): {} kb", memory_kb);
                }
            }
        }
    }
}