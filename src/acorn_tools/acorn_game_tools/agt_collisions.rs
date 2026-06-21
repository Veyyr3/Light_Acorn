// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

use macroquad::prelude::*;
use bevy_ecs::prelude::*;
use crate::acorn_tools::acorn_game_tools::prelude::*;
use crate::acorn_kernel::prelude::*;
use std::collections::HashMap;

use crate::acorn_settings::{AcornGlobalContext, AcornZoneContext}; // for Acorn functions

// sugar macros
use crate::{location};

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

#[derive(Debug, Clone, Component)]
/// ## Description
/// Speed of entities on XYZ.
/// 
/// Use it to add speed for your entities. It is necessary for entities moving and collisions.
pub struct Acorn3DSpeed {
    pub speed_value: Vec3,
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
        .query_filtered::<(Entity, &AcornEntity3DTransform), With<AcornAABB>>();

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

    let mut query = world.query::<(&AcornEntity3DTransform, &AcornAABB)>();

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

pub fn agt_grid_do_simple_collision(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext
) {
    let grid = &context.game_base_preset.world_collision_grid;
    let mut query = world.query::<(&AcornEntity3DTransform, &mut Acorn3DSpeed, &AcornAABB)>();

    for (_coord, entities) in grid.cells.iter().filter(|(_, e)| e.len() >= 2) {
        for i in 0..entities.len() {
            for j in 0..entities.len() {
                if i == j { continue; } // Проверяем объект со всеми соседями в клетке

                let entity_a = entities[i];
                let entity_b = entities[j];

                // Получаем мутабельный доступ к скорости А, и иммутабельный к Б
                if let Ok([(trans_a, mut speed_a, aabb_a), (trans_b, _, aabb_b)]) = 
                    query.get_many_mut(world, [entity_a, entity_b]) 
                {
                    // Если у А скорость нулевая, проверять нечего
                    if speed_a.speed_value == Vec3::ZERO { continue; }

                    // 1. Текущее положение объекта Б (препятствие)
                    let b_min = trans_b.position + aabb_b.min;
                    let b_max = trans_b.position + aabb_b.max;

                    // 2. ГИПОТЕТИЧЕСКОЕ положение объекта А (Текущая позиция + Желаемая скорость)
                    let future_pos_a = trans_a.position + speed_a.speed_value;
                    let a_future_min = future_pos_a + aabb_a.min;
                    let a_future_max = future_pos_a + aabb_a.max;

                    // Проверяем, столкнутся ли они в будущем кадре
                    let will_collide = 
                        a_future_min.x <= b_max.x && a_future_max.x >= b_min.x &&
                        a_future_min.y <= b_max.y && a_future_max.y >= b_min.y &&
                        a_future_min.z <= b_max.z && a_future_max.z >= b_min.z;

                    if will_collide {
                        // Пофигукс! Столкновение неизбежно. Гасим скорость.
                        // Для идеального скольжения вдоль стен можно гасить только ту ось, 
                        // которая пересекает границу, но для "Simple Collision" — обнуляем вектор целиком:
                        speed_a.speed_value = Vec3::ZERO;
                        
                        // Или альтернативный вариант (проверка по осям):
                        // Из-за дискретности шага обнуление всего Vec3 гарантирует 100% остановку без проваливания.
                        
                        println!("[Agt Физика] Скорость сущности {:?} погашена перед {:?}", entity_a, entity_b);
                    }
                }
            }
        }
    }
}

pub fn agt_2d_grid_clear(
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

// ---------------------------- Acorn Functions Sets ----------------------------
// const AGT_SIMPLE_COLLISION: (AcornFunction, AcornFunction, AcornFunction) = (agt_2d_grid_create, agt_grid_do_simple_collision, agt_2d_grid_clear);