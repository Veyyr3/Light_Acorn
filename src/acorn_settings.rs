// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

/*
Warning: YOU shouldn't delete this file because other Acorn files use AcornZoneContext 

Warning: Do not rename 'before_2d_zone' or 'after_2d_zone' fields, kernel depends on them
*/

// src/acorn_settings.rs
use crate::acorn_kernel::prelude::*; // import Zone
// game suggestions
use crate::acorn_tools::acorn_game_tools::agt_heart::Acorn3DAssetDatabase;

/// Contain here your Zones
pub struct AcornZoneContext {
    pub ui_input_zone: Zone,
    pub before_2d_zone: Zone,
    pub after_2d_zone: Zone,
    // add here your Zone trough comma
}

/// Contain here your global statements 
pub struct AcornGlobalContext {
    // from game tools
    pub assets_3d: Acorn3DAssetDatabase,
}