// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

use macroquad::prelude::*;

// src/acorn_kernel/acorn_heart.rs
use crate::acorn_kernel::{
    acorn_heart::Zone, 
    // suggestions
    acorn_tools::acorn_game_tools::{
        agt_heart::Acorn3DAssetDatabase, 
        mq_experimental::AcornMeshInstanceDB
    }
};

// pub struct AcornRenderBuffer {
//     pub vertices: Vec<Vertex>,
//     pub indices: Vec<u16>,
// }

// impl AcornRenderBuffer {
//     pub fn new() -> Self {
//         Self {
//             // Резервируем память заранее, чтобы i3 не тупил на реаллокациях
//             vertices: Vec::with_capacity(10000), 
//             indices: Vec::with_capacity(15000),
//         }
//     }
// }

/// Contain here your Zones and global statements 
/// 
/// Advise: better keep global statements in other struct.
pub struct AcornContext {
    pub before_2d_zone: Zone,
    pub after_2d_zone: Zone,
    // from game tools
    pub assets_3d: Acorn3DAssetDatabase,
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub acorns_x: u64,
    pub acorns_y: u64,
    pub acorns_count: u64,
    // for acorn_generate_transform, acorn_game_draw_3d_instanced
    pub matrix_buffer: Vec<(usize, Mat4)>, // usize is index of 3D asset.
    // pub render_buffer: AcornRenderBuffer,
    pub instancing_pipeline: miniquad::Pipeline, 
    pub gpu_instance_buffer: miniquad::BufferId,
    pub instance_assets: AcornMeshInstanceDB,
}