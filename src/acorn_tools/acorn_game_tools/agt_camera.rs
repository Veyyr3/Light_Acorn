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
    pub position: Vec3,
    pub look_speed: f32, // mouse look sensitivity
    pub move_speed: f32, // camera move speed
    // look settings
    pub look: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

// ---------------------------- Implementations ----------------------------

impl Acorn3DCamera {
    /// Create camera
    pub fn create(position: Vec3, look_speed: f32, move_speed: f32) -> Self {
        Self { 
            position, 
            look_speed,
            move_speed,
            look: position + acorn_game_camera_get_look_dir(0.0, 0.0),
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
pub fn acorn_game_camera_3d_control_free_fly(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // take context
    let ctx = &mut context.game_base_preset;

    // 1. rotate
    let mouse_delta = mouse_delta_position();
    ctx.camera.yaw -= mouse_delta.x * ctx.camera.look_speed; 
    ctx.camera.pitch += mouse_delta.y * ctx.camera.look_speed;
    ctx.camera.pitch = ctx.camera.pitch.clamp(-1.5, 1.5);

    let look_dir = acorn_game_camera_get_look_dir(ctx.camera.yaw, ctx.camera.pitch);

    // set look for camera (camera_pos Vec3 + camera_look Vec3)
    ctx.camera.look = ctx.camera.position + look_dir;

    // 2. move
    // Here: position += look_dir * camera_3d_move_speed * frame_delta
    if is_key_down(KeyCode::W) { ctx.camera.position += look_dir * ctx.camera.move_speed; }
    if is_key_down(KeyCode::S) { ctx.camera.position -= look_dir * ctx.camera.move_speed; }

    let right = look_dir.cross(vec3(0.0, 1.0, 0.0)).normalize();
    if is_key_down(KeyCode::D) { ctx.camera.position += right * ctx.camera.move_speed; }
    if is_key_down(KeyCode::A) { ctx.camera.position -= right * ctx.camera.move_speed; }
}