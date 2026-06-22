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
use std::collections::HashMap;
// for Acorn functions
use crate::acorn_settings::{AcornGlobalContext, AcornZoneContext};

// ---------------------------- Structs ----------------------------

/// ## Description
/// Use it to add collisions between entities. This struct is included in `Acorn3DGameBase`.
/// 
/// Acorn collision based on **Sparse Spatial Grid**. An invisible grid is stretched across the entire game world along the XZ axes, the cells of which exist only where entities exist. The grid is two-dimensional (only XZ, no Y).
/// 
/// ## How it works?
/// It means that the collision is triggered only in those places where the entities are close to each other (that is, in the same cell). It's very nice for optimization.
/// 
/// ## Fields description
/// * `cells` - contains cell coordinate and vector of Bevy entities ID.
/// * `cell_size` - the larger the cell size, the more entities can fit into one cell.
pub struct AcornXZWorldGrid {
    pub cells: HashMap<CellXZCoordinates, Vec<Entity>>,
    pub cell_size: f32,
}

#[derive(PartialEq, Eq, Hash)]
/// ## Description
/// It is inluded in `AcornXZWorldGrid`. Each cell is two-dimensional (only XZ, no Y).
pub struct CellXZCoordinates {
    pub x: i32,
    pub z: i32,
}

// ---------------------------- Components ----------------------------

#[derive(Clone, Copy, Debug, Component)]
/// ## Description
/// A Bevy component. A simple stucture for collisions. Entities have a "box" for intersection to each other.
/// 
/// ## Example in `AcornFunction` to spawn entitiy:
/// ```
/// world.spawn((
///    AcornAABB {
///        min: vec3(-1.0, -1.0, -1.0),
///        max: vec3(1.0, 1.0, 1.0)
///    },
/// ));, 
/// ```
pub struct AcornAABB {
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Debug, Clone, Component)]
/// ## Description
/// Speed of entities on XYZ.
/// 
/// Use it to add speed for your entities. It is necessary for entities moving and collisions.
/// 
/// ## Example in `AcornFunction` to spawn entitiy:
/// ```
/// world.spawn((
///     Acorn3DSpeed {
///       speed_value: Vec3::ZERO
///     },
/// ));, 
/// ```
pub struct Acorn3DSpeed {
    pub speed_value: Vec3,
}

// useless
#[allow(dead_code)]
#[derive(Component)]
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

impl Default for AcornXZWorldGrid {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
            cell_size: 5.0,
        }
    }
}

// ---------------------------- Acorn Functions ----------------------------

// ====== fn about XZ grid ======

#[allow(dead_code)]
/// ## Description
/// Create **Sparse Spatial Collision Grid** on XZ for whole game world.
/// 
/// It is necessary for full collision job.
/// 
/// **Be sure to place it BEFORE collision functions such as: `agt_xz_grid_do_simple_collision`, etc.**
/// 
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase`
pub fn agt_xz_grid_create(
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
        let cell_z = (transform.position.z / cell_size).floor() as i32;

        let coord = CellXZCoordinates { x: cell_x, z: cell_z };

        // Add the entity ID to the corresponding cell
        grid.cells.entry(coord).or_insert_with(Vec::new).push(entity);
    }
}

#[allow(dead_code)]
/// ## Description
/// Clear **Sparse Spatial Collision Grid** on XZ for whole game world.
/// 
/// It is necessary for full collision job.
/// 
/// **Be sure to place it AFTER collision functions such as: `agt_xz_grid_do_simple_collision`, etc.**
/// 
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase`
pub fn agt_xz_grid_clear(
    _world: &mut World,
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // Clear cells every frame (Sparse Spatial Grid)
    context.game_base_preset.world_collision_grid.cells.clear(); 
}

