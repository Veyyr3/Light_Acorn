// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_presets.rs

use crate::acorn_tools::acorn_game_tools::agt_camera::Acorn3DCamera;

/*
This file contains frequent fields used in game development.

This is necessary for brevity and composition.
*/

/// ## Description
/// Base preset to build games.
pub struct Acorn3DGameBase {
  // camera
  pub camera: Acorn3DCamera, // 3D camera
}