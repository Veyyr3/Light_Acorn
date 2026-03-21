// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_heart.rs
use macroquad::{math::Vec3, models::Mesh};
use bevy_ecs::prelude::*;

/// It is vector of your 3D models.
pub struct Acorn3DAssetDatabase {
    pub meshes: Vec<Mesh>
}

#[derive(Component)]
pub struct Entity3DTransform {
    pub position: Vec3,
    pub rotation: f32,
    pub scale: Vec3,
}

#[derive(Component)]
pub struct Entity3DModel {
    pub mesh_id: usize // instead of Mesh
}