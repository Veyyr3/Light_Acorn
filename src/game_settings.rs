// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

use crate::acorn_tools::acorn_game_tools::prelude::*;
use macroquad::prelude::*;
use bevy_ecs::prelude::*;

pub const ACORN_MODEL:  usize = 0;
pub const FINISH_MODEL:  usize = 1;
pub const COIN_MODEL:  usize = 2;

pub struct Finish {
    pub position: Vec3,
    pub aabb: AcornAABB,
}

impl Default for Finish {
    fn default() -> Self {
        Self {
            position: vec3(45.0, 15.0, 10.0),
            aabb: AcornAABB {
                min: vec3(-2.0, -2.0, -2.0),
                max: vec3(2.0, 2.0, 2.0)
            }
        }
    }
}