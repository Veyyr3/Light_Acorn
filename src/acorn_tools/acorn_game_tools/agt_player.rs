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

// ---------------------------- Impls ----------------------------

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