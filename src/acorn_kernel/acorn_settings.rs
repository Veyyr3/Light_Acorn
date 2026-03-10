// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/acorn_kernel/acorn_settings.rs
use crate::acorn_kernel::{
    acorn_heart::Zone, 
    // suggestions
    acorn_tools::acorn_game_tools::agt_heart::Acorn3DAssetDatabase
};

/// Contain here your Zones and global statements 
/// 
/// Advise: better keep global statements in other struct.
pub struct AcornContext {
    pub before_2d_zone: Zone,
    pub after_2d_zone: Zone,
    // from game tools
    pub assets_3d: Acorn3DAssetDatabase
}