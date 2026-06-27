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
use crate::acorn_tools::acorn_game_tools::prelude::*; // AABB
use bevy_ecs::prelude::*;

// ---------------------------- Structs ----------------------------

#[allow(dead_code)]
/// ## Description
/// Common 3D camera.
/// 
/// ## Fields
/// **Main fields**
/// * `position` – camera position (obviously) 
/// * `look_speed` – mouse look sensitivity
/// * `move_speed` – camera move speed (for special functions)
/// 
/// **Look fields**
/// * `look` – the spot the camera looks at
/// * `yaw` and `pitch` – for camera rotation
/// * `right` – to mark the right/left side of the camera
pub struct Acorn3DCamera {
    pub position: Vec3,
    pub look_speed: f32,
    pub move_speed: f32, 
    // look settings
    pub look: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub right: Vec3
}

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Acorn3DCamera {
    /// Create camera
    pub fn create(position: Vec3, look_speed: f32, move_speed: f32) -> Self {
        let look = agt_camera_get_look_dir(0.0, 0.0);

        Self { 
            position, 
            look_speed,
            move_speed,
            look,
            yaw: 0.0, 
            pitch: 0.0, 
            right: look.cross(vec3(0.0, 1.0, 0.0)).normalize()
        }
    }
}

#[allow(dead_code)]
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
        let position = vec3(0.0, 0.0, 0.0);
        let look = agt_camera_get_look_dir(0.0, 0.0);

        Self{
            position,
            look_speed: 1.0,
            move_speed: 10.0,
            look,
            yaw: 0.0,
            pitch: 0.0,
            right: look
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

// ---------------------------- Acorn Before 2D Zone Functions ----------------------------

#[allow(dead_code)]
/// Add to before 2d zone (in after 2d zone it may work incorrect)
pub fn agt_3d_camera(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // spawn camera
    set_camera(&Camera3D {
        position: context.game_base_preset.camera.position,
        up: vec3(0.0, 1.0, 0.0),
        target: context.game_base_preset.camera.look,
        ..Default::default()
    });
}

#[allow(dead_code)]
/// Add to before 2d zone (in after 2d zone it may work incorrect)
pub fn agt_3d_camera_physical(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // spawn camera
    set_camera(&Camera3D {
        position: context.game_base_preset.camera_physical.position,
        up: vec3(0.0, 1.0, 0.0),
        target: context.game_base_preset.camera_physical.look,
        ..Default::default()
    });
}

// ---------------------------- Acorn UI Functions ----------------------------

pub fn agt_3d_camera_common_control(
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
}

#[allow(dead_code)]
/// ## Description
/// Allows your camera to fly freely.
/// 
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase,`
/// 
/// ## Necessary functions in Zones for full functionality:
/// * `agt_camera` in `before_2d_zone`
/// 
/// ## Example: 
/// ```
/// let ui_input_zone = zone! {
///     location! {
///         agt_camera_3d_control_free_fly, // update camera position and look
///     }
/// };
/// 
/// let before_2d_zone = zone! {
///    location! {
///        agt_camera, // camera should be here first in this Zone!
///    }
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

#[allow(dead_code)]
/// ## Description
/// Allows your camera walk and jump through objects.
/// 
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase`
/// 
/// ## Necessary functions in Zones for full functionality:
/// * `agt_camera_physical` in `before_2d_zone`
/// 
/// ## Example: 
/// ```
/// let ui_input_zone = zone! {
///     location! {
///         agt_camera_3d_control_fps,
///     }
/// };
/// 
/// let before_2d_zone = zone! {
///    location! {
///        agt_camera_physical, // camera should be here first in this Zone!
///    }
/// };
/// ```
pub fn agt_camera_3d_control_fps(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // take from context
    let camera = &mut context.game_base_preset.camera_physical;
    let frame_delta = &mut context.frame_delta;

    // 1. rotate
    let mouse_delta = mouse_delta_position();
    camera.yaw -= mouse_delta.x * camera.look_speed; 
    camera.pitch += mouse_delta.y * camera.look_speed;
    camera.pitch = camera.pitch.clamp(-1.5, 1.5);

    // 2. calculate look dir
    let look_dir = agt_camera_get_look_dir(camera.yaw, camera.pitch);
    
    // so as not to fly up/down
    let mut move_dir = vec3(look_dir.x, 0.0, look_dir.z);
    if move_dir.length_squared() > 0.0 {
        move_dir = move_dir.normalize();
    }
    
    let right = move_dir.cross(vec3(0.0, 1.0, 0.0)).normalize();

    // 3. move
    let mut input_move = vec3(0.0, 0.0, 0.0);
    if is_key_down(KeyCode::W) { input_move += move_dir; }
    if is_key_down(KeyCode::S) { input_move -= move_dir; }
    if is_key_down(KeyCode::D) { input_move += right; }
    if is_key_down(KeyCode::A) { input_move -= right; }

    if input_move.length_squared() > 0.0 {
        camera.position += input_move.normalize() * camera.move_speed * *frame_delta;
    }

    // 4. physic + jumping
    
    // on the ground?
    if camera.position.y <= camera.look_height {
        camera.position.y = camera.look_height;
        camera.velocity_y = 0.0;
        camera.is_grounded = true;
    } else {
        camera.is_grounded = false;
    }

    // jump if on the ground
    if camera.is_grounded && is_key_pressed(KeyCode::Space) {
        camera.velocity_y = camera.jump_force;
        camera.is_grounded = false;
    }

    // if camera fly
    if !camera.is_grounded {
        camera.velocity_y -= camera.gravity_force * *frame_delta;
    }

    // vertical speed
    camera.position.y += camera.velocity_y * *frame_delta;

    // on the ground? Again!
    if camera.position.y < camera.look_height {
        camera.position.y = camera.look_height;
        camera.velocity_y = 0.0;
        camera.is_grounded = true;
    }

    // 5. set look for camera
    camera.look = camera.position + look_dir;
}

/// experimental
pub fn agt_camera_3d_control_fps_collision_experimental(
    _world: &mut World, 
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext
) {
    let camera = &mut context.game_base_preset.camera_physical;
    let a = &mut context.frame_delta;
    let dt = *a;

    // --- ДЛЯ ТЕСТА: Создадим пару кубов-препятствий (в реальной игре они будут в _zones) ---
    let environment_walls = vec![
        // Куб в центре карты размером 2x2x2 метра
        AcornAABB { min: vec3(-1.0, 0.0, -5.0), max: vec3(1.0, 2.0, -3.0) },
        // Длинная стена справа
        AcornAABB { min: vec3(4.0, 0.0, -10.0), max: vec3(5.0, 3.0, 10.0) },
    ];
    // ----------------------------------------------------------------------------------

    // 1. Вращение
    let mouse_delta = mouse_delta_position();
    camera.yaw -= mouse_delta.x * camera.look_speed; 
    camera.pitch += mouse_delta.y * camera.look_speed;
    camera.pitch = camera.pitch.clamp(-1.5, 1.5);

    // 2. Расчет векторов направления
    let look_dir = agt_camera_get_look_dir(camera.yaw, camera.pitch);
    let mut move_dir = vec3(look_dir.x, 0.0, look_dir.z);
    if move_dir.length_squared() > 0.0 {
        move_dir = move_dir.normalize();
    }
    let right = move_dir.cross(vec3(0.0, 1.0, 0.0)).normalize();

    // 3. Расчет желаемого горизонтального смещения
    let mut input_move = vec3(0.0, 0.0, 0.0);
    if is_key_down(KeyCode::W) { input_move += move_dir; }
    if is_key_down(KeyCode::S) { input_move -= move_dir; }
    if is_key_down(KeyCode::D) { input_move += right; }
    if is_key_down(KeyCode::A) { input_move -= right; }

    let horizontal_displacement = if input_move.length_squared() > 0.0 {
        input_move.normalize() * camera.move_speed * dt
    } else {
        vec3(0.0, 0.0, 0.0)
    };

    // Вспомогательная лямбда: строит AABB игрока вокруг заданной позиции Y-координата ног игрока = position.y - look_height
    let get_player_aabb = |pos: Vec3| -> AcornAABB {
        let radius = 0.3; // Ширина игрока (0.6 метра суммарно)
        AcornAABB {
            min: vec3(pos.x - radius, pos.y - camera.look_height, pos.z - radius),
            max: vec3(pos.x + radius, pos.y, pos.z + radius),
        }
    };

    // --- ДВИЖЕНИЕ И КОЛЛИЗИИ ПО ОСИ X ---
    if horizontal_displacement.x != 0.0 {
        camera.position.x += horizontal_displacement.x;
        let player_aabb = get_player_aabb(camera.position);
        
        // Если пересеклись со стеной — отменяем шаг по X
        for wall in &environment_walls {
            if player_aabb.intersects(wall) {
                camera.position.x -= horizontal_displacement.x;
                break;
            }
        }
    }

    // --- ДВИЖЕНИЕ И КОЛЛИЗИИ ПО ОСИ Z ---
    if horizontal_displacement.z != 0.0 {
        camera.position.z += horizontal_displacement.z;
        let player_aabb = get_player_aabb(camera.position);
        
        // Если пересеклись со стеной — отменяем шаг по Z
        for wall in &environment_walls {
            if player_aabb.intersects(wall) {
                camera.position.z -= horizontal_displacement.z;
                break;
            }
        }
    }

    // 4. Вертикальная физика (Гравитация, Прыжок и Коллизии по Y)
    
    // Прыжок (доступен только если мы стоим на земле)
    if camera.is_grounded && is_key_pressed(KeyCode::Space) {
        camera.velocity_y = camera.jump_force;
        camera.is_grounded = false;
    }

    // Применяем гравитацию
    if !camera.is_grounded {
        camera.velocity_y -= camera.gravity_force * dt;
    }

    // Применяем вертикальный сдвиг
    camera.position.y += camera.velocity_y * dt;
    camera.is_grounded = false; // Сбрасываем флаг перед проверками

    // Проверка коллизии по Y со стенами (потолками/платформами)
    let player_aabb = get_player_aabb(camera.position);
    for wall in &environment_walls {
        if player_aabb.intersects(wall) {
            // Если летели вниз — приземляемся НА объект
            if camera.velocity_y < 0.0 {
                camera.position.y = wall.max.y + camera.look_height;
                camera.velocity_y = 0.0;
                camera.is_grounded = true;
            } 
            // Если летели вверх — бьемся головой об ПОТОЛОК объекта
            else if camera.velocity_y > 0.0 {
                camera.position.y = wall.min.y - 0.01; // Смещаем чуть ниже потолка
                camera.velocity_y = 0.0;
            }
            break;
        }
    }

    // Самая базовая проверка «пола по умолчанию» на высоте 0 (твой старый код)
    if camera.position.y < camera.look_height {
        camera.position.y = camera.look_height;
        camera.velocity_y = 0.0;
        camera.is_grounded = true;
    }

    // 5. Обновление точки взгляда
    camera.look = camera.position + look_dir;
}