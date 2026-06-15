// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

use macroquad::prelude::*;
use bevy_ecs::prelude::*;
use std::collections::HashMap;
use crate::acorn_tools::acorn_game_tools::prelude::*;

use crate::acorn_settings::{AcornGlobalContext, AcornZoneContext};

// ---------------------------- Structs ----------------------------

pub struct Acorn2DWorldGrid {
    pub cells: HashMap<CellCoordinates, Vec<Entity>>,
    pub cell_size: f32,
}

#[derive(PartialEq, Eq, Hash)]
pub struct CellCoordinates {
    pub x: i32,
    pub y: i32,
}

// ---------------------------- Components ----------------------------

#[derive(Clone, Copy, Debug, Component)]
/// ## Description
/// A Bevy component. A simple stucture for collisions. Entities have a "box" for intersection to each other.
pub struct AcornAABB {
    pub min: Vec3,
    pub max: Vec3,
}

// new
#[derive(Component)]
pub struct AcornSimpleAABB {
    pub half_extents: Vec3
}

// useless
#[derive(Component)]
#[allow(dead_code)]
pub struct CollisionFlags {
    pub can_move_pos_x: bool,
    pub can_move_neg_x: bool,
    pub can_move_pos_y: bool,
    pub can_move_neg_y: bool,
    pub can_move_pos_z: bool,
    pub can_move_neg_z: bool,
}

// ---------------------------- Implementations ----------------------------

impl AcornAABB {
    #[allow(dead_code)]
    /// intersect between two entities with AABB
    pub fn intersects(&self, other: &AcornAABB) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
}

// ---------------------------- Default behaviors ----------------------------

impl Default for Acorn2DWorldGrid {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
            cell_size: 2.0,
        }
    }
}

// ---------------------------- Acorn Functions ----------------------------

pub fn agt_2d_grid_create(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    let cell_size = context.game_base_preset.world_collision_grid.cell_size;

    let grid = &mut context.game_base_preset.world_collision_grid;

    let mut query = world
        .query_filtered::<(Entity, &Entity3DTransform), With<AcornAABB>>();

    for (entity, transform) in query.iter(world) {
        let cell_x = (transform.position.x / cell_size).floor() as i32;
        let cell_y = (transform.position.y / cell_size).floor() as i32;

        let coord = CellCoordinates { x: cell_x, y: cell_y };

        // Add the entity ID to the corresponding cell
        grid.cells.entry(coord).or_insert_with(Vec::new).push(entity);
    }
}

pub fn agt_grid_check_collision(
    world: &mut World,
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    let grid = &context.game_base_preset.world_collision_grid;

    let mut query = world.query::<(&Entity3DTransform, &AcornAABB)>();

    // Take cells where there are at least 2 entities
    for (coord, entities) in grid.cells.iter().filter(|(_, e)| e.len() >= 2) {
        
        // Check everyone with everyone (Cartesian product)
        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let entity_a = entities[i];
                let entity_b = entities[j];

                if let (Ok((trans_a, aabb_a)), Ok((trans_b, aabb_b))) = 
                    (query.get(world, entity_a), query.get(world, entity_b)) 
                {
                    // Local AABB to World Coordinates
                    let world_min_a = trans_a.position + aabb_a.min;
                    let world_max_a = trans_a.position + aabb_a.max;
                    let world_min_b = trans_b.position + aabb_b.min;
                    let world_max_b = trans_b.position + aabb_b.max;

                    // AABB collisions
                    let is_colliding = 
                        world_min_a.x <= world_max_b.x && world_max_a.x >= world_min_b.x &&
                        world_min_a.y <= world_max_b.y && world_max_a.y >= world_min_b.y &&
                        world_min_a.z <= world_max_b.z && world_max_a.z >= world_min_b.z;

                    if is_colliding {
                        println!(
                            "[Grid] collision in ({}, {}): {:?} and {:?}", 
                            coord.x, coord.y, entity_a, entity_b
                        );
                    }
                }
            }
        }
    }
}

pub fn agt_grid_clear(
    _world: &mut World,
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // Clear cells every frame (Sparse Spatial Grid)
    context.game_base_preset.world_collision_grid.cells.clear(); 
}

// ---------------------------- Public Functions ----------------------------

#[allow(dead_code)]
pub fn intersects_between_two_aabb(first: &AcornAABB, second: &AcornAABB) -> bool {
    first.min.x <= second.max.x && first.max.x >= second.min.x &&
    first.min.y <= second.max.y && first.max.y >= second.min.y &&
    first.min.z <= second.max.z && first.max.z >= second.min.z
}