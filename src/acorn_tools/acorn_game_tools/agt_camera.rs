// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

use macroquad::prelude::*;
use crate::acorn_settings::{
    AcornZoneContext,
    AcornGlobalContext,
};
use bevy_ecs::prelude::*;

// ---------------------------- Structs ----------------------------

pub struct AcornCamera {
    // look settings
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

// ---------------------------- Implementations ----------------------------

impl AcornCamera {
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

pub fn acorn_game_camera_free_fly(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    let look_speed = 1.0;
    let move_speed = 1.0;

    // 1. rotate
    let mouse_delta = mouse_delta_position();
    context.camera.yaw -= mouse_delta.x * look_speed; 
    context.camera.pitch += mouse_delta.y * look_speed;
    context.camera.pitch = context.camera.pitch.clamp(-1.5, 1.5);

    let look_dir = acorn_game_camera_get_look_dir(context.camera.yaw, context.camera.pitch);

    // 2. move
    if is_key_down(KeyCode::W) { context.camera.position += look_dir * move_speed; }
    if is_key_down(KeyCode::S) { context.camera.position -= look_dir * move_speed; }

    let right = look_dir.cross(vec3(0.0, 1.0, 0.0)).normalize();
    if is_key_down(KeyCode::D) { context.camera.position += right * move_speed; }
    if is_key_down(KeyCode::A) { context.camera.position -= right * move_speed; }
}