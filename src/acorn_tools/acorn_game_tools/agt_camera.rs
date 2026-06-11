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

pub struct Acorn3DCameraPhysical {
    pub position: Vec3,
    pub look_speed: f32, // mouse look sensitivity
    pub move_speed: f32, // camera move speed
    // look settings
    pub look: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    // Physic fields
    pub velocity_y: f32, // to accumulate falling speed speed of falling
    pub is_grounded: bool, // on ground or not
    pub gravity_force: f32, // speed of falling. Usually 20.0
    pub jump_force: f32,
    pub look_height: f32, // camera's height under ground 
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

impl Acorn3DCameraPhysical {
    /// Create camera
    pub fn create(
        position: Vec3, 
        look_speed: f32, 
        move_speed: f32, 
        is_grounded: bool,
        gravity_force: f32, // Usually 20.0
        jump_force: f32,
        look_height: f32
    ) -> Self {
        Self { 
            position, 
            look_speed,
            move_speed,
            look: position + agt_camera_get_look_dir(0.0, 0.0),
            yaw: 0.0, 
            pitch: 0.0,
            velocity_y: 0.0,
            is_grounded,
            gravity_force,
            jump_force,
            look_height 
        }
    }
}

// ---------------------------- Implementations Default ----------------------------

impl Default for Acorn3DCamera {
    fn default() -> Self {
        Self{
            position: vec3(0.0, 0.0, 0.0),
            look_speed: 1.0,
            move_speed: 10.0,
            look: vec3(0.0, 1.8, 0.0),
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

impl Default for Acorn3DCameraPhysical {
    fn default() -> Self {
        Self {
            position: vec3(0.0, 1.8, 0.0),
            look_speed: 1.0,
            move_speed: 5.0,
            look: vec3(0.0, 1.8, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            velocity_y: 0.0,
            is_grounded: false,
            gravity_force: 20.0,
            jump_force: 7.0,
            look_height: 1.8,
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

pub fn agt_camera_3d_control_fps(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    let camera = &mut context.game_base_preset.camera_physical;
    let frame_delta = &mut context.frame_delta;

    // 1. Вращение (остается прежним)
    let mouse_delta = mouse_delta_position();
    camera.yaw -= mouse_delta.x * camera.look_speed; 
    camera.pitch += mouse_delta.y * camera.look_speed;
    camera.pitch = camera.pitch.clamp(-1.5, 1.5);

    // 2. Расчет векторов направления
    let look_dir = agt_camera_get_look_dir(camera.yaw, camera.pitch);
    
    // Проекция вектора движения на плоскость XZ (чтобы не летать вверх/вниз)
    let mut move_dir = vec3(look_dir.x, 0.0, look_dir.z);
    if move_dir.length_squared() > 0.0 {
        move_dir = move_dir.normalize();
    }
    
    // Вектор «вправо» для стрейфа
    let right = move_dir.cross(vec3(0.0, 1.0, 0.0)).normalize();

    // 3. Горизонтальное перемещение (W, S, A, D)
    let mut input_move = vec3(0.0, 0.0, 0.0);
    if is_key_down(KeyCode::W) { input_move += move_dir; }
    if is_key_down(KeyCode::S) { input_move -= move_dir; }
    if is_key_down(KeyCode::D) { input_move += right; }
    if is_key_down(KeyCode::A) { input_move -= right; }

    if input_move.length_squared() > 0.0 {
        camera.position += input_move.normalize() * camera.move_speed * *frame_delta;
    }

    // 4. Вертикальная физика (Гравитация и Прыжок)
    
    // Проверка: на земле ли мы?
    if camera.position.y <= camera.look_height {
        camera.position.y = camera.look_height;
        camera.velocity_y = 0.0;
        camera.is_grounded = true;
    } else {
        camera.is_grounded = false;
    }

    // Прыжок (доступен только на земле)
    if camera.is_grounded && is_key_pressed(KeyCode::Space) {
        camera.velocity_y = camera.jump_force;
        camera.is_grounded = false;
    }

    // Применяем гравитацию, если мы в воздухе
    if !camera.is_grounded {
        camera.velocity_y -= camera.gravity_force * *frame_delta;
    }

    // Применяем вертикальную скорость к позиции
    camera.position.y += camera.velocity_y * *frame_delta;

    // Повторная проверка коллизии после изменения позиции (чтобы не провалиться на этом кадре)
    if camera.position.y < camera.look_height {
        camera.position.y = camera.look_height;
        camera.velocity_y = 0.0;
        camera.is_grounded = true;
    }

    // 5. Обновление точки взгляда
    camera.look = camera.position + look_dir;
}