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
            look: position + agt_camera_get_look_dir(0.0, 0.0),
            yaw: 0.0, 
            pitch: 0.0 
        }
    }
}

// ---------------------------- Functions ----------------------------

fn agt_camera_get_look_dir(yaw: f32, pitch: f32) -> Vec3 {
    vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos()
    ).normalize()
}

// ---------------------------- Acorn Functions ----------------------------

/// ## Description
/// Allows your camera to fly freely.
/// 
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase,`
/// 
/// ## Example: 
/// ```
/// let ui_input_zone = zone! {
///     location! {
///         agt_camera_3d_control_free_fly, // update camera position and look
///     }
/// };
/// ```
pub fn agt_camera_3d_control_free_fly(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // take from context
    let camera = &mut context.game_base_preset.camera;
    let frame_delta = &mut context.frame_delta;

    // 1. rotate
    let mouse_delta = mouse_delta_position();
    camera.yaw -= mouse_delta.x * camera.look_speed; 
    camera.pitch += mouse_delta.y * camera.look_speed;
    camera.pitch = camera.pitch.clamp(-1.5, 1.5);

    // 2. calculate look dir
    let look_dir = agt_camera_get_look_dir(camera.yaw, camera.pitch);
    let right = look_dir.cross(vec3(0.0, 1.0, 0.0)).normalize();

    // 3. move
    if is_key_down(KeyCode::W) { camera.position += look_dir * camera.move_speed * *frame_delta; }
    if is_key_down(KeyCode::S) { camera.position -= look_dir * camera.move_speed * *frame_delta; }

    if is_key_down(KeyCode::D) { camera.position += right * camera.move_speed * *frame_delta; }
    if is_key_down(KeyCode::A) { camera.position -= right * camera.move_speed * *frame_delta; }

    // 4. set look for camera
    camera.look = camera.position + look_dir;
}