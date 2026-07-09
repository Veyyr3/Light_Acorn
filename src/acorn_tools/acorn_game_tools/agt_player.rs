// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_player.rs

use macroquad::prelude::*;
use bevy_ecs::prelude::*;
use crate::acorn_tools::acorn_game_tools::prelude::*;
// for Acorn functions
use crate::acorn_settings::{AcornGlobalContext, AcornZoneContext};

/*
Here are all for player.
*/

// ---------------------------- Structs ----------------------------

pub struct AcornPlayer3D {
    pub position: Vec3,
    pub eye_position: Vec3, // where the camera will be positioned relative to the player.
    pub move_speed_forward: f32,
    pub move_speed_backward: f32,
    pub move_speed_side: f32,
    pub jump_force: f32,
}

// ---------------------------- Components ----------------------------

#[derive(Component)]
pub struct AcornIs3DPlayer;

// ---------------------------- Implementations ----------------------------

impl AcornPlayer3D {
    pub fn new(
        position: Vec3, 
        eye_position: Vec3, 
        move_speed_forward: f32,
        move_speed_backward: f32,
        move_speed_side: f32,
        jump_force: f32,
    ) -> Self {
        Self {
            position,
            eye_position,
            move_speed_forward,
            move_speed_backward,
            move_speed_side,
            jump_force
        }
    }
}

// ---------------------------- Default Behaviors ----------------------------

impl Default for AcornPlayer3D {
    fn default() -> Self {
        Self::new(
            vec3(0.0, 0.0, 0.0), 
            vec3(0.0, 1.0, 0.0),
            5.0,
            2.0,
            3.0,
            2.0
        ) 
    }
}

// ---------------------------- Acorn Functions ----------------------------
#[allow(dead_code)]
/// Add to before 2d zone (in after 2d zone it may work incorrect)
pub fn agt_3d_camera_link_to_player(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext,
) {
    let player = &mut context.game_base_preset.player;
    let camera = &mut context.game_base_preset.camera;
    
    // Берем позицию трансформа игрока (предполагаем наличие компонента AcornEntity3DTransform)
    let mut query = 
        world.query_filtered::<&AcornEntity3DTransform, With<AcornIs3DPlayer>>();
    
    if let Some(player_transform) = query.iter(world).next() {
        // 1. Обновляем глобальные координаты игрока
        player.position = player_transform.position;

        // 2. Позиция камеры = Позиция игрока + локальное смещение камеры (eye_position)
        camera.position = player.position + player.eye_position;
    }
}

pub fn agt_player_fps_speed(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext,
) {
    let dt = context.frame_delta;
    let player = &context.game_base_preset.player; 
    let camera = &context.game_base_preset.camera;

    // Считаем горизонтальное направление камеры (проецируем на плоскость XZ, чтобы не взлетать)
    let look_dir = (camera.look - camera.position).normalize();
    let forward_xz = vec3(look_dir.x, 0.0, look_dir.z).normalize();
    
    // Вектор "вправо" получаем через векторное произведение forward_xz и мировой оси Y
    let right_xz = forward_xz.cross(vec3(0.0, 1.0, 0.0)).normalize();

    // Запрос к скоростям игрока
    let mut query = world.query_filtered::<&mut Acorn3DSpeed, With<AcornIs3DPlayer>>();

    for mut speed in query.iter_mut(world) {
        let mut move_dir = Vec3::ZERO;

        // Опрос WASD клавиш
        if is_key_down(KeyCode::W) {
            move_dir += forward_xz * player.move_speed_forward;
        }
        if is_key_down(KeyCode::S) {
            move_dir -= forward_xz * player.move_speed_backward;
        }
        if is_key_down(KeyCode::D) {
            move_dir += right_xz * player.move_speed_side;
        }
        if is_key_down(KeyCode::A) {
            move_dir -= right_xz * player.move_speed_side;
        }

        // Применяем скорость по осям X и Z с учетом дельты времени
        speed.speed_value.x = move_dir.x * dt;
        speed.speed_value.z = move_dir.z * dt;

        // Прыжок: если нажат Пробел, задаем импульс вверх по Y
        // (Гравитация сама потянет вниз в другой системе)
        if is_key_pressed(KeyCode::Space) {
            // Здесь предполагается проверка "на земле ли игрок", если твоя физика это позволяет.
            // Для простоты выставляем силу прыжка напрямую:
            speed.speed_value.y = player.jump_force * dt;
        }
    }
}

pub fn agt_spawn_player(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
   world.spawn((
        AcornEntity3DTransform {
            position: context.game_base_preset.player.position,
            rotation: 0.0,
            scale: vec3(1.0, 1.0, 1.0)
        }, 
        AcornAABB {
            min: vec3(-1.0, -1.0, -1.0),
            max: vec3(1.0, 1.0, 1.0)
        },
        Acorn3DSpeed {
            speed_value: Vec3::ZERO
        },
        AcornHasGravity,
        AcornIs3DPlayer,
    )); 
}