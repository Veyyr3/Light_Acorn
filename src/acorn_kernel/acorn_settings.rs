// src/acorn_kernel/acorn_heart.rs
use crate::acorn_kernel::{
    acorn_heart::Zone, 
    // suggestions
    acorn_tools::acorn_game_tools::agt_heart::Acorn3DAssetDatabase
};

/// Contain here your Zones and global statements
pub struct AcornContext {
    pub before_2d_zone: Zone,
    pub after_2d_zone: Zone,
    // from game tools
    pub assets_3d: Acorn3DAssetDatabase
}