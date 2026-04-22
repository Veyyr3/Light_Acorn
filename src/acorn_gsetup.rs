// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/acorn_gsetup.rs
use crate::acorn_settings::{
    AcornGlobalContext,
};
// game suggestions
use crate::acorn_tools::acorn_game_tools::prelude::*;

/// Create here your Global States.
pub fn acorn_global_setup() -> AcornGlobalContext {
    // ---------------------------- Game setup ----------------------------
    // Keep 3d models in assets database.
    let mut assets_3d = Acorn3DAssetDatabase {meshes: Vec::new()};

    // Add your .obj files with push.
    // PLEASE, remember index of your 3d models when you add news.
    // It so, because for perfomance. 
    // BUT I leave it to you for organize logic assets keeping.
    assets_3d.meshes.push(
        load_obj_with_materials_to_mesh("src/acorn_tools/acorn_game_tools/objs/acorn_engine.obj")
    );
    
    AcornGlobalContext { 
        // suggestion for game
        assets_3d
    }
}