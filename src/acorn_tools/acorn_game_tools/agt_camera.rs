// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_camera.rs

use macroquad::prelude::*;
use crate::acorn_settings::{
    AcornZoneContext,
    AcornGlobalContext,
};
use bevy_ecs::prelude::*;

// ---------------------------- Structs ----------------------------

pub struct Acorn3DCamera {
    // look settings
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

// ---------------------------- Implementations ----------------------------

impl Acorn3DCamera {
    /// Create camera
    pub fn create(position: Vec3) -> Self {
        Self { 
            position, 
            yaw: 0.0, 
            pitch: 0.0 
        }
    }
}

// ---------------------------- Functions ----------------------------

fn acorn_game_camera_get_look_dir(yaw: f32, pitch: f32) -> Vec3 {
    vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos()
    ).normalize()
}

// ---------------------------- Public Functions ----------------------------

/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase,`
/// 
/// Example: 
/// ```
///
/// ```
pub fn acorn_game_camera_3d_free_fly(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // 1. rotate
    let mouse_delta = mouse_delta_position();
    context.game_base_preset.camera.yaw -= mouse_delta.x * context.game_base_preset.camera_3d_move_speed; 
    context.game_base_preset.camera.pitch += mouse_delta.y * context.game_base_preset.camera_3d_move_speed;
    context.game_base_preset.camera.pitch = context.game_base_preset.camera.pitch.clamp(-1.5, 1.5);

    let look_dir = acorn_game_camera_get_look_dir(context.game_base_preset.camera.yaw, context.game_base_preset.camera.pitch);

    // 2. move
    // Here: position += look_dir * camera_3d_move_speed * frame_delta
    if is_key_down(KeyCode::W) { context.game_base_preset.camera.position += look_dir * context.game_base_preset.camera_3d_move_speed * context.frame_delta; }
    if is_key_down(KeyCode::S) { context.game_base_preset.camera.position -= look_dir * context.game_base_preset.camera_3d_move_speed * context.frame_delta; }

    let right = look_dir.cross(vec3(0.0, 1.0, 0.0)).normalize();
    if is_key_down(KeyCode::D) { context.game_base_preset.camera.position += right * context.game_base_preset.camera_3d_move_speed * context.frame_delta; }
    if is_key_down(KeyCode::A) { context.game_base_preset.camera.position -= right * context.game_base_preset.camera_3d_move_speed * context.frame_delta; }
}