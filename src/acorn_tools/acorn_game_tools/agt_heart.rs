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

// ---------------------------- Structs ----------------------------

/// ## Description
/// It is vector of your 3D models.
/// 
/// Add this into `AcornGlobalContext` in `acorn_settings.rs`
pub struct Acorn3DAssetDatabase {
    pub meshes: Vec<Mesh>
}

#[derive(Component)]
/// ## Description
/// A Bevy component. Keep here transform of your 3D models.
/// 
/// ## Example:
/// ```
/// world.spawn((
///     Entity3DTransform {
///         position: vec3(0.0, 1.0, 0.0),
///         rotation: 0.0,
///         scale: vec3(1.0, 1.0, 1.0)
///     }, 
/// ));
/// ```
pub struct Entity3DTransform {
    pub position: Vec3,
    pub rotation: f32,
    pub scale: Vec3,
}

#[derive(Component)]
/// ## Description
/// A Bevy component. Keep here index of Mesh (3D model) from Acorn3DAssetDatabase which you want to add for your 3D entity.
/// 
/// ## Example
/// ```
/// world.spawn((
///     Entity3DModel {
///         // WARNING: you should remember index of your 3d model
///         mesh_id: 0 

///         /*
///         But you can use a trick:

///         // src/game_assets.rs
///         pub const ACORN_MODEL: usize = 0;
///         pub const TREE_MODEL: usize = 1;
///         pub const ROCK_MODEL: usize = 2;

///         AND write like that:
///         mesh_id: ACORN_MODEL
///         */
///     },
/// ));
/// ```
pub struct Entity3DModel {
    pub mesh_id: usize // instead of Mesh
}

#[derive(Clone, Copy, Debug, Component)]
/// ## Description
/// A simple stucture for collisions. Entities have a "box" for intersection to each other.
pub struct AcornAABB {
    pub min: Vec3,
    pub max: Vec3,
}

// ---------------------------- Implementations ----------------------------

impl AcornAABB {
    /// intersect between two entities with AABB
    pub fn intersects(&self, other: &AcornAABB) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
}