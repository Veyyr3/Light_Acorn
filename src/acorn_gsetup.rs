// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/acorn_gsetup.rs

use crate::acorn_settings::{
    AcornGlobalContext,
};
use macroquad::prelude::*;
// game suggestions
use crate::acorn_tools::acorn_game_tools::prelude::*; // Acorn3DGameBase, Acorn3DAssetDatabase

/// Create here your Global States.
pub fn acorn_global_setup() -> AcornGlobalContext {
    // ---------------------------- Game setup ----------------------------

    // =================================
    // setup base 
    let mut game_base_preset = Acorn3DGameBase {
        // grid collision
        world_collision_grid: AcornXZWorldGrid { 
            cell_size: 5.0,
            ..Default::default()
        },
        // camera
        camera: Acorn3DCamera{
            position: vec3(10.0, 1.8, 0.0),
            ..Default::default()
        },
        // player
        player: AcornPlayer3D { 
            position: vec3(10.0, 10.0, 10.0), 
            eye_position: vec3(0.0, 5.0, 0.0),
            ..Default::default()
        },
        // gravity
        gravity_force: 1.0,
        ..Default::default()
    };
    // =================================



    // =================================
    // it's important thing. The speed of the camera and objects will not depend on FPS.
    let frame_delta = get_frame_time();
    // =================================



    // =================================
    // Keep 3d models in assets database.
    let assets_3d = &mut game_base_preset.assets_3d;

    // Add your .obj files with push.
    // PLEASE, remember index of your 3d models when you add news.
    // It so, because for perfomance. 
    // BUT I leave it to you for organize logic assets keeping.
    assets_3d.meshes.push(
        load_obj_with_materials_to_mesh("objs/acorn_engine.obj")
    );
    // =================================
    


    // return global context
    AcornGlobalContext { 
        // suggestion for game
        game_base_preset,
        frame_delta,
    }
}