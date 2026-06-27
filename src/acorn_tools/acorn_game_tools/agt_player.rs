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

// ---------------------------- Structs ----------------------------

pub struct AcornPlayer3D {
    pub position: Vec3,
    pub look_position: Vec3,
}

// ---------------------------- Components ----------------------------

#[derive(Component)]
pub struct AcornIsPlayer;

// ---------------------------- Implementations ----------------------------

impl AcornPlayer3D {
    pub fn new(position: Vec3, look_position: Vec3) -> Self {
        Self {
            position,
            look_position
        }
    }
}

// ---------------------------- Default Behaviors ----------------------------

impl Default for AcornPlayer3D {
    fn default() -> Self {
        Self::new(
            vec3(0.0, 0.0, 0.0), 
            vec3(0.0, 1.0, 0.0)
        ) 
    }
}

// ---------------------------- Acorn Functions ----------------------------
#[allow(dead_code)]
/// Add to before 2d zone (in after 2d zone it may work incorrect)
pub fn agt_player_3d_camera(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // let player_postion = ;

    // spawn camera
    set_camera(&Camera3D {
        position: context.game_base_preset.camera.position,
        up: vec3(0.0, 1.0, 0.0),
        target: context.game_base_preset.camera.look,
        ..Default::default()
    });
}