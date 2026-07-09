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
    pub aabb: AcornAABB,
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
        aabb: AcornAABB
    ) -> Self {
        Self {
            position,
            eye_position,
            move_speed_forward,
            move_speed_backward,
            move_speed_side,
            jump_force,
            aabb
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
            30.0,
            AcornAABB { 
                min: vec3(-1.0, -1.0, -1.0), 
                max: vec3(1.0, 1.0, 1.0) 
            }
        ) 
    }
}

// ---------------------------- Acorn Functions ----------------------------
#[allow(dead_code)]
/// ## Description
/// Function links camera position to your player.
/// 
/// ## Related other Acorn Functions:
/// * [`agt_player_fps_speed_control`]
/// 
/// ## WARNING
/// **Put only AFTER function** `agt_do_entities_move` **or Functions Sets with Collision like** 'AGT_SIMPLE_COLLISION'.
pub fn agt_3d_camera_link_to_player(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext,
) {
    // get context
    let player = &mut context.game_base_preset.player;
    let camera = &mut context.game_base_preset.camera;
    
    // create query
    let mut query = 
        world.query_filtered::<&AcornEntity3DTransform, With<AcornIs3DPlayer>>();
    
    // take first entitiy with AcornIs3DPlayer.
    if let Some(player_transform) = query.iter(world).next() {
        player.position = player_transform.position;

        camera.position = player.position + player.eye_position;
    }
}

/// ## Description
/// Function allows to you control your player via WASD handling.
/// 
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase,`
/// 
/// ## Necessary pipeline in Zones for full functionality:
/// ```
/// let ui_input_zone = zone! {
///     location! {
///         agt_3d_camera_common_control, // update camera position and look
///         agt_player_fps_speed_control, // <- this function
///     }
/// };
/// 
/// let before_2d_zone = zone! {
///     // Minor-Location
///     location! {
///         agt_3d_camera, // camera should be here first!
///     },
///     location! {
///         agt_gravity_no_under_ground, // for jump
///     },
///     AGT_DEBUG_SLIDE_COLLISION, // collision
///     location! {
///         agt_3d_camera_link_to_player, // only after collision.
///     }
/// };
/// ```
pub fn agt_player_fps_speed_control(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext,
) {
    // get context
    let dt = context.frame_delta;
    let player = &context.game_base_preset.player; 
    let camera = &context.game_base_preset.camera;

    // Calculate the horizontal direction of the camera
    let look_dir = (camera.look - camera.position).normalize();
    let forward_xz = vec3(look_dir.x, 0.0, look_dir.z).normalize();
    let right_xz = forward_xz.cross(vec3(0.0, 1.0, 0.0)).normalize();

    // create query
    let mut query = world
        .query_filtered::<&mut Acorn3DSpeed, With<AcornIs3DPlayer>>();

    for mut speed in query.iter_mut(world) {
        let mut move_dir = Vec3::ZERO;
        let mut input_dir = Vec3::ZERO;
        let mut current_speed = player.move_speed_forward;

        if is_key_down(KeyCode::W) {
            input_dir += forward_xz;
            current_speed = player.move_speed_forward;
        }
        if is_key_down(KeyCode::S) {
            input_dir -= forward_xz;
            current_speed = player.move_speed_backward;
        }
        if is_key_down(KeyCode::D) {
            input_dir += right_xz;
            // Diagonal acceleration protection.
            if !is_key_down(KeyCode::W) && !is_key_down(KeyCode::S) {
                current_speed = player.move_speed_side;
            }
        }
        if is_key_down(KeyCode::A) {
            input_dir -= right_xz;
            // Diagonal acceleration protection.
            if !is_key_down(KeyCode::W) && !is_key_down(KeyCode::S) {
                current_speed = player.move_speed_side;
            }
        }

        // Normalize the input vector if it is non-zero
        if input_dir != Vec3::ZERO {
            move_dir = input_dir.normalize() * current_speed;
        }

        // Apply speed X, Z
        speed.speed_value.x = move_dir.x * dt;
        speed.speed_value.z = move_dir.z * dt;

        // Jump: SPACE
        if is_key_pressed(KeyCode::Space) {
            speed.speed_value.y = player.jump_force * dt;
        }
    }
}

/// ## Description
/// Spawn Bevy entity as Player.
/// 
/// ## Example:
/// ```
/// #[macroquad::main("Light Acorn test")]
/// async fn main() {
///     let mut acorn_ecs = acorn_ecs_setup();
///     let mut acorn_zone_context = acorn_zone_setup();
///     let mut acorn_global_context = acorn_global_setup();
/// 
///     agt_spawn_player( // <- put here
///         &mut acorn_ecs.world, 
///         &mut acorn_zone_context, 
///         &mut acorn_global_context
///     );
/// 
///     acorn_loop(acorn_ecs, acorn_zone_context, acorn_global_context).await;
/// }
/// ```
pub fn agt_spawn_player(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // get context
    let player = &context.game_base_preset.player;

    world.spawn((
        AcornEntity3DTransform {
            position: player.position,
            rotation: 0.0,
            scale: vec3(1.0, 1.0, 1.0)
        }, 
        player.aabb,
        Acorn3DSpeed {
            speed_value: Vec3::ZERO
        },
        AcornHasGravity,
        AcornIs3DPlayer,
    )); 
}