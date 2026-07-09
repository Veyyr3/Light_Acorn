// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_presets.rs

use crate::acorn_tools::acorn_game_tools::prelude::*;

/*
This file contains frequent fields used in game development.

This is necessary for brevity and composition.
*/

// ---------------------------- Structs ----------------------------

#[derive(Default)]
/// ## Description
/// Base preset to build games.
pub struct Acorn3DGameBase {
  // Cameras
  pub camera: Acorn3DCamera, // 3D camera
  pub camera_physical: Acorn3DCameraPhysical, // 3D camera with physic
  // Collisions
  pub world_collision_grid: AcornXZWorldGrid,
  // 3D models
  pub assets_3d: Acorn3DAssetDatabase, // 3D meshes
  // Special
  pub gravity_force: f32,
  // Player
  pub player: AcornPlayer3D,
}