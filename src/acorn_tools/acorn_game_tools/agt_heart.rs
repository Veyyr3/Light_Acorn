// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_heart.rs

use macroquad::prelude::*;
use bevy_ecs::prelude::*;
use std::collections::HashSet;

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

#[derive(Component)]
pub struct CollisionFlags {
    pub can_move_pos_x: bool,
    pub can_move_neg_x: bool,
    pub can_move_pos_y: bool,
    pub can_move_neg_y: bool,
    pub can_move_pos_z: bool,
    pub can_move_neg_z: bool,
}

#[derive(Component)]
pub struct AcornAABB {
    pub radius_size: Vec3,
}

// #[derive(Clone, Copy, Debug, Component)]
// /// ## Description
// /// A Bevy component. A simple stucture for collisions. Entities have a "box" for intersection to each other.
// pub struct AcornAABB {
//     pub min: Vec3,
//     pub max: Vec3,
// }

// ---------------------------- Struct (Resources) ----------------------------

#[derive(Resource, Default)]
pub struct AcornPhysicsWorld {
    // Храним координаты занятых блоков (например, как в Minecraft или сетке уровня)
    pub solid_blocks: HashSet<IVec3>, 
}

// ---------------------------- Implementations ----------------------------

// impl AcornAABB {
//     /// intersect between two entities with AABB
//     pub fn intersects(&self, other: &AcornAABB) -> bool {
//         self.min.x <= other.max.x && self.max.x >= other.min.x &&
//         self.min.y <= other.max.y && self.max.y >= other.min.y &&
//         self.min.z <= other.max.z && self.max.z >= other.min.z
//     }
// }

impl AcornPhysicsWorld {
    // Проверка одной точки: занята ли она? Время выполнения: O(1)
    pub fn is_solid(&self, pos: Vec3) -> bool {
        let grid_pos = IVec3::new(pos.x.round() as i32, pos.y.round() as i32, pos.z.round() as i32);
        self.solid_blocks.contains(&grid_pos)
    }
}

// ---------------------------- Default behaviors ----------------------------

impl Default for CollisionFlags {
    fn default() -> Self {
        Self {
            can_move_pos_x: true, can_move_neg_x: true,
            can_move_pos_y: true, can_move_neg_y: true,
            can_move_pos_z: true, can_move_neg_z: true,
        }
    }
}

// ---------------------------- Public Functions ----------------------------

// #[allow(dead_code)]
// pub fn intersects_between_two_aabb(first: &AcornAABB, second: &AcornAABB) -> bool {
//     first.min.x <= second.max.x && first.max.x >= second.min.x &&
//     first.min.y <= second.max.y && first.max.y >= second.min.y &&
//     first.min.z <= second.max.z && first.max.z >= second.min.z
// }