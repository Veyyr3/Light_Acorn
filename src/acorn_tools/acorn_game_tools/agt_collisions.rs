// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_collisions.rs

use macroquad::prelude::*;
use bevy_ecs::prelude::*;
use crate::acorn_tools::acorn_game_tools::prelude::*;
use std::collections::HashMap;
// for Acorn functions
use crate::acorn_settings::{AcornGlobalContext, AcornZoneContext};

/*
Here are all for Collisions. 

Acorn Collision based on Predictive Sparse Spatial Grid.
*/

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

#[derive(Component)]
/// ## Description
/// Add this to your entities if they should be a trigger.
pub struct AcornIsTrigger {
    pub is_trigger: bool
}

#[derive(Component)]
/// ## Description
/// Add this to your entities if they are triggers.
/// 
/// You may use them in your own functions to detect the collision and make an event. For example, when a player touches the finish of level.
pub struct AcornIsCollided{
    pub is_collided: bool
}

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
/// 
/// ## Required entity components for collisions:
/// * `Acorn3DSpeed`
/// * `AcornEntity3DTransform`
/// * `AcornAABB`
pub fn agt_xz_grid_create(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // get context
    let cell_size = context.game_base_preset.world_collision_grid.cell_size;
    let grid = &mut context.game_base_preset.world_collision_grid;

    // create query
    let mut query = world
        .query_filtered::<(Entity, &AcornEntity3DTransform), With<AcornAABB>>();

    for (entity, transform) in query.iter(world) {
        // center of entity position
        let exact_x = transform.position.x / cell_size;
        let exact_z = transform.position.z / cell_size;

        // coordinates of the main cell
        let cell_x = exact_x.floor() as i32;
        let cell_z = exact_z.floor() as i32;

        // add the main cell into Grid
        grid.cells.entry(CellXZCoordinates { x: cell_x, z: cell_z })
            .or_insert_with(Vec::new)
            .push(entity);

        // offset from the main cell
        let fract_x = exact_x - exact_x.floor();
        let fract_z = exact_z - exact_z.floor();

        // determine the locations of neighbors.
        // If the center is in the right half (fract > 0.5), then the neighbor is to the right (+1), otherwise to the left (-1)
        let step_x = if fract_x >= 0.5 { 1 } else { -1 };
        let step_z = if fract_z >= 0.5 { 1 } else { -1 };

        // add neighbors for the main cell

        // neighbor (X)
        grid.cells.entry(CellXZCoordinates { x: cell_x + step_x, z: cell_z })
            .or_insert_with(Vec::new)
            .push(entity);

        // neighbor (Z)
        grid.cells.entry(CellXZCoordinates { x: cell_x, z: cell_z + step_z })
            .or_insert_with(Vec::new)
            .push(entity);

        // Diagonal neighbor (X and Z)
        grid.cells.entry(CellXZCoordinates { x: cell_x + step_x, z: cell_z + step_z })
            .or_insert_with(Vec::new)
            .push(entity);
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
/// 
/// ## Required entity components for collisions:
/// * `Acorn3DSpeed`
/// * `AcornEntity3DTransform`
/// * `AcornAABB`
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
///     agt_do_entities_move, // This is necessary for the entities to move.
/// },
/// ```
///  
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase`
/// 
/// ## Required entity components for collisions:
/// * `Acorn3DSpeed`
/// * `AcornEntity3DTransform`
/// * `AcornAABB`
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
/// Function for collision between 2 entities. The function resets the speed (XYZ) for an entity if its own future position collides with another entity.
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
/// 
/// ## Required entity components for collisions:
/// * `Acorn3DSpeed`
/// * `AcornEntity3DTransform`
/// * `AcornAABB`
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
/// ## Description
/// Function for collision between 2 entities. This function adds wall sliding. If an entity encounters an obstacle on the X-axis, all of its X-axis velocity is transferred to the Z-axis. And vice versa.
/// 
/// ## Necessary set of Acorn functions for full functionality:
/// copy&paste this into `acorn_zsetup`:
/// ```
/// location! {
///     agt_xz_grid_create,
///     agt_xz_grid_do_slide_collision, // <-
///     agt_xz_grid_clear,
///     agt_do_entities_move, // This is necessary for the entities to move.
/// },
/// ```
///  
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase`
/// 
/// ## Required entity components for collisions:
/// * `Acorn3DSpeed`
/// * `AcornEntity3DTransform`
/// * `AcornAABB`
pub fn agt_xz_grid_do_slide_collision(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext
) {
    let grid = &context.game_base_preset.world_collision_grid;
    let mut query = world.query::<(&AcornEntity3DTransform, &mut Acorn3DSpeed, &AcornAABB)>();

    for (_coord, entities) in grid.cells.iter().filter(|(_, e)| e.len() >= 2) {
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

                    // --- TEST X AXIS ---
                    if speed_a.speed_value.x != 0.0 {
                        let a_test_min = trans_a.position + Vec3::new(speed_a.speed_value.x, 0.0, 0.0) + aabb_a.min;
                        let a_test_max = trans_a.position + Vec3::new(speed_a.speed_value.x, 0.0, 0.0) + aabb_a.max;

                        let collide_x = 
                            a_test_min.x <= b_max.x && a_test_max.x >= b_min.x &&
                            (trans_a.position.y + aabb_a.min.y) <= b_max.y && (trans_a.position.y + aabb_a.max.y) >= b_min.y &&
                            (trans_a.position.z + aabb_a.min.z) <= b_max.z && (trans_a.position.z + aabb_a.max.z) >= b_min.z;

                        if collide_x {
                            // Add the speed from X to Z.
                            let push_dir_z = if speed_a.speed_value.z >= 0.0 { 1.0 } else { -1.0 };
                            speed_a.speed_value.z += speed_a.speed_value.x.abs() * push_dir_z;
                            
                            // X velocity is zero
                            speed_a.speed_value.x = 0.0; 
                        }
                    }

                    // --- TEST Z AXIS ---
                    if speed_a.speed_value.z != 0.0 {
                        let a_test_min = trans_a.position + Vec3::new(0.0, 0.0, speed_a.speed_value.z) + aabb_a.min;
                        let a_test_max = trans_a.position + Vec3::new(0.0, 0.0, speed_a.speed_value.z) + aabb_a.max;

                        let collide_z = 
                            (trans_a.position.x + aabb_a.min.x) <= b_max.x && (trans_a.position.x + aabb_a.max.x) >= b_min.x &&
                            (trans_a.position.y + aabb_a.min.y) <= b_max.y && (trans_a.position.y + aabb_a.max.y) >= b_min.y &&
                            a_test_min.z <= b_max.z && a_test_max.z >= b_min.z;

                        if collide_z {
                            // Add the speed from X to Z.
                            let push_dir_x = if speed_a.speed_value.x >= 0.0 { 1.0 } else { -1.0 };
                            speed_a.speed_value.x += speed_a.speed_value.z.abs() * push_dir_x;
                            
                            // Z velocity is zero
                            speed_a.speed_value.z = 0.0;
                        }
                    }

                    // --- TEST Y AXIS ---
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
/// ## Description
/// Function for collision between 2 entities. The function resets the speed (XYZ) for an entity if its own future position collides with another entity.
/// 
/// Also the function includes logic for triggers. 
/// Trigger is an entity that others can move through. 
/// When an entity moved through the trigger then trigger changes its own bool flag in [`AcornIsCollided`]. 
/// You can use this function to implement objects like finish, bonus, button and etc. in your game.
/// 
/// ## Necessary set of Acorn functions for full functionality:
/// copy&paste this into `acorn_zsetup`:
/// ```
/// location! {
///     agt_xz_grid_create,
///     agt_xz_grid_do_simple_collision_include_triggers, // <-
///     agt_xz_grid_clear,
///     agt_do_entities_move, // This is necessary for the entities to move.
/// },
/// ```
///  
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase`
/// 
/// ## Required entity components for collisions:
/// * [`Acorn3DSpeed`]
/// * [`AcornEntity3DTransform`]
/// * [`AcornAABB`]
/// * [`AcornIsTrigger`]
/// * [`AcornIsCollided`]
pub fn agt_xz_grid_do_simple_collision_include_triggers(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext
) {
    let grid = &context.game_base_preset.world_collision_grid;
    
    let mut query = world.query::<(
        &AcornEntity3DTransform, 
        &mut Acorn3DSpeed, 
        &AcornAABB, 
        &mut AcornIsCollided,
        &AcornIsTrigger
    )>();

    for (_coord, entities) in grid.cells.iter().filter(|(_, e)| e.len() >= 2) {
        for i in 0..entities.len() {
            for j in 0..entities.len() {
                if i == j { continue; } // do not check self (necessary 2 different Bevy entities ID)

                let entity_a = entities[i];
                let entity_b = entities[j];

                if let Ok([components_a, components_b]) = query.get_many_mut(world, [entity_a, entity_b]) {
                    let (trans_a, mut speed_a, aabb_a, _, _) = components_a;
                    let (trans_b, _, aabb_b, mut collided_b, trigger_b) = components_b;

                    // entity B coordinates
                    let b_min = trans_b.position + aabb_b.min;
                    let b_max = trans_b.position + aabb_b.max;

                    // --- ЖЕЛЕЗОБЕТОННАЯ ПРОВЕРКА КАСАНИЯ (МИКРОЛУЧИ / ЗАЗОР 0.1) ---
                    // Проверяем текущее положение, искусственно расширяя AABB на 0.1 во все стороны
                    let ray_padding = 0.1;
                    let a_current_min = trans_a.position + aabb_a.min - Vec3::splat(ray_padding);
                    let a_current_max = trans_a.position + aabb_a.max + Vec3::splat(ray_padding);

                    // Также проверяем предиктивное положение со смещением на скорость
                    let future_pos_a = trans_a.position + speed_a.speed_value;
                    let a_future_min = future_pos_a + aabb_a.min - Vec3::splat(ray_padding);
                    let a_future_max = future_pos_a + aabb_a.max + Vec3::splat(ray_padding);

                    let touching_now = 
                        a_current_min.x <= b_max.x && a_current_max.x >= b_min.x &&
                        a_current_min.y <= b_max.y && a_current_max.y >= b_min.y &&
                        a_current_min.z <= b_max.z && a_current_max.z >= b_min.z;

                    let touching_future = 
                        a_future_min.x <= b_max.x && a_future_max.x >= b_min.x &&
                        a_future_min.y <= b_max.y && a_future_max.y >= b_min.y &&
                        a_future_min.z <= b_max.z && a_future_max.z >= b_min.z;

                    if touching_now || touching_future {
                        // Если микролуч дотянулся до сущности B — стабильно взводим флаг
                        collided_b.is_collided = true;

                        // Если это триггер — мы зафиксировали факт касания и завершаем обработку этой пары
                        if trigger_b.is_trigger {
                            continue;
                        }
                    }

                    // --- ЧЕСТНАЯ ФИЗИКА ОСТАНОВКИ ДЛЯ ТВЕРДЫХ СТЕН ---
                    // Сюда код дойдёт, только если это стена (!trigger_b.is_trigger)
                    if speed_a.speed_value == Vec3::ZERO { continue; }

                    // Проверяем строго оригинальный хитбокс без каких-либо зазоров
                    let pure_future_pos_a = trans_a.position + speed_a.speed_value;
                    let pure_a_future_min = pure_future_pos_a + aabb_a.min;
                    let pure_a_future_max = pure_future_pos_a + aabb_a.max;

                    let will_collide_pure = 
                        pure_a_future_min.x <= b_max.x && pure_a_future_max.x >= b_min.x &&
                        pure_a_future_min.y <= b_max.y && pure_a_future_max.y >= b_min.y &&
                        pure_a_future_min.z <= b_max.z && pure_a_future_max.z >= b_min.z;

                    if will_collide_pure {
                        speed_a.speed_value = Vec3::ZERO;
                    }
                } 
            }
        }
    }
}

#[allow(dead_code)]
/// ## Description
/// Function for collision between 2 entities. This function adds wall sliding. If an entity encounters an obstacle on the X-axis, all of its X-axis velocity is transferred to the Z-axis. And vice versa.
/// 
/// Also the function includes logic for triggers. 
/// Trigger is an entity that others can move through. 
/// When an entity moved through the trigger then trigger changes its own bool flag in [`AcornIsCollided`]. 
/// You can use this function to implement objects like finish, bonus, button and etc. in your game.
/// 
/// ## Necessary set of Acorn functions for full functionality:
/// copy&paste this into `acorn_zsetup`:
/// ```
/// location! {
///     agt_xz_grid_create,
///     agt_xz_grid_do_slide_collision, // <-
///     agt_xz_grid_clear,
///     agt_do_entities_move, // This is necessary for the entities to move.
/// },
/// ```
///  
/// ## Necessary Global States in `AcornGlobalContext`:
/// * `pub game_base_preset: Acorn3DGameBase`
/// 
/// ## Required entity components for collisions:
/// * [`Acorn3DSpeed`]
/// * [`AcornEntity3DTransform`]
/// * [`AcornAABB`]
/// * [`AcornIsTrigger`]
/// * [`AcornIsCollided`]
pub fn agt_xz_grid_do_slide_collision_include_triggers(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    context: &mut AcornGlobalContext
) {
    let grid = &context.game_base_preset.world_collision_grid;
    
    let mut query = world.query::<(
        &AcornEntity3DTransform, 
        &mut Acorn3DSpeed, 
        &AcornAABB,
        &mut AcornIsCollided,
        &AcornIsTrigger
    )>();

    for (_coord, entities) in grid.cells.iter().filter(|(_, e)| e.len() >= 2) {
        for i in 0..entities.len() {
            for j in 0..entities.len() {
                if i == j { continue; } 

                let entity_a = entities[i];
                let entity_b = entities[j];

                if let Ok([components_a, components_b]) = query.get_many_mut(world, [entity_a, entity_b]) {
                    let (trans_a, mut speed_a, aabb_a, _, _) = components_a;
                    let (trans_b, _, aabb_b, mut collided_b, trigger_b) = components_b;

                    let b_min = trans_b.position + aabb_b.min;
                    let b_max = trans_b.position + aabb_b.max;

                    // --- ЖЕЛЕЗОБЕТОННАЯ ПРОВЕРКА КАСАНИЯ (МИКРОЛУЧИ / ЗАЗОР 0.1) ---
                    // Проверяем текущее положение игрока, но искусственно расширяем его AABB на 0.1 во все стороны.
                    // Это эквивалентно тому, что из игрока во все стороны торчат лучи длиной 0.1.
                    let ray_padding = 0.2;
                    let a_current_min = trans_a.position + aabb_a.min - Vec3::splat(ray_padding);
                    let a_current_max = trans_a.position + aabb_a.max + Vec3::splat(ray_padding);

                    // Также проверяем предиктивное положение со смещением на скорость
                    let future_pos_a = trans_a.position + speed_a.speed_value;
                    let a_future_min = future_pos_a + aabb_a.min - Vec3::splat(ray_padding);
                    let a_future_max = future_pos_a + aabb_a.max + Vec3::splat(ray_padding);

                    let touching_now = 
                        a_current_min.x <= b_max.x && a_current_max.x >= b_min.x &&
                        a_current_min.y <= b_max.y && a_current_max.y >= b_min.y &&
                        a_current_min.z <= b_max.z && a_current_max.z >= b_min.z;

                    let touching_future = 
                        a_future_min.x <= b_max.x && a_future_max.x >= b_min.x &&
                        a_future_min.y <= b_max.y && a_future_max.y >= b_min.y &&
                        a_future_min.z <= b_max.z && a_future_max.z >= b_min.z;

                    if touching_now || touching_future {
                        // Если микролуч дотянулся до сущности B — взводим флаг без всяких "но"
                        collided_b.is_collided = true;

                        // Если это триггер — мы просто взвели флаг касания и скипаем физику слайдинга!
                        // Он пролетит насквозь, даже если его скорость равна нулю (стоя на месте).
                        if trigger_b.is_trigger {
                            continue;
                        }
                    }

                    // --- СТАНДАРТНАЯ ФИЗИКА СЛАЙДИНГА СТЕН (Выполняется только для твердых стен) ---
                    if speed_a.speed_value == Vec3::ZERO { continue; }

                    // --- TEST X AXIS ---
                    if speed_a.speed_value.x != 0.0 {
                        let a_test_min = trans_a.position + Vec3::new(speed_a.speed_value.x, 0.0, 0.0) + aabb_a.min;
                        let a_test_max = trans_a.position + Vec3::new(speed_a.speed_value.x, 0.0, 0.0) + aabb_a.max;

                        let collide_x = 
                            a_test_min.x <= b_max.x && a_test_max.x >= b_min.x &&
                            (trans_a.position.y + aabb_a.min.y) <= b_max.y && (trans_a.position.y + aabb_a.max.y) >= b_min.y &&
                            (trans_a.position.z + aabb_a.min.z) <= b_max.z && (trans_a.position.z + aabb_a.max.z) >= b_min.z;

                        if collide_x {
                            // Add the speed from X to Z.
                            let push_dir_z = if speed_a.speed_value.z >= 0.0 { 1.0 } else { -1.0 };
                            speed_a.speed_value.z += speed_a.speed_value.x.abs() * push_dir_z;
                            
                            // X velocity is zero
                            speed_a.speed_value.x = 0.0; 
                        }
                    }

                    // --- TEST Z AXIS ---
                    if speed_a.speed_value.z != 0.0 {
                        let a_test_min = trans_a.position + Vec3::new(0.0, 0.0, speed_a.speed_value.z) + aabb_a.min;
                        let a_test_max = trans_a.position + Vec3::new(0.0, 0.0, speed_a.speed_value.z) + aabb_a.max;

                        let collide_z = 
                            (trans_a.position.x + aabb_a.min.x) <= b_max.x && (trans_a.position.x + aabb_a.max.x) >= b_min.x &&
                            (trans_a.position.y + aabb_a.min.y) <= b_max.y && (trans_a.position.y + aabb_a.max.y) >= b_min.y &&
                            a_test_min.z <= b_max.z && a_test_max.z >= b_min.z;

                        if collide_z {
                            // Add the speed from X to Z.
                            let push_dir_x = if speed_a.speed_value.x >= 0.0 { 1.0 } else { -1.0 };
                            speed_a.speed_value.x += speed_a.speed_value.z.abs() * push_dir_x;
                            
                            // Z velocity is zero
                            speed_a.speed_value.z = 0.0;
                        }
                    }

                    // --- TEST Y AXIS ---
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

// ====== fn about collision and entities ======

#[allow(dead_code)]
/// ## Description
/// Function to move entities. The function gets all speed (XYZ) from each entity and applies speed to position.
/// 
/// **You can copy this function and create own to filter by type entities (example, Goblin, Orc) through With<Component> for multithreading.** But you also need to rewrite this function into Bevy system and add to `acorn_esetup` for begin multithreading.
/// 
/// ## Necessary set of Acorn functions for full functionality:
/// copy&paste this into `acorn_zsetup`:
/// ```
/// location! {
///     agt_xz_grid_create,
///     agt_xz_grid_do_simple_collision, // use any type collision
///     agt_xz_grid_clear,
///     agt_do_entities_move, // <-
/// },
/// ```
///  
/// **But you also need create own function to update speed of entities. Example:**
/// ```
/// fn example_speed_your_entity(
///     world: &mut World, 
///     _zones: &mut AcornZoneContext, 
///     context: &mut AcornGlobalContext
/// ) {
///     let mut query = world.query_filtered::<&mut Acorn3DSpeed, With<YourTypeEntity>>();
/// 
///     // it's important thing. The speed of the camera and objects will not depend on FPS.
///     let dt = context.frame_delta; 
/// 
///     for mut i in query.iter_mut(world) {
///         if is_key_down(KeyCode::Right){
///             i.speed_value.x = 2.0 * dt;
///         } else {
///             i.speed_value.x = 0.0;
///         }
/// 
///         if is_key_down(KeyCode::Left){
///             i.speed_value.x = -2.0 * dt;
///         } else {
///             i.speed_value.x = 0.0;
///         }
///     }
/// }
/// ```
/// 
/// **And put your speed function BEFORE Location with collision functions (or in `ui_input_zone`):**
/// ```
/// location! {
///     example_speed_your_entity, // <- your function
///     agt_xz_grid_create,
///     agt_xz_grid_do_simple_collision, // use any type collision
///     agt_xz_grid_clear,
///     agt_do_entities_move, 
/// }, 
/// ```
/// 
/// ## Required entity components for collisions:
/// * `Acorn3DSpeed`
/// * `AcornEntity3DTransform`
/// * `AcornAABB`
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
/// ## Description
/// Function to draw AABB of entities.
/// 
/// ## Necessary set of Acorn functions for full functionality:
/// copy&paste this into `acorn_zsetup`:
/// ```
/// location! {
///     agt_xz_grid_create,
///     agt_debug_entities_aabb_draw, // <-
///     agt_xz_grid_do_simple_collision, // use any type collision
///     agt_xz_grid_clear,
///     agt_do_entities_move, // This is necessary for the entities to move.
/// },
/// ```
/// 
/// ## Required entity components for collisions:
/// * `Acorn3DSpeed`
/// * `AcornEntity3DTransform`
/// * `AcornAABB`
pub fn agt_debug_entities_aabb_draw(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    let mut query = world.query::<(&AcornEntity3DTransform, &AcornAABB)>();

    for (transform, aabb) in query.iter(world) {
        let world_min = transform.position + aabb.min;
        let world_max = transform.position + aabb.max;

        let position = (world_min + world_max) * 0.5;

        let size = world_max - world_min;

        let color = RED;

        draw_cube(position, size, None, color);
    }
}

#[allow(dead_code)]
/// ## Description 
/// Set for all entities false state for [`AcornIsCollided`].
/// 
/// This is a must-use if you have collision functions or Functions Sets that include triggers like [`agt_xz_grid_do_slide_collision_include_triggers`].
/// Otherwise, your entities always be collided even it's not so.
/// 
/// ## Required entity components for work:
/// * [`AcornIsCollided`]
pub fn agt_clear_collision_triggers(
    world: &mut World,
    _zones: &mut AcornZoneContext,
    _context: &mut AcornGlobalContext
) {
    let mut query = world.query::<&mut AcornIsCollided>();

    for mut collided in query.iter_mut(world) {
        collided.is_collided = false;
    }
}

// ---------------------------- Public Functions ----------------------------

#[allow(dead_code)]
/// ## Description 
/// Check intersects between 2 AABB.
pub fn agt_intersects_between_two_aabb(first: &AcornAABB, second: &AcornAABB) -> bool {
    first.min.x <= second.max.x && first.max.x >= second.min.x &&
    first.min.y <= second.max.y && first.max.y >= second.min.y &&
    first.min.z <= second.max.z && first.max.z >= second.min.z
}

#[allow(dead_code)]
/// ## Description 
/// Check collision between two object. Based on position and AABB.
pub fn agt_is_collide(
    first_aabb: &AcornAABB, 
    first_position: Vec3,
    second_aabb: &AcornAABB, 
    second_position: Vec3
) -> bool {
    let first_min = first_aabb.min + first_position;
    let first_max = first_aabb.max + first_position;

    let second_min = second_aabb.min + second_position;
    let second_max = second_aabb.max + second_position;

    first_min.x <= second_max.x && first_max.x >= second_min.x &&
    first_min.y <= second_max.y && first_max.y >= second_min.y &&
    first_min.z <= second_max.z && first_max.z >= second_min.z
}

#[allow(dead_code)]
/// ## Description 
/// Check predictive collision between two object. The first object should have [`Acorn3DSpeed`]. Based on Acorn Predicitve Collision: position, speed, AABB.
pub fn agt_is_predictive_collide(
    first_aabb: &AcornAABB, 
    first_position: Vec3,
    first_speed: &Acorn3DSpeed,
    second_aabb: &AcornAABB, 
    second_position: Vec3
) -> bool {
    // AABB of second entity
    let second_min = second_aabb.min + second_position;
    let second_max = second_aabb.max + second_position;

    // future position of first entity
    let first_future_pos = first_position + first_speed.speed_value;
    let first_future_min = first_future_pos + first_aabb.min;
    let first_future_max = first_future_pos + first_aabb.max;

    // collided?
    let will_collide = 
        first_future_min.x <= second_max.x && first_future_max.x >= second_min.x &&
        first_future_min.y <= second_max.y && first_future_max.y >= second_min.y &&
        first_future_min.z <= second_max.z && first_future_max.z >= second_min.z;

    will_collide
}