#[allow(dead_code)]
/// ## Description
/// Count all cells and cells with >= 2 entities. In cells where >= 2 will execute collision between entities.
/// 
/// **Be sure to place it BEFORE: `agt_xz_grid_clear`**
/// 
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase`
pub fn agt_xz_grid_debug_count_cells(
    _world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext
) {
    let grid = &context.game_base_preset.world_collision_grid;
    
    // all cells
    let total_cells = grid.cells.len();
    
    // cells with >= 2 entities
    let hot_cells = grid.cells
        .values()
        .filter(|entities| entities.len() >= 2)
        .count();
    
    println!(
        "[Acorn XZ Grid Debug] Cells in grid: {} | Grids with >=2 entities: {}", 
        total_cells, 
        hot_cells
    );
}

#[allow(dead_code)]
/// ## Description
/// Draw grid cells.
/// 
/// **Be sure to place it BEFORE: `agt_xz_grid_clear`**
/// 
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase`
pub fn agt_xz_grid_debug_draw(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    let grid = &context.game_base_preset.world_collision_grid;
    let cell_size = grid.cell_size;

    for coord in grid.cells.keys() {
        let world_x = (coord.x as f32 * cell_size) + (cell_size * 0.5);
        let world_y = 0.0;
        let world_z = (coord.z as f32 * cell_size) + (cell_size * 0.5);

        let position = Vec3::new(world_x, world_y, world_z);
        
        let size = Vec3::new(cell_size, 0.1, cell_size);
        
        let color = YELLOW;

        println!("[Acorn XZ Grid Debug] Draw cell position: {}", position);
        draw_cube_wires(position, size, color);
    }
}

// ====== fn about XZ grid collision ======

#[allow(dead_code)]
/// ## Description
/// Function for debug place collision.
/// 
/// ## Necessary set of Acorn functions for full functionality:
/// copy&paste this into `acorn_zsetup`:
/// ```
/// location! {
///     agt_xz_grid_create,
///     agt_xz_grid_debug_check_collision, // <-
///     agt_xz_grid_clear,
/// },
/// ```
///  
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase`
pub fn agt_xz_grid_debug_check_collision(
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
                            "[Acorn XZ Grid Debug] collision in ({}, {}): {:?} and {:?}", 
                            coord.x, coord.z, entity_a, entity_b
                        );
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
/// ## Description
/// Function for debug place collision.
/// 
/// ## Necessary set of Acorn functions for full functionality:
/// copy&paste this into `acorn_zsetup`:
/// ```
/// location! {
///     agt_xz_grid_create,
///     agt_xz_grid_do_simple_collision, // <-
///     agt_xz_grid_clear,
///     agt_do_entities_move, // This is necessary for the entities to move.
/// },
/// ```
///  
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase`
pub fn agt_xz_grid_do_simple_collision(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext
) {
    let grid = &context.game_base_preset.world_collision_grid;
    let mut query = world.query::<(&AcornEntity3DTransform, &mut Acorn3DSpeed, &AcornAABB)>();

    for (_coord, entities) in grid.cells.iter().filter(|(_, e)| e.len() >= 2) {
        for i in 0..entities.len() {
            for j in 0..entities.len() {
                if i == j { continue; } // do not check self (necessary 2 different Bevy entities ID)

                let entity_a = entities[i];
                let entity_b = entities[j];

                if let Ok([(trans_a, mut speed_a, aabb_a), (trans_b, _, aabb_b)]) = 
                    query.get_many_mut(world, [entity_a, entity_b]) 
                {
                    // if entity A has 0 speed, pass
                    if speed_a.speed_value == Vec3::ZERO { continue; }

                    // entity B
                    let b_min = trans_b.position + aabb_b.min;
                    let b_max = trans_b.position + aabb_b.max;

                    // future position of entity A
                    let future_pos_a = trans_a.position + speed_a.speed_value;
                    let a_future_min = future_pos_a + aabb_a.min;
                    let a_future_max = future_pos_a + aabb_a.max;

                    // collided?
                    let will_collide = 
                        a_future_min.x <= b_max.x && a_future_max.x >= b_min.x &&
                        a_future_min.y <= b_max.y && a_future_max.y >= b_min.y &&
                        a_future_min.z <= b_max.z && a_future_max.z >= b_min.z;

                    if will_collide {
                        // Set all speed to Zero
                        speed_a.speed_value = Vec3::ZERO;
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn agt_xz_grid_do_slide_collision(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext
) {
    let grid = &context.game_base_preset.world_collision_grid;
    // Возвращаем иммутабельный трансформ, мутабельную скорость
    let mut query = world.query::<(&AcornEntity3DTransform, &mut Acorn3DSpeed, &AcornAABB)>();

    for (coord, entities) in grid.cells.iter().filter(|(_, e)| e.len() >= 2) {
        for i in 0..entities.len() {
            for j in 0..entities.len() {
                if i == j { continue; } 

                let entity_a = entities[i];
                let entity_b = entities[j];

                if let Ok([(trans_a, mut speed_a, aabb_a), (trans_b, _, aabb_b)]) = 
                    query.get_many_mut(world, [entity_a, entity_b]) 
                {
                    if speed_a.speed_value == Vec3::ZERO { continue; }

                    let b_min = trans_b.position + aabb_b.min;
                    let b_max = trans_b.position + aabb_b.max;

                    // --- ТЕСТ ПО ОСИ X ---
                    if speed_a.speed_value.x != 0.0 {
                        let a_test_min = trans_a.position + Vec3::new(speed_a.speed_value.x, 0.0, 0.0) + aabb_a.min;
                        let a_test_max = trans_a.position + Vec3::new(speed_a.speed_value.x, 0.0, 0.0) + aabb_a.max;

                        let collide_x = 
                            a_test_min.x <= b_max.x && a_test_max.x >= b_min.x &&
                            (trans_a.position.y + aabb_a.min.y) <= b_max.y && (trans_a.position.y + aabb_a.max.y) >= b_min.y &&
                            (trans_a.position.z + aabb_a.min.z) <= b_max.z && (trans_a.position.z + aabb_a.max.z) >= b_min.z;

                        if collide_x {
                            // Перенаправляем скорость: сохраняем знак движения по X, 
                            // но добавляем эту кинетическую энергию к оси Z!
                            let push_dir_z = if speed_a.speed_value.z >= 0.0 { 1.0 } else { -1.0 };
                            speed_a.speed_value.z += speed_a.speed_value.x.abs() * push_dir_z;
                            
                            // Гасим оригинальный X
                            speed_a.speed_value.x = 0.0; 
                        }
                    }

                    // --- ТЕСТ ПО ОСИ Z ---
                    if speed_a.speed_value.z != 0.0 {
                        let a_test_min = trans_a.position + Vec3::new(0.0, 0.0, speed_a.speed_value.z) + aabb_a.min;
                        let a_test_max = trans_a.position + Vec3::new(0.0, 0.0, speed_a.speed_value.z) + aabb_a.max;

                        let collide_z = 
                            (trans_a.position.x + aabb_a.min.x) <= b_max.x && (trans_a.position.x + aabb_a.max.x) >= b_min.x &&
                            (trans_a.position.y + aabb_a.min.y) <= b_max.y && (trans_a.position.y + aabb_a.max.y) >= b_min.y &&
                            a_test_min.z <= b_max.z && a_test_max.z >= b_min.z;

                        if collide_z {
                            // Если врезались по Z — аналогично переносим остаток в X (если там свободно)
                            let push_dir_x = if speed_a.speed_value.x >= 0.0 { 1.0 } else { -1.0 };
                            speed_a.speed_value.x += speed_a.speed_value.z.abs() * push_dir_x;
                            
                            speed_a.speed_value.z = 0.0;
                        }
                    }

                    // --- ТЕСТ ПО ОСИ Y (Высота/Гравитация) ---
                    if speed_a.speed_value.y != 0.0 {
                        let a_test_min = trans_a.position + Vec3::new(0.0, speed_a.speed_value.y, 0.0) + aabb_a.min;
                        let a_test_max = trans_a.position + Vec3::new(0.0, speed_a.speed_value.y, 0.0) + aabb_a.max;

                        let collide_y = 
                            (trans_a.position.x + aabb_a.min.x) <= b_max.x && (trans_a.position.x + aabb_a.max.x) >= b_min.x &&
                            a_test_min.y <= b_max.y && a_test_max.y >= b_min.y &&
                            (trans_a.position.z + aabb_a.min.z) <= b_max.z && (trans_a.position.z + aabb_a.max.z) >= b_min.z;

                        if collide_y {
                            speed_a.speed_value.y = 0.0;
                        }
                    }

                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn agt_xz_grid_do_independent_collision(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext
) {
    let grid = &context.game_base_preset.world_collision_grid;
    let mut query = world.query::<(&AcornEntity3DTransform, &mut Acorn3DSpeed, &AcornAABB)>();

    for (coord, entities) in grid.cells.iter().filter(|(_, e)| e.len() >= 2) {
        for i in 0..entities.len() {
            for j in 0..entities.len() {
                if i == j { continue; } // Не проверяем самого себя

                let entity_a = entities[i];
                let entity_b = entities[j];

                if let Ok([(trans_a, mut speed_a, aabb_a), (trans_b, _, aabb_b)]) = 
                    query.get_many_mut(world, [entity_a, entity_b]) 
                {
                    if speed_a.speed_value == Vec3::ZERO { continue; }

                    // Текущие мировые границы объекта Б (препятствие)
                    let b_min = trans_b.position + aabb_b.min;
                    let b_max = trans_b.position + aabb_b.max;

                    // Текущие мировые границы объекта А (без учёта скорости)
                    let a_curr_min = trans_a.position + aabb_a.min;
                    let a_curr_max = trans_a.position + aabb_a.max;

                    // --- НЕЗАВИСИМЫЙ ТЕСТ ПО ОСИ X ---
                    if speed_a.speed_value.x != 0.0 {
                        // Сдвигаем текущие границы А только по X
                        let mut a_test_min = a_curr_min;
                        let mut a_test_max = a_curr_max;
                        a_test_min.x += speed_a.speed_value.x;
                        a_test_max.x += speed_a.speed_value.x;

                        // Проверяем пересечение с Б (Y и Z берутся исходные, без движения)
                        let collide_x = 
                            a_test_min.x <= b_max.x && a_test_max.x >= b_min.x &&
                            a_test_min.y <= b_max.y && a_test_max.y >= b_min.y &&
                            a_test_min.z <= b_max.z && a_test_max.z >= b_min.z;

                        if collide_x {
                            speed_a.speed_value.x = 0.0; // Врезались по X — блокируем только X
                        }
                    }

                    // --- НЕЗАВИСИМЫЙ ТЕСТ ПО ОСИ Y ---
                    if speed_a.speed_value.y != 0.0 {
                        // Сдвигаем текущие границы А только по Y
                        let mut a_test_min = a_curr_min;
                        let mut a_test_max = a_curr_max;
                        a_test_min.y += speed_a.speed_value.y;
                        a_test_max.y += speed_a.speed_value.y;

                        let collide_y = 
                            a_test_min.x <= b_max.x && a_test_max.x >= b_min.x &&
                            a_test_min.y <= b_max.y && a_test_max.y >= b_min.y &&
                            a_test_min.z <= b_max.z && a_test_max.z >= b_min.z;

                        if collide_y {
                            speed_a.speed_value.y = 0.0; // Врезались по Y — блокируем только Y
                        }
                    }

                    // --- НЕЗАВИСИМЫЙ ТЕСТ ПО ОСИ Z ---
                    if speed_a.speed_value.z != 0.0 {
                        // Сдвигаем текущие границы А только по Z
                        let mut a_test_min = a_curr_min;
                        let mut a_test_max = a_curr_max;
                        a_test_min.z += speed_a.speed_value.z;
                        a_test_max.z += speed_a.speed_value.z;

                        let collide_z = 
                            a_test_min.x <= b_max.x && a_test_max.x >= b_min.x &&
                            a_test_min.y <= b_max.y && a_test_max.y >= b_min.y &&
                            a_test_min.z <= b_max.z && a_test_max.z >= b_min.z;

                        if collide_z {
                            speed_a.speed_value.z = 0.0; // Врезались по Z — блокируем только Z
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn agt_xz_grid_debug_do_independent_collision(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext
) {
    let grid = &context.game_base_preset.world_collision_grid;
    let mut query = world.query::<(&AcornEntity3DTransform, &mut Acorn3DSpeed, &AcornAABB)>();

    for (coord, entities) in grid.cells.iter().filter(|(_, e)| e.len() >= 2) {
        for i in 0..entities.len() {
            for j in 0..entities.len() {
                if i == j { continue; }

                let entity_a = entities[i];
                let entity_b = entities[j];

                if let Ok([(trans_a, mut speed_a, aabb_a), (trans_b, _, aabb_b)]) = 
                    query.get_many_mut(world, [entity_a, entity_b]) 
                {
                    if speed_a.speed_value == Vec3::ZERO { continue; }

                    let b_min = trans_b.position + aabb_b.min;
                    let b_max = trans_b.position + aabb_b.max;

                    let a_curr_min = trans_a.position + aabb_a.min;
                    let a_curr_max = trans_a.position + aabb_a.max;

                    // Запоминаем исходную скорость для профайлера
                    let original_speed = speed_a.speed_value;

                    // --- ТЕСТ ПО ОСИ X ---
                    if speed_a.speed_value.x != 0.0 {
                        let mut a_test_min = a_curr_min;
                        let mut a_test_max = a_curr_max;
                        a_test_min.x += speed_a.speed_value.x;
                        a_test_max.x += speed_a.speed_value.x;

                        let collide_x = 
                            a_test_min.x <= b_max.x && a_test_max.x >= b_min.x &&
                            a_test_min.y <= b_max.y && a_test_max.y >= b_min.y &&
                            a_test_min.z <= b_max.z && a_test_max.z >= b_min.z;

                        if collide_x {
                            speed_a.speed_value.x = 0.0;
                            println!(
                                "[PROFILER X] Entity {:?} blocked by {:?}.\n -> MoveX: {}\n -> A_test_X: [{}..{}], B_X: [{}..{}]\n -> A_curr_Z: [{}..{}], B_Z: [{}..{}]",
                                entity_a, entity_b, original_speed.x, a_test_min.x, a_test_max.x, b_min.x, b_max.x, a_curr_min.z, a_curr_max.z, b_min.z, b_max.z
                            );
                        }
                    }

                    // --- ТЕСТ ПО ОСИ Y ---
                    if speed_a.speed_value.y != 0.0 {
                        let mut a_test_min = a_curr_min;
                        let mut a_test_max = a_curr_max;
                        a_test_min.y += speed_a.speed_value.y;
                        a_test_max.y += speed_a.speed_value.y;

                        let collide_y = 
                            a_test_min.x <= b_max.x && a_test_max.x >= b_min.x &&
                            a_test_min.y <= b_max.y && a_test_max.y >= b_min.y &&
                            a_test_min.z <= b_max.z && a_test_max.z >= b_min.z;

                        if collide_y {
                            speed_a.speed_value.y = 0.0;
                        }
                    }

                    // --- ТЕСТ ПО ОСИ Z ---
                    if speed_a.speed_value.z != 0.0 {
                        let mut a_test_min = a_curr_min;
                        let mut a_test_max = a_curr_max;
                        a_test_min.z += speed_a.speed_value.z;
                        a_test_max.z += speed_a.speed_value.z;

                        let collide_z = 
                            a_test_min.x <= b_max.x && a_test_max.x >= b_min.x &&
                            a_test_min.y <= b_max.y && a_test_max.y >= b_min.y &&
                            a_test_min.z <= b_max.z && a_test_max.z >= b_min.z;

                        if collide_z {
                            speed_a.speed_value.z = 0.0;
                            println!(
                                "[PROFILER Z] Entity {:?} blocked by {:?}.\n -> MoveZ: {}\n -> A_test_Z: [{}..{}], B_Z: [{}..{}]\n -> A_curr_X: [{}..{}], B_X: [{}..{}]",
                                entity_a, entity_b, original_speed.z, a_test_min.z, a_test_max.z, b_min.z, b_max.z, a_curr_min.x, a_curr_max.x, b_min.x, b_max.x
                            );
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn agt_xz_grid_do_predict_independent_collision(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext
) {
    let grid = &context.game_base_preset.world_collision_grid;
    let mut query = world.query::<(&AcornEntity3DTransform, &mut Acorn3DSpeed, &AcornAABB)>();

    for (coord, entities) in grid.cells.iter().filter(|(_, e)| e.len() >= 2) {
        for i in 0..entities.len() {
            for j in 0..entities.len() {
                if i == j { continue; }

                let entity_a = entities[i];
                let entity_b = entities[j];

                if let Ok([(trans_a, mut speed_a, aabb_a), (trans_b, _, aabb_b)]) = 
                    query.get_many_mut(world, [entity_a, entity_b]) 
                {
                    if speed_a.speed_value == Vec3::ZERO { continue; }

                    let b_min = trans_b.position + aabb_b.min;
                    let b_max = trans_b.position + aabb_b.max;

                    // Текущие границы объекта А без движения
                    let a_curr_min = trans_a.position + aabb_a.min;
                    let a_curr_max = trans_a.position + aabb_a.max;

                    // Проверяем, пересекаются ли объекты по осям ПРЯМО СЕЙЧАС (до движения)
                    let already_overlap_x = a_curr_min.x < b_max.x && a_curr_max.x > b_min.x;
                    let already_overlap_y = a_curr_min.y < b_max.y && a_curr_max.y > b_min.y;
                    let already_overlap_z = a_curr_min.z < b_max.z && a_curr_max.z > b_min.z;

                    // --- 1. ТЕСТ ПО ОСИ X ---
                    if speed_a.speed_value.x != 0.0 {
                        let a_test_min = trans_a.position + Vec3::new(speed_a.speed_value.x, 0.0, 0.0) + aabb_a.min;
                        let a_test_max = trans_a.position + Vec3::new(speed_a.speed_value.x, 0.0, 0.0) + aabb_a.max;

                        let collide_x = 
                            a_test_min.x <= b_max.x && a_test_max.x >= b_min.x &&
                            a_test_min.y <= b_max.y && a_test_max.y >= b_min.y &&
                            a_test_min.z <= b_max.z && a_test_max.z >= b_min.z;

                        // Блокируем X только если мы движемся вглубь объекта. 
                        // Если мы уже пересекались по Y и Z, и движемся ЕЩЕ И по X в стену — гасим.
                        if collide_x && (!already_overlap_x || (already_overlap_y && already_overlap_z)) {
                            speed_a.speed_value.x = 0.0;
                        }
                    }

                    // --- 2. ТЕСТ ПО ОСИ Y ---
                    if speed_a.speed_value.y != 0.0 {
                        let a_test_min = trans_a.position + Vec3::new(speed_a.speed_value.x, speed_a.speed_value.y, 0.0) + aabb_a.min;
                        let a_test_max = trans_a.position + Vec3::new(speed_a.speed_value.x, speed_a.speed_value.y, 0.0) + aabb_a.max;

                        let collide_y = 
                            a_test_min.x <= b_max.x && a_test_max.x >= b_min.x &&
                            a_test_min.y <= b_max.y && a_test_max.y >= b_min.y &&
                            a_test_min.z <= b_max.z && a_test_max.z >= b_min.z;

                        if collide_y && (!already_overlap_y || (speed_a.speed_value.x == 0.0 && already_overlap_z)) {
                            speed_a.speed_value.y = 0.0;
                        }
                    }

                    // --- 3. ТЕСТ ПО ОСИ Z ---
                    if speed_a.speed_value.z != 0.0 {
                        let a_test_min = trans_a.position + Vec3::new(speed_a.speed_value.x, speed_a.speed_value.y, speed_a.speed_value.z) + aabb_a.min;
                        let a_test_max = trans_a.position + Vec3::new(speed_a.speed_value.x, speed_a.speed_value.y, speed_a.speed_value.z) + aabb_a.max;

                        let collide_z = 
                            a_test_min.x <= b_max.x && a_test_max.x >= b_min.x &&
                            a_test_min.y <= b_max.y && a_test_max.y >= b_min.y &&
                            a_test_min.z <= b_max.z && a_test_max.z >= b_min.z;

                        if collide_z && (!already_overlap_z || (speed_a.speed_value.x == 0.0 && already_overlap_y)) {
                            speed_a.speed_value.z = 0.0;
                        }
                    }
                }
            }
        }
    }
}

// ====== fn about collision and entities ======

#[allow(dead_code)]
pub fn agt_do_entities_move(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    _context: &mut AcornGlobalContext
) {
    let mut query = world.query::<(&mut AcornEntity3DTransform, &Acorn3DSpeed)>();

    for (mut entity_transform, entity_speed) in query.iter_mut(world) {
        entity_transform.position.x += entity_speed.speed_value.x;
        entity_transform.position.y += entity_speed.speed_value.y;
        entity_transform.position.z += entity_speed.speed_value.z;
    }
}

#[allow(dead_code)]
pub fn agt_debug_entities_aabb_draw(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    let mut query = world.query::<(&AcornEntity3DTransform, &AcornAABB)>();

    for (transform, aabb) in query.iter(world) {
        // 1. Переводим локальные границы AABB в мировые координаты
        let world_min = transform.position + aabb.min;
        let world_max = transform.position + aabb.max;

        // 2. Вычисляем точный геометрический центр коробки в мире
        let position = (world_min + world_max) * 0.5;

        // 3. Вычисляем полный размер коробки по трем осям (длина, высота, ширина)
        let size = world_max - world_min;

        // Зеленый (или красный/бирюзовый) цвет традиционно хорош для хитбоксов
        let color = RED;

        // Отрисовка каркаса хитбокса сущности
        draw_cube(position, size, None, color);
    }
}

// ---------------------------- Public Functions ----------------------------

#[allow(dead_code)]
pub fn intersects_between_two_aabb(first: &AcornAABB, second: &AcornAABB) -> bool {
    first.min.x <= second.max.x && first.max.x >= second.min.x &&
    first.min.y <= second.max.y && first.max.y >= second.min.y &&
    first.min.z <= second.max.z && first.max.z >= second.min.z
}

// ---------------------------- Acorn Functions Sets ----------------------------
// const AGT_SIMPLE_COLLISION: (AcornFunction, AcornFunction, AcornFunction) = (agt_xz_grid_create, agt_grid_do_simple_collision, agt_xz_grid_clear